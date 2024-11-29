use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::DefaultTerminal;

mod event;
mod layout;
mod status;
mod thread_view;
mod trace;

use event::{EventLoop, FileModeEventLoop, Input, Key, LiveModeEventLoop, Msg, StopSignal};
use layout::LayoutDefinition;
use status::{StatusLevel, StatusLine};
use thread_view::ThreadView;
use trace::{TraceThreadMap, TraceTree, TraceView};

static LOGO: &str = r#" ______ __  __ __  __ ______ ______ ______ __  __  
/\  ___\\ \/ //\ \_\ \\  == \\  __ \\  __ \\ \/ /  
\ \___  \\  _"-.\____ \\  __< \ \/\ \\ \/\ \\  _"-.
 \/\_____\\_\ \_\\_____\\_____\\_____\\_____\\_\ \_\
  \/_____//_/\/_//_____//_____//_____//_____//_/\/_/"#;

/// Trace view tool CLI
#[derive(Debug, Clone, PartialEq, Parser)]
#[clap(bin_name="skybook-trace-view", before_help=LOGO)]
pub struct Cli {
    /// File to open or save to
    pub file: PathBuf,

    /// Connect to server to receive live trace events
    #[clap(short = 'c', long = "connect")]
    pub address: Option<String>,

    /// Overwrite existing output file in live mode
    #[clap(short, long)]
    pub force: bool,
}

fn main() {
    let cli = Cli::parse();
    #[allow(clippy::collapsible_if)] // readability
    if cli.address.is_some() && !cli.force {
        if cli.file.exists() {
            eprintln!("output file already exists, use --force to overwrite");
            return;
        }
    }
    let mut screen = ratatui::init();
    let mut out = Vec::new();
    let e = main_internal(&mut out, &mut screen, &cli);
    // ratatui sets up panic hooks internally,
    // so restore will be called if app panics
    ratatui::restore();
    if !out.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&out));
    }
    if let Err(e) = e {
        eprintln!("fatal: {:?}", e);
    }
}

