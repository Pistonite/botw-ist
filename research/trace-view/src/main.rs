use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use chrono::{Local, TimeZone};
use event::{EventLoop, FileModeEventLoop, Key, Msg, StopSignal};
use layout::LayoutDefinition;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph};
use ratatui::DefaultTerminal;
use trace::{Trace, TraceEvent, TraceNode, TraceThreadMap, TraceTree};

mod event;
mod layout;
mod trace;

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
    Message,
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

    /// Main trace data for each thread
    trace: TraceThreadMap<TraceTree>,
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
    trace_event_list_states: TraceThreadMap<TraceView>,
}

impl App<FileModeEventLoop> {
    /// Open a previous JSON trace dump file
    pub fn open_file(stop: &StopSignal, path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let trace = trace::load_trace_tree_file(path)?;
        let event_loop = FileModeEventLoop::new(stop);

        let path_text = path.display().to_string();
        let mut app = Self::create(stop, event_loop, trace, Mode::File, path_text);
        app.set_status(StatusLevel::Info, "file opened successfully");

        Ok(app)
    }
}

impl<E: EventLoop> App<E> {
    fn create(
        stop: &StopSignal,
        event_loop: E,
        trace: TraceThreadMap<TraceTree>,
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
        let mut data = self.trace.iter().map(|(k,v)| (k, v.len())).collect::<Vec<_>>();
        // sort by number of events
        data.sort_by_key(|x| std::cmp::Reverse(x.1));
        let thread_list = data.into_iter().map(|(k,v)| format!("{} ({})", k, v)).collect::<Vec<_>>();
        
        // if there are no threads, clear the selection
        if thread_list.is_empty() {
            self.thread_list_state.select(None);
        } else if let Some(selected) = &selected_id{
            // if the selected thread is not in the list, clear the selection
            if !self.trace.contains_key(selected) {
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
        self.thread_list_state
            .selected()
            .and_then(|x| match self.thread_list.get(x) {
                Some(x) => Some(x),
                None => self.thread_list.last(),
            })
            .and_then(|x| x.split_once(" ("))
            .map(|x| x.0)
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
                    match self.trace.get(id) {
                        None => {
                            self.focus = Focus::Threads;
                        }
                        Some(trace) => {
                            if trace.is_empty() {
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

    fn handle_event(&mut self, event: Msg) -> Result<()> {
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

    fn handle_key(&mut self, key: Key) {
        match self.focus {
            Focus::Threads => match key {
                Key::Quit => {
                    self.stop.stop();
                }
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
                    let previous = self
                        .thread_list_state
                        .selected()
                        .map_or(usize::MAX, |i| i.saturating_sub(page));
                    self.thread_list_state.select(Some(previous));
                }
                Key::PageDown => {
                    let page = self.layout.resolve(self.area).threads.height as usize;
                    let next = self
                        .thread_list_state
                        .selected()
                        .map_or(0, |i| i.saturating_add(page));
                    self.thread_list_state.select(Some(next));
                }
                Key::First => {
                    self.thread_list_state.select_first();
                }
                Key::Last => {
                    self.thread_list_state.select_last();
                }
                _ => {}
            },
            Focus::Events => {
                match self
                    .get_selected_thread_id()
                    .map(|x| x.to_string())
                    .and_then(|x| self.trace_event_list_states.get_mut(&x).map(|y| (x, y)))
                {
                    None => {
                        self.focus = Focus::Threads;
                    }
                    Some((id, view)) => {
                        let tree = self.trace.get_mut(&id);
                        match key {
                            Key::Left => {
                                self.focus = Focus::Threads;
                            }
                            Key::Enter => {
                                if let Some(tree) = tree {
                                    view.toggle_expanded(tree);
                                }
                            }
                            // expand and enter
                            Key::Right => {
                                if let Some(tree) = tree {
                                    view.set_expanded(tree, true);
                                    view.select_next(tree, 1);
                                }
                            }
                            Key::Quit => {
                                // goto parent
                                if let Some(tree) = tree {
                                    if !view.select_parent(tree) {
                                        self.focus = Focus::Threads;
                                    }
                                }
                            }
                            Key::Up => {
                                if let Some(tree) = tree {
                                    view.select_previous(tree, 1);
                                }
                            }
                            Key::Down => {
                                if let Some(tree) = tree {
                                    view.select_next(tree, 1);
                                }
                            }
                            Key::PageUp => {
                                if let Some(tree) = tree {
                                view.select_previous(tree, self.layout.resolve(self.area).events.height as usize);
                                }
                            }
                            Key::PageDown => {
                                if let Some(tree) = tree {
                                    view.select_next(tree, self.layout.resolve(self.area).events.height as usize);
                                }
                            }
                            Key::First => {
                                view.set_selected(0);
                            }
                            Key::Last => {
                                if let Some(tree) = tree {
                                    view.select_last(tree);
                                }
                            }
                            Key::View => {
                                self.focus = Focus::Message;
                            }
                        }
                    }
                }
            }
            Focus::Message => match key {
                Key::Quit => {
                    self.focus = Focus::Events;
                }
                _ => {}
            },
        }
    }
}

impl<E: EventLoop> Widget for &mut App<E> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = self.layout.resolve(area);

        // draw mode
        match self.mode {
            Mode::File => {
                Paragraph::new(Line::styled(
                    &self.mode_text,
                    Style::default().fg(Color::LightMagenta),
                ))
                .block(
                    Block::default()
                        .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                        .title("File"),
                )
                .render(layout.mode, buf);
            }
            Mode::Live => {
                Paragraph::new(Line::styled(
                    &self.mode_text,
                    Style::default().fg(Color::LightRed),
                ))
                .block(
                    Block::default()
                        .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                        .title("Live"),
                )
                .render(layout.mode, buf);
            }
        }

        let (status_title, status_style) = match self.status_level {
            StatusLevel::Info => ("Info", Style::default().fg(Color::White).italic()),
            StatusLevel::Warning => ("Warning", Style::default().fg(Color::LightYellow).italic()),
            StatusLevel::Error => ("Error", Style::default().fg(Color::LightRed).italic()),
        };

        // draw status
        Paragraph::new(Line::raw(&self.status))
            .style(status_style)
            .block(
                Block::default()
                    .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                    .title(status_title),
            )
            .render(layout.status, buf);

        // draw thread list
        let thread_list = List::new(self.thread_list.clone()).scroll_padding(4);
        let thread_list = if self.focus == Focus::Threads {
            thread_list.highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        } else {
            thread_list.highlight_style(Style::default().fg(Color::LightYellow))
        };
        let thread_list = thread_list.block(make_block(
            "Threads",
            self.thread_list_state.selected(),
            self.thread_list.len(),
            self.focus == Focus::Threads,
            true,
        ));

        StatefulWidget::render(
            thread_list,
            layout.threads,
            buf,
            &mut self.thread_list_state,
        );

        // draw event list
        let selected_thread_id = self.get_selected_thread_id();
        let trace_events = selected_thread_id.clone().and_then(|x| self.trace.get(x).map(|y| (x, y)));
        match trace_events {
            None => {
                Paragraph::new("No thread selected")
                    .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT).title("Events"))
                .render(layout.events, buf);
        
                Paragraph::new("")
                    .block(Block::default().borders(Borders::ALL).title("Message"))
                .render(layout.message, buf);
        
            }
            Some((id, tree)) => {
                let view = self.trace_event_list_states.entry(id.to_string()).or_default();
                let block = make_block("Events", Some(view.selected), tree.len(), self.focus == Focus::Events, false);
                let inner_area = block.inner(layout.events);
                
                view.update(tree, inner_area.width, inner_area.height, self.focus == Focus::Events);
                block.render(layout.events, buf);
                view.render(inner_area, buf);
                
                // draw message
                if let Some(event) = tree.get(view.selected) {
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

fn make_block(
    title: &str,
    current: Option<usize>,
    total: usize,
    focused: bool,
    bottom: bool,
) -> Block<'static> {
    let block = Block::default();
    let block = if bottom {
        block.borders(Borders::ALL)
    } else {
        block.borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
    };
    let block = match current {
        Some(x) => block.title(format!("{} ({}/{})", title, x.saturating_add(1), total)),
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
            Span::styled("<h><j><k><l>", Style::new().fg(Color::LightCyan)),
        ]),
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

    /// index of the first line in the view
    offset: usize,
    /// index of the selected line
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
            offset: 0,
            selected: 0,
            rendered: Default::default(),
        }
    }
}

impl TraceView {
    pub fn select_previous(&mut self, tree: &TraceTree, count: usize) {
        self.set_selected(self.find_previous(tree, count));
    }

    fn find_previous(&self, tree: &TraceTree, count: usize) -> usize {
        let mut new = Some(self.selected);
        for _ in 0..count {
            match new {
                Some(x) => {
                    new = tree.find_list_previous(x);
                }
                None => {
                    break;
                }
            }
        }
        new.unwrap_or_default()
    }

    pub fn select_next(&mut self, tree: &TraceTree, count: usize) {
        self.set_selected(self.find_next(tree, count));
    }

    fn find_next(&self, tree: &TraceTree, count: usize) -> usize {
        let mut new = Some(self.selected);
        for _ in 0..count {
            match new {
                Some(x) => {
                    new = tree.find_list_next(x);
                }
                None => {
                    break;
                }
            }
        }
        new.unwrap_or(tree.len().saturating_sub(1))
    }

    /// Toggle expanded state of the selected line
    pub fn toggle_expanded(&mut self, tree: &mut TraceTree) {
        tree.set_expanded(self.selected, !tree.is_expanded(self.selected));
        self.dirty = true;
    }

    pub fn set_expanded(&mut self, tree: &mut TraceTree, expanded: bool) {
        tree.set_expanded(self.selected, expanded);
        self.dirty = true;
    }

    /// Select the first event before the selected event that has a lower level
    /// or the first event in the list
    pub fn select_parent(&mut self, tree: &TraceTree) -> bool {
        match tree.find_parent(self.selected) {
            Some(x) => {
                self.set_selected(x);
                true
            }
            None => false,
        }
    }

    pub fn select_last(&mut self, tree: &TraceTree) {
        self.set_selected(tree.find_list_last().unwrap_or_default());
    }

    fn set_selected(&mut self, selected: usize) {
        if self.selected != selected {
            self.selected = selected;
            self.dirty = true;
        }
    }

    pub fn update(&mut self, tree: &TraceTree, width: u16, height: u16, focused: bool) {
        if tree.is_empty() {
            self.rendered.clear();
            return;
        }
        if !self.too_small
            && !self.dirty
            && self.last_width == width
            && self.last_height == height
            && self.last_focused == focused
        {
            return;
        }
        // compute the layout:
        // - context lines, up to 25% of height, min 1
        // - scroll padding, min 1, max 4, 10% of remaining height
        let mut context = {
            let context_max = (height as usize / 4).max(1);
            let context = tree.get_context(self.selected, context_max);
            if (height as usize) < context.len() {
                self.too_small = true;
                return;
            }
            context
        };

        let line_number_size = (self.selected + 1).to_string().len().max(3) + 1;
        let timestamp_size = 10; // |HH:MM:SS|
        let available = width as usize - line_number_size - timestamp_size - 2; // 2 for border
        let mut indices = Vec::with_capacity(height as usize);
        let mut context_size: usize;
        let mut main_height: usize;

        // render in a loop, if a line in the context becomes visible as
        // a result of rendering, adjust the context and try again
        'render: loop {
            context_size = if context.is_empty() {
                0
            } else {
                context.len() + 1
            };
            let scroll_padding = ((height as usize - context_size) / 10).min(4).max(1);
            // plus 1 for main content
            if (height as usize) < scroll_padding * 2 + context_size + 1 {
                self.too_small = true;
                return;
            }
            self.dirty = false;
            self.too_small = false;
            self.last_width = width;
            self.last_height = height;
            self.last_focused = focused;
            let height = height as usize;
            main_height = height - context_size;
            // update offset if scrolled out of view
            {
                let lower = self.find_previous(tree, scroll_padding);
                if self.offset > lower {
                    self.offset = lower;
                } else {
                    let upper = self.find_next(tree, scroll_padding);
                    if self.offset + main_height < upper {
                        self.offset = upper.saturating_sub(main_height);
                    }
                }
            }
            // compute indices and relative line number for the main content
            indices.clear();
            let mut i = Some(self.offset);
            for _ in 0..main_height {
                let idx = match i {
                    Some(x) => x,
                    None => break,
                };
                // adjust context and recalculate layout if needed
                if let Some(x) = context.iter().position(|&y| y == idx) {
                    context.truncate(x);
                    continue 'render;
                }
                indices.push(idx);
                i = tree.find_list_next(idx);
            }
            break;
        }
        // reset render
        self.rendered.clear();

        // render context
        if !context.is_empty() {
            let line_number_space = " ".repeat(line_number_size);
            for i in context {
                self.render_line(
                    available,
                    &line_number_space,
                    i,
                    tree.node(i).unwrap(),
                    focused,
                );
            }
            // context separator
            self.rendered.push(Line::styled(
                ">".repeat(width as usize),
                Style::new().fg(Color::LightBlue),
            ));
        }
        // compute relative line numbers for the main content
        let mut line_numbers = Vec::with_capacity(main_height);
        let mut line_number_after = 1;
        for (line_number_before, idx) in indices.iter().enumerate() {
            if *idx == self.selected {
                // line numbers before
                for j in (0..line_number_before).rev() {
                    line_numbers.push(format!("{:>width$}", j+1, width = line_number_size))
                }
                // line number of selected line
                line_numbers.push(format!(
                    "{:>width$}",
                    "-",
                    width = line_number_size
                ));
            } else if *idx > self.selected {
                // line numbers after
                line_numbers.push(format!("{:>width$}", line_number_after, width = line_number_size));
                line_number_after += 1;
            }
        }
        while line_numbers.len() < main_height {
            line_numbers.push(format!("{:>width$}", "~", width = line_number_size));
        }
        // render main content
        for (idx, line_number) in indices.into_iter().zip(line_numbers) {
            self.render_line(
                available,
                &line_number,
                idx,
                tree.node(idx).unwrap(),
                focused,
            );
        }
    }

    fn render_line(
        &mut self,
        available: usize,
        line_number: &str,
        i: usize,
        node: &TraceNode,
        focused: bool,
    ) {
        // timestamp
        let raw_timestamp: Option<i64> = node.timestamp.try_into().ok();
        let timestamp = raw_timestamp
            .and_then(|x| Local.timestamp_opt(x, 0).earliest())
            .map(|x| x.format("%H:%M:%S").to_string())
            .unwrap_or_else(|| "--:--:--".to_string());
        // indentation for nesting
        let nest_space: usize = (node.level * 2).try_into().unwrap_or_default();

        let expand_indicator = if node.expanded {
            "🞃 "
        } else if node.last_child.is_none() {
            // not a parent node
            "  "
        } else {
            "🞂 "
        };

        let (message, ellipsis) = truncate_message(&node.message, available - nest_space - 2);

        let line = Line::from(vec![
            Span::raw(format!("{}|", line_number)),
            Span::raw(timestamp),
            Span::raw("| "),
            Span::raw(" ".repeat(nest_space)),
            Span::raw(expand_indicator),
            Span::raw(message.to_string()),
            Span::raw(ellipsis),
        ]);
        let line = if i == self.selected {
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
            Paragraph::new("Terminal is too small").render(area, buf);
            return;
        }
        for (i, line) in self.rendered.iter().enumerate() {
            line.render(Rect::new(area.x, area.y + i as u16, area.width, 1), buf);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse trace event")]
    TraceParseError,
}
