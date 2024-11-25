use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::thread;

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use derive_more::derive::{Deref, DerefMut};
use event::{EventLoop, FileModeEventLoop, Key, Msg, StopSignal};
use layout::LayoutDefinition;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph};
// use ratatui::layout::{Constraint, Direction, Layout, Rect};
// use ratatui::widgets::Widget;
use ratatui::DefaultTerminal;
use serde::{Deserialize, Serialize};

mod event;
mod layout;

fn main() {
        let mut screen = ratatui::init();
    let e = main_internal(&mut screen);
        // ratatui sets up panic hooks internally,
        // so restore will be called if app panics
        ratatui::restore();
    if let Err(e) = e {
        eprintln!("fatal: {:?}", e);
    }
}

fn main_internal(screen: &mut DefaultTerminal) -> Result<()> {
    let stop = StopSignal::default();
    let app = App::open_file(&stop, "test.json")?;
    app.main_loop(screen)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StatusLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    File,
    Live,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Focus {
    Threads,
    Events,
    Message
}

struct App<E: EventLoop> {
    /// Main app loop escape switch
    stop: StopSignal,
    /// Layout for the app
    layout: LayoutDefinition,
    /// EventLoop for the app
    event_loop: E,
    /// Focus state
    focus: Focus,

    /// Current drawing area
    area: Rect,

    /// Main trace data
    trace: Trace,
    /// Writers for writing trace events to temporary files as they come in
    trace_writers: BTreeMap<String, BufWriter<File>>,

    /// Mode title
    mode: Mode,
    /// Mode text
    mode_text: String,

    /// Status message
    status: String,
    /// Status message level
    status_level: StatusLevel,

    /// Threads sorted by number of events
    ///
    /// Each element is the text to display (<ID> (<count>))
    thread_list: Vec<String>,
    /// State of the threads list
    thread_list_state: ListState,
    /// States of the events list
    trace_event_list_states: BTreeMap<String, TraceView>,
}

impl App<FileModeEventLoop> {
    /// Open a previous JSON trace dump file
    pub fn open_file(stop: &StopSignal, path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let trace: Trace = serde_json::from_reader(buf_reader)?;
        let event_loop = FileModeEventLoop::new(stop);

        let path_text = path.display().to_string();
        let mut app = Self::create(
            stop, event_loop,trace,
            Mode::File, path_text);
        app.set_status(StatusLevel::Info, "file opened successfully");

        Ok(app)
    }
}

impl<E: EventLoop> App<E> {

    fn create(
        stop: &StopSignal,
        event_loop: E,
        trace: Trace, 
        mode: Mode, 
        mode_text: String,
    ) -> Self {

        let mut app = Self {
            stop: stop.clone(),
            layout: Default::default(),
            event_loop,
            focus: Focus::Threads,
            area: Rect::default(),
            trace,
            trace_writers: Default::default(),
            mode,
            mode_text,
            status: String::new(),
            status_level: StatusLevel::Info,
            thread_list: vec![],
            thread_list_state: ListState::default(),
            trace_event_list_states: Default::default(),
        };

        app.update_thread_list();
        app
    }

    pub fn set_status(&mut self, level: StatusLevel, status: impl Into<String>) {
        self.status = status.into();
        self.status_level = level;
    }

    /// Update the thread list and the selection state
    pub fn update_thread_list(&mut self) {
        // get the selected thread id
        let selected_id = self.get_selected_thread_id().map(|x| x.to_string());
        let mut data = self.trace.data.iter().map(|(k,v)| (k, v.events.len())).collect::<Vec<_>>();
        // sort by number of events
        data.sort_by_key(|x| std::cmp::Reverse(x.1));
        let thread_list = data.into_iter().map(|(k,v)| format!("{} ({})", k, v)).collect::<Vec<_>>();

        // if there are no threads, clear the selection
        if thread_list.is_empty() {
            self.thread_list_state.select(None);
        } else if let Some(selected) = &selected_id{
            // if the selected thread is not in the list, clear the selection
            if !self.trace.data.contains_key(selected) {
            self.thread_list_state.select(None);
            }
        }

        // if there is no selection, select the first thread if possible
        if self.thread_list_state.selected().is_none() && !thread_list.is_empty() {
                self.thread_list_state.select(Some(0));
        } else {
            // update the selection index based on the new list
            if let Some(selected) = selected_id {
                if let Some(index) = thread_list.iter().position(|x| x.starts_with(&selected)) {
                    self.thread_list_state.select(Some(index));
                }
            }
        }

        self.thread_list = thread_list;
    }

    fn get_selected_thread_id(&self) -> Option<&str> {
        self.thread_list_state.selected().and_then(|x| {
            match self.thread_list.get(x) {
                Some(x) => Some(x),
                None=> self.thread_list.last()
            }
        })
        .and_then(|x| x.split_once(" (")).map(|x| x.0)
    }

    /// Main application event loop
    pub fn main_loop(mut self, screen: &mut DefaultTerminal) -> Result<()> {
        while !self.stop.is_stopped() {
            // update focus
            match self.get_selected_thread_id() {
                None => {
                    self.focus = Focus::Threads;
                }
                Some(id) => {
                    match self.trace.data.get(id) {
                        None => {
                            self.focus = Focus::Threads;
                        }
                        Some(trace) => {
                            if trace.events.is_empty() {
                                self.focus = Focus::Threads;
                            }
                        }
                    }
                }
            }
            // draw screen
            screen.draw(|f| self.draw(f))?;
            // handle events
            if let Some(event) = self.event_loop.recv() {
                self.handle_event(event)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) {
        let area = f.area();
        self.area = area;
        f.render_widget(self, area);
    }

    fn handle_event(&mut self, event: Msg ) -> Result<()> {
        match event {
            Msg::Terminate => {
                self.stop.stop();
            }
            Msg::Key(key) => {
                self.handle_key(key);
            }
            Msg::Rerender => {}
        }

        self.event_loop.resume();

        Ok(())
    }

    fn handle_key(&mut self, key: Key ) {
        match self.focus {
            Focus::Threads => {
                match key {
                    Key::Quit => {
                        self.stop.stop();
                    },
                    Key::Enter | Key::Right => {
                        self.focus = Focus::Events;
                    }
                    Key::Up => {
                        self.thread_list_state.select_previous();
                    }
                    Key::Down => {
                        self.thread_list_state.select_next();
                    }
                    Key::PageUp => {
                        let page = self.layout.resolve(self.area).threads.height as usize;
                        let previous = self.thread_list_state.selected().map_or(usize::MAX, |i| i.saturating_sub(page));
                        self.thread_list_state.select(Some(previous));
                    }
                    Key::PageDown => {
                        let page = self.layout.resolve(self.area).threads.height as usize;
                        let next = self.thread_list_state.selected().map_or(0, |i| i.saturating_add(page));
                        self.thread_list_state.select(Some(next));
                    }
                    Key::First => {
                        self.thread_list_state.select_first();
                    }
                    Key::Last => {
                        self.thread_list_state.select_last();
                    }
                    _ => {}
                }
            }
            Focus::Events => {
                match self.get_selected_thread_id().map(|x|x.to_string()).and_then(|x| self.trace_event_list_states.get_mut(&x).map(|y| (x, y))) {
                    None => {
                        self.focus = Focus::Threads;
                    }
                    Some((id, view)) => {
                        let events = self.trace.data.get(&id).map(|x| &x.events);
                        match key {
                            Key::Left => {
                                self.focus = Focus::Threads;
                            }
                            Key::Enter => {
                                view.toggle_expanded(events.map(|x|x.len()).unwrap_or_default());
                            }
                            Key::Right => {
                                if let Some(events) = events {
                                    view.select_parent(events);
                                    view.set_expanded(false);
                                }
                            }
                            Key::Quit => {
                                if let Some(events) = events {
                                    view.set_expanded(true);
                                    view.select_next(1, events.len());
                                }
                            }
                            Key::Up => {
                                view.select_previous(1);
                            }
                            Key::Down => {
                                if let Some(events) = events {
                                    view.select_next(1, events.len());
                                }
                            }
                            Key::PageUp => {
                                view.select_previous(self.layout.resolve(self.area).events.height as usize);
                            }
                            Key::PageDown => {
                                if let Some(events) = events {
                                    view.select_next(self.layout.resolve(self.area).events.height as usize, events.len());
                                }
                            }
                            Key::First => {
                                view.select_previous(usize::MAX);
                            }
                            Key::Last => {
                                if let Some(events) = events {
                                    view.select_next(usize::MAX, events.len());
                                }
                            }
                            Key::View => {
                                self.focus = Focus::Message;
                            }
                        }
                    }
                }
            }
            Focus::Message => {
                match key {
                    Key::Quit => {
                        self.focus = Focus::Events;
                    }
                    _ => {}
                }
            }
        }
    }

}

impl<E: EventLoop> Widget for &mut App<E> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = self.layout.resolve(area);

        // draw mode
        match self.mode {
            Mode::File => {
                Paragraph::new(
                        Line::styled(&self.mode_text, Style::default().fg(Color::LightMagenta)),
                ).block(Block::default().borders(Borders::TOP | Borders::RIGHT | Borders::LEFT).title("File"))
                .render(layout.mode, buf);
            }
            Mode::Live => {
                Paragraph::new( Line::styled(&self.mode_text, Style::default().fg(Color::LightRed)))
                    .block(Block::default().borders(Borders::TOP | Borders::RIGHT | Borders::LEFT).title("Live"))
                .render(layout.mode, buf);
            }
        }

        let (status_title, status_style) = match self.status_level {
            StatusLevel::Info => ("Info", Style::default().fg(Color::White).italic()),
            StatusLevel::Warning => ("Warning", Style::default().fg(Color::LightYellow).italic()),
            StatusLevel::Error => ("Error", Style::default().fg(Color::LightRed).italic()),
        };

        // draw status
        Paragraph::new(
            Line::raw( &self.status)
        )
            .style(status_style)
            .block(Block::default().borders(Borders::TOP | Borders::RIGHT | Borders::LEFT).title(status_title))
        .render(layout.status, buf);

        // draw thread list
        let thread_list = List::new(self.thread_list.clone())
            .scroll_padding(4);
        let thread_list = if self.focus == Focus::Threads {
            thread_list.highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        } else {
            thread_list.highlight_style(Style::default().fg(Color::LightYellow))
            };
        let thread_list = thread_list
        .block(
                make_block("Threads", self.thread_list_state.selected(), self.thread_list.len(), self.focus == Focus::Threads, true)
            );

        StatefulWidget::render(thread_list, layout.threads, buf, &mut self.thread_list_state);

        // draw event list
        let selected_thread_id = self.get_selected_thread_id();
        let trace_events = selected_thread_id.clone().and_then(|x| self.trace.data.get(x).map(|y| (x, y)));
        match trace_events {
            None => {
                Paragraph::new("No thread selected")
                    .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT).title("Events"))
                .render(layout.events, buf);

                Paragraph::new("")
                    .block(Block::default().borders(Borders::ALL).title("Message"))
                .render(layout.message, buf);

            }
            Some((id, trace)) => {
                let view = self.trace_event_list_states.entry(id.to_string()).or_default();
                let block = make_block("Events", view.selected(trace.events.len()), trace.events.len(), self.focus == Focus::Events, false);
                let inner_area = block.inner(layout.events);

                view.update_rendered(&trace.events, inner_area.width, inner_area.height, self.focus == Focus::Events);
                block.render(layout.events, buf);
                view.render(inner_area, buf);

                if let Some(event) = view.selected(trace.events.len()).and_then(|x| trace.events.get(x)) {
                        let block = Block::default().borders(Borders::ALL).title("Message");
                        let block = if self.focus == Focus::Message {
                            block.border_style(Style::default().fg(Color::LightGreen))
                        } else {
                            block
                        };
                        Paragraph::new(event.message.clone()).block(block)
                        .render(layout.message, buf);
                } else {
                Paragraph::new("")
                    .block(Block::default().borders(Borders::ALL).title("Message"))
                .render(layout.message, buf);
                }


            }
        }

        // draw help
        help_text().render(layout.help, buf);

        

    }
}