fn main_internal(out: &mut dyn Write, screen: &mut DefaultTerminal, cli: &Cli) -> Result<()> {
    let stop = StopSignal::default();
    match &cli.address {
        Some(address) => {
            let mut app = App::open_connection(&stop, address)?;
            app.main_loop(screen)?;
            if let Err(e) = app.save_trace(&cli.file) {
                let _ = writeln!(out, "\u{1b}[1Kfailed to save trace: {:?}", e);
            } else {
                let _ = writeln!(out, "\u{1b}[1Ktrace saved to: {}", cli.file.display());
            }
        }
        None => {
            let mut app = App::open_file(&stop, &cli.file)?;
            app.main_loop(screen)?;
        }
    };
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Focus {
    Threads,
    Events,
    Message,
    SearchInput,
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
    /// Status message
    status: StatusLine,
    /// Current drawing area
    area: Rect,

    /// Main trace data for each thread
    trace: TraceThreadMap<TraceTree>,

    /// The thread list view
    thread_view: ThreadView,
    /// The events list views
    trace_views: TraceThreadMap<TraceView>,
    /// Event search string
    search_input: String,
    previous_focus: Focus,
}

impl App<FileModeEventLoop> {
    /// Open a previous JSON trace dump file
    pub fn open_file(stop: &StopSignal, path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let trace = trace::load_trace_file(path)?;
        let event_loop = FileModeEventLoop::new(stop);

        let status = StatusLine::file(path);
        let app = Self::create(stop, event_loop, trace, status);

        Ok(app)
    }
}

impl App<LiveModeEventLoop> {
    pub fn open_connection(stop: &StopSignal, address: &str) -> Result<Self> {
        let trace = Default::default();
        let event_loop = LiveModeEventLoop::new(stop, address);

        let status = StatusLine::live(address);
        let app = Self::create(stop, event_loop, trace, status);

        Ok(app)
    }
}

impl<E: EventLoop> App<E> {
    fn create(
        stop: &StopSignal,
        event_loop: E,
        trace: TraceThreadMap<TraceTree>,
        status: StatusLine,
    ) -> Self {
        let mut app = Self {
            stop: stop.clone(),
            layout: Default::default(),
            event_loop,
            focus: Focus::Threads,
            area: Rect::default(),
            trace,
            status,
            thread_view: Default::default(),
            trace_views: Default::default(),
            search_input: Default::default(),
            previous_focus: Focus::Threads,
        };

        app.thread_view.update(&app.trace);
        app
    }

    /// Main application event loop
    pub fn main_loop(&mut self, screen: &mut DefaultTerminal) -> Result<()> {
        while !self.stop.is_stopped() {
            // update focus
            if self.focus != Focus::SearchInput {
                match self.thread_view.get_selected() {
                    None => {
                        self.focus = Focus::Threads;
                    }
                    Some(id) => match self.trace.get(id) {
                        None => {
                            self.focus = Focus::Threads;
                        }
                        Some(trace) => {
                            if trace.is_empty() {
                                self.focus = Focus::Threads;
                            }
                        }
                    },
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

    pub fn save_trace(&self, path: impl AsRef<Path>) -> Result<()> {
        let writer = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(writer, &self.trace)?;
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
            Msg::Focus(x, y) => {
                if self.focus != Focus::SearchInput {
                    let layout = self.layout.resolve(self.area);
                    let position = Position::new(x, y);
                    if layout.threads.contains(position) {
                        self.focus = Focus::Threads;
                    } else if layout.events.contains(position) {
                        self.focus = Focus::Events;
                    } else if layout.message.contains(position) {
                        self.focus = Focus::Message;
                    }
                }
            }
            Msg::ActionCount(count) => {
                if self.focus != Focus::SearchInput {
                    self.status.append_action_count(count);
                }
            }
            Msg::Key(key) => {
                let count = self.status.take_action_count();
                if count == 1 || key != Key::Quit {
                    for _ in 0..count {
                        self.handle_key(key);
                    }
                }
            }
            Msg::Input(input) => {
                if self.focus == Focus::SearchInput {
                    match input {
                        Input::Char(x) => {
                            self.search_input.push(x);
                        }
                        Input::Backspace => {
                            self.search_input.pop();
                        }
                        Input::Done => {
                            self.focus = self.previous_focus;
                        }
                    }
                } else if input == Input::Char('/') {
                    self.previous_focus = self.focus;
                    self.focus = Focus::SearchInput;
                }
            }
            Msg::Status(level, msg) => {
                self.status.set(level, msg);
            }
            Msg::Trace(payload) => {
                if let Err(e) = self
                    .trace
                    .entry(payload.thread_id)
                    .or_default()
                    .add_event(payload.event)
                {
                    self.status
                        .set(StatusLevel::Error, format!("failed to add event: {}", e));
                }
                self.thread_view.update(&self.trace);
            }
            Msg::Rerender => {}
        }

        Ok(())
    }

    fn handle_key(&mut self, key: Key) {
        match self.focus {
            Focus::Threads => self.handle_key_for_threads(key),
            Focus::Events => {
                let page = self.layout.resolve(self.area).events.height as usize;
                match self
                    .thread_view
                    .get_selected()
                    .map(|x| (self.trace_views.get_mut(x), self.trace.get_mut(x)))
                {
                    Some((Some(view), Some(tree))) => {
                        self.focus = Self::handle_key_for_traces(key, view, tree, page);
                    }
                    _ => {
                        self.focus = Focus::Threads;
                    }
                }
            }
            Focus::Message => {
                let page = self.layout.resolve(self.area).message.height;
                match self
                    .thread_view
                    .get_selected()
                    .map(|x| (self.trace_views.get(x), self.trace.get_mut(x)))
                {
                    Some((Some(view), Some(tree))) => {
                        if let Some(selected) = view.get_selected(tree) {
                            self.focus = Self::handle_key_for_message(key, selected, tree, page);
                        }
                    }
                    _ => {
                        self.focus = Focus::Threads;
                    }
                }
            }
            Focus::SearchInput => {}
        }
    }

    fn handle_key_for_threads(&mut self, key: Key) {
        match key {
            Key::Quit => {
                self.stop.stop();
            }
            Key::Enter | Key::Right | Key::View => {
                self.focus = Focus::Events;
            }
            Key::Up => {
                self.thread_view.select_prev(1);
            }
            Key::Down => {
                self.thread_view.select_next(1);
            }
            Key::PageUp => {
                let page_size = self.layout.resolve(self.area).threads.height as usize;
                self.thread_view.select_prev(page_size);
            }
            Key::PageDown => {
                let page_size = self.layout.resolve(self.area).threads.height as usize;
                self.thread_view.select_next(page_size);
            }
            Key::First => {
                self.thread_view.select_first();
            }
            Key::Last => {
                self.thread_view.select_last();
            }
            _ => {}
        }
    }

    fn handle_key_for_traces(
        key: Key,
        view: &mut TraceView,
        tree: &mut TraceTree,
        page: usize,
    ) -> Focus {
        match key {
            Key::Left => {
                return Focus::Threads;
            }
            Key::Enter => {
                view.toggle_expanded(tree);
            }
            // expand and enter
            Key::Right => {
                view.set_expanded(tree, true);
                view.select_next(tree, 1);
            }
            Key::Quit => {
                // goto parent
                if !view.select_parent(tree) {
                    return Focus::Threads;
                }
            }
            Key::Up => {
                view.select_previous(tree, 1);
            }
            Key::Down => {
                view.select_next(tree, 1);
            }
            Key::PageUp => {
                view.select_previous(tree, page);
            }
            Key::PageDown => {
                view.select_next(tree, page);
            }
            Key::First => {
                view.select_first(tree);
            }
            Key::Last => {
                view.select_last(tree);
            }
            Key::View => {
                return Focus::Message;
            }
            Key::SearchNext => {
                view.select_search_next(tree);
            }
            Key::SearchPrev => {
                view.select_search_prev(tree);
            }
        };

        Focus::Events
    }

    fn handle_key_for_message(key: Key, selected: usize, tree: &mut TraceTree, page: u16) -> Focus {
        match key {
            Key::Quit => {
                return Focus::Events;
            }
            Key::Down => {
                tree.scroll_message_down(selected, page, 1);
            }
            Key::Up => {
                tree.scroll_message_up(selected, 1);
            }
            Key::PageUp => {
                tree.scroll_message_up(selected, page);
            }
            Key::PageDown => {
                tree.scroll_message_down(selected, page, page);
            }
            Key::First => {
                tree.scroll_message_up(selected, u16::MAX);
            }
            Key::Last => {
                tree.scroll_message_down(selected, page, u16::MAX);
            }
            _ => {}
        };
        Focus::Message
    }
}

impl<E: EventLoop> Widget for &mut App<E> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = self.layout.resolve(area);

        // draw status
        self.status.render(layout.status, buf);

        // draw thread list
        self.thread_view.set_focused(self.focus == Focus::Threads);
        self.thread_view.render(layout.threads, buf);

        // draw event list
        let selected_id = self.thread_view.get_selected();
        match selected_id.and_then(|x| self.trace.get(x).map(|y| (x, y))) {
            None => {
                TraceView::render_empty(layout.events, buf);

                Paragraph::new("")
                    .block(Block::default().borders(Borders::ALL).title("Message"))
                    .render(layout.message, buf);
            }
            Some((id, tree)) => {
                let view = self.trace_views.entry(id.to_string()).or_default();
                let search = if self.focus == Focus::SearchInput || !self.search_input.is_empty() {
                    Some(self.search_input.as_str())
                } else {
                    None
                };
                view.update_and_render(
                    search,
                    tree,
                    layout.events,
                    buf,
                    self.focus == Focus::Events,
                    self.status.is_live(),
                );

                // draw message
                if let Some(event) = view.get_selected_event(tree) {
                    let block = Block::default().borders(Borders::ALL).title("Message");
                    let block = if self.focus == Focus::Message {
                        block.border_style(Style::default().fg(Color::LightGreen))
                    } else {
                        block
                    };
                    let spans = layout::highlight_message(&event.message, &self.search_input);
                    Paragraph::new(Line::from(spans))
                        .scroll((event.message_scroll, 0))
                        .block(block)
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

fn help_text() -> Paragraph<'static> {
    Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::raw("Move   : "),
            Span::styled("<h><j><k><l>", Style::new().fg(Color::LightCyan)),
        ]),
        Line::from(vec![
            Span::raw("Page   : "),
            Span::styled("<u><d>", Style::new().fg(Color::LightCyan)),
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
        Line::from(vec![
            Span::raw("Search : "),
            Span::styled("</>", Style::new().fg(Color::LightCyan)),
        ]),
        Line::from(vec![
            Span::raw("Jump   : "),
            Span::styled("<n><N>", Style::new().fg(Color::LightCyan)),
        ]),
    ]))
}