fn make_block(title: &str, current: Option<usize>, total: usize, focused: bool, bottom: bool) -> Block<'static> {
    let block = Block::default();
        let block = if bottom {
            block.borders(Borders::ALL)
        } else {
            block.borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
        };
    let block = match current {
        Some(x) => block.title(format!("{} ({}/{})", title, x + 1, total)),
        None => block.title(format!("{} ({})", title, total)),
    };
    if focused {
        block.border_style(Style::default().fg(Color::LightGreen))
    } else {
        block
    }
}



fn help_text() -> Paragraph<'static> {
        Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::raw("Move   : "),
                Span::styled("<h><j><k><l>", Style::new().fg(Color::LightCyan)),]),

            Line::from(vec![
                Span::raw("       | "),
                Span::styled("<←><↓><↑><→>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(vec![
                Span::raw("Page   : "),
                Span::styled("<u><d>", Style::new().fg(Color::LightCyan)),
                Span::raw(" | "),
                Span::styled("<[><]>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(vec![
                Span::raw("       | "),
                Span::styled("<PAGEUP><PAGEDOWN>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(vec![
                Span::raw("First  : "),
                Span::styled("<g>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(vec![
                Span::raw("Last   : "),
                Span::styled("<G>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Select : "),
                Span::styled("<ENTER>", Style::new().fg(Color::LightCyan)),
                Span::raw(" | "),
                Span::styled("<SPACE>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(vec![
                Span::raw("       | "),
                Span::styled("<o>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(vec![
                Span::raw("Quit   : "),
                Span::styled("<ESC>", Style::new().fg(Color::LightCyan)),
                Span::raw(" | "),
                Span::styled("<q>", Style::new().fg(Color::LightCyan)),
                Span::raw(" | "),
                Span::styled("<C-c>", Style::new().fg(Color::LightCyan)),
            ]),
            Line::from(vec![
                Span::raw("View   : "),
                Span::styled("<v>", Style::new().fg(Color::LightCyan)),
            ]),
        ]))
}

#[derive(Debug, Clone, PartialEq)]
struct TraceView {
    too_small: bool,
    dirty: bool,
    last_width: u16,
    last_height: u16,
    last_focused: bool,
    padding: usize,
    offset: usize,
    expanded: BTreeSet<usize>,
    selected: usize,
    rendered: Vec<Line<'static>>,
}

impl Default for TraceView {
    fn default() -> Self {
        Self {
            too_small: false,
            dirty: true,
            last_width: 0,
            last_height: 0,
            last_focused: false,
            padding: 4,
            offset: 0,
            expanded: Default::default(),
            selected: 0,
            rendered: Default::default(),
        }
    }
}

impl TraceView {
    pub fn select_previous(&mut self, count: usize) {
        let new = if self.selected >= count {
            self.selected - count
        } else {
            0
        };
        if self.selected != new {
            self.selected = new;
            if new < self.offset + self.padding {
                self.offset = if new < self.padding {
                    0
                } else {
                    new - self.padding
                };
            }
            self.dirty = true;
        }
    }

    pub fn select_next(&mut self, count: usize, max: usize) {
        let new = if self.selected.saturating_add(count) < max {
            self.selected + count
        } else {
            max - 1
        };
        if self.selected != new {
            self.selected = new;
            self.dirty = true;
        }
    }

    /// Toggle expanded state of the selected line
    pub fn toggle_expanded(&mut self, max: usize) {
        if self.selected >= max {
            self.expanded.remove(&self.selected);
            self.selected = max - 1;
        }
        if !self.expanded.remove(&self.selected) {
            self.expanded.insert(self.selected);
        }
        self.dirty = true;
    }

    pub fn set_expanded(&mut self, expanded: bool) {
        if expanded {
            if self.expanded.insert(self.selected) {
                self.dirty = true;
            }
        } else {
            if self.expanded.remove(&self.selected) {
                self.dirty = true;
            }
        }
    }

    /// Check if the selected event is expanded
    pub fn is_expanded(&self) -> bool {
        self.expanded.contains(&self.selected)
    }

    pub fn selected(&self, max: usize) -> Option<usize> {
        if self.selected < max {
            Some(self.selected)
        } else {
            None
        }
    }

    /// Select the first event before the selected event that has a lower level
    /// or the first event in the list
    pub fn select_parent(&mut self, list: &[TraceEvent]) {
        let mut current = self.selected;
        if current >= list.len() {
            current = list.len() - 1;
        }
        let level = list[current].level;
        while current > 0 {
            current -= 1;
            if list[current].level < level {
                break;
            }
        }
        if current != self.selected {
            self.selected = current;
            self.dirty = true;
        }
    }

    pub fn update_rendered(&mut self, list: &[TraceEvent], width: u16, height: u16, focused: bool) {
        if list.is_empty() {
            self.rendered.clear();
            return;
        }
        if !self.too_small && !self.dirty && self.last_width == width && self.last_height == height && self.last_focused == focused {
            return;
        }
        if (height as usize) < self.padding * 2 + 1 {
            self.too_small = true;
            return;
        }
        self.dirty = false;
        self.too_small = false;
        self.last_width = width;
        self.last_height = height;
        self.last_focused = focused;
        // update offset if scrolled out of view below
        let height = height as usize;
        if self.selected + self.padding > self.offset + height {
            // rhs will not overflow because the line above
            self.offset = self.selected + self.padding - height;
        }
        if self.offset + height > list.len() {
            self.offset = list.len().saturating_sub(height);
        }
        // count the entries based on expanded state
        let mut entry_count = 0;
        for i in self.offset.. {
            let entry = match list.get(i) {
                Some(x) => x,
                None => break,
            };
        }

        let selected = self.selected;

        let line_number_size = (selected+1).to_string().len().max(3) + 1;
        let timestamp_size = 10; // |HH:MM:SS|
        let available = width as usize - line_number_size - timestamp_size - 2; // 2 for border
        
        self.rendered.clear();
        
        for i in self.offset..list.len().min(self.offset + height) {
            let entry = &list[i];
            // line number
            let line_number = match i.cmp(&selected) {
                std::cmp::Ordering::Less=> format!("{:>width$}", selected - i, width=line_number_size),
                std::cmp::Ordering::Equal => format!(" {:<width$}", selected + 1, width=line_number_size-1),
                std::cmp::Ordering::Greater => format!("{:>width$}", i - selected, width=line_number_size),
            };
            // timestamp
            let raw_timestamp: Option<i64> = entry.timestamp.try_into().ok();
            let timestamp = raw_timestamp
                .and_then(|x| Local.timestamp_opt(x, 0).earliest())
                .map(|x| x.format("%H:%M:%S").to_string())
                .unwrap_or_else(|| "--:--:--".to_string());

            // indentation for nesting
            let nest_space: usize = (entry.level * 2).try_into().unwrap_or_default();

            // expand indicator
            let expandable = i < list.len() - 1 && list[i + 1].level > entry.level;
            let expand_indicator = if !expandable {
                Span::raw("  ")
            } else if self.expanded.contains(&i) {
                Span::raw("- ")
            } else if i == selected{
                Span::raw("+ ")
            } else {
                Span::styled("+ ", Style::default().fg(Color::LightRed))
            };

            let (message, ellipsis) = truncate_message(&entry.message, available-nest_space-2);

            let line = Line::from(vec![
                Span::raw(format!("{}|", line_number)),
                if i == selected {
                        Span::raw(timestamp)
                } else {
                        Span::styled(timestamp, Style::default().fg(Color::Magenta))
                },
                Span::raw("| "),
                Span::raw(" ".repeat(nest_space)),
                expand_indicator,
                Span::raw(message.to_string()),
                Span::raw(ellipsis),
            ]);
            let line = if i == selected {
                if focused {
                    line.style(Style::new().fg(Color::Black).bg(Color::White))
                } else {
                    line.style(Style::new().fg(Color::LightYellow))
                }
            } else {
                line
            };
            self.rendered.push(line);
        }
    }
}

fn truncate_message(message: &str, available: usize) -> (&str, &'static str) {
    if message.len() > available {
        (&message[..available - 3], "...")
    } else {
        (message, "")
    }
}

impl Widget for &TraceView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.too_small {
            Paragraph::new("Terminal is too small")
                .render(area, buf);
            return;
        }
        for (i, line) in self.rendered.iter().enumerate() {
            line.render(Rect::new(area.x, area.y + i as u16, area.width, 1), buf);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TraceEvent {
    timestamp: u64,
    level: u64,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Deref, DerefMut)]
struct TraceEventPayload {
    thread_id: String,
    #[deref]
    #[deref_mut]
    event: TraceEvent,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct ThreadTrace {
    events: Vec<TraceEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Trace {
    #[serde(flatten)]
    data: BTreeMap<String, ThreadTrace>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to parse trace event")]
    TraceParseError,
}

/// Parse the raw buffer into the trace object
///
/// The buffer should have the following format:
/// ```text
/// <time> <thread> <level> <message>
/// ```
/// `time`, `thread` and `level` are hex strings with no `0x` prefix
/// and no space in between
fn parse_trace_event(raw_input: &[u8]) -> Result<TraceEventPayload> {
    let mut iter = raw_input.split(|x| *x == b' ');
    let timestamp = iter.next().ok_or(Error::TraceParseError)?;
    let timestamp = parse_hex(timestamp)?;
    let thread_id = iter.next().ok_or(Error::TraceParseError)?;
    let thread_id = format!("0x{}", std::str::from_utf8(thread_id)?);
    let level = iter.next().ok_or(Error::TraceParseError)?;
    let level = parse_hex(level)?;

    let message = iter.map(|x| std::str::from_utf8(x))
        .collect::<std::result::Result<Vec<_>, _>>()?.join(" ");

    let event = TraceEvent {
        timestamp,
        level,
        message,
    };

    Ok(TraceEventPayload {
        thread_id,
        event,
    })
}

fn parse_hex(buf: &[u8]) -> Result<u64> {
    let s = std::str::from_utf8(buf)?;
    Ok(u64::from_str_radix(s, 16)?)
}
