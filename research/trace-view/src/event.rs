//! Event messaging of the application

use std::io::BufReader;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

use crate::status::StatusLevel;
use crate::trace::{self, TraceEventPayload};

/// Event messages
pub enum Msg {
    /// Terminate the application
    Terminate,
    /// Change focus to a new point
    Focus(u16, u16),
    /// Buffer action count by using number keys
    ActionCount(usize),
    /// Keyboard event from terminal
    Key(Key),
    /// Character input event
    Input(Input),
    /// Trace event from live connection
    Trace(TraceEventPayload),
    /// Status update,
    Status(StatusLevel, String),
    /// Request rerender without changing the state
    Rerender,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Char(char),
    Backspace,
    Done,
}

/// Key events
///
/// These map to OS key events. What actions they map to depends on the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// Move up
    Up,
    /// Move down
    Down,
    /// Move left
    Left,
    /// Move right
    Right,
    /// First key
    First,
    /// Last key
    Last,
    /// Page down key
    PageDown,
    /// Page up key
    PageUp,
    /// Quit key
    Quit,
    /// Enter/Select key
    Enter,
    /// View Key
    View,
    /// Jump to next search result
    SearchNext,
    /// Jump to previous search result
    SearchPrev,
}

/// A shared switch for stopping the application
#[derive(Clone, Default)]
pub struct StopSignal {
    stop: Arc<AtomicBool>,
}

impl StopSignal {
    pub fn stop(&self) {
        self.stop.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_stopped(&self) -> bool {
        self.stop.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Abstract event loop that may have one or more event sources
///
/// Implementation should take a stop signal and automatically
/// clean up the resources on Drop
pub trait EventLoop {
    /// Receive the next event. Will block until an event is available.
    ///
    /// After returning an event, the event loop should wait for resume()
    /// before blocking again on event sources, in case the app wants to exit
    ///
    /// Returns None if it's not possible to receive more events.
    fn recv(&self) -> Option<Msg>;
}

/// Event loop for handling open a dump file
pub struct FileModeEventLoop {
    recv: Receiver<Msg>,
    thread: Option<JoinHandle<()>>,
}

impl FileModeEventLoop {
    pub fn new(stop: &StopSignal) -> Self {
        let (send, recv) = mpsc::channel();
        let thread = start_terminal_event_thread(stop, send);
        Self {
            recv,
            thread: Some(thread),
        }
    }
}

impl EventLoop for FileModeEventLoop {
    fn recv(&self) -> Option<Msg> {
        self.recv.recv().ok()
    }
}

impl Drop for FileModeEventLoop {
    fn drop(&mut self) {
        // drop the receiver so the thread can exit
        if let Some(thread) = self.thread.take() {
            thread.join().ok();
        }
    }
}

fn start_terminal_event_thread(stop: &StopSignal, send: Sender<Msg>) -> JoinHandle<()> {
    let stop = stop.clone();
    {
        let stop = stop.clone();
        let send = send.clone();
        let ctrlc_times = AtomicUsize::new(0);
        let _ = ctrlc::set_handler(move || {
            match ctrlc_times.load(std::sync::atomic::Ordering::Relaxed) {
                0 => {
                    // send terminate message for the first time
                    let _ = send.send(Msg::Terminate);
                }
                1 => {
                    // try invoking the stop signal and send terminate again
                    stop.stop();
                    let _ = send.send(Msg::Terminate);
                }
                2 => {
                    // inform the user
                    let _ = send.send(Msg::Status(
                        StatusLevel::Warning,
                        "Ctrl+C again to force quit".to_string(),
                    ));
                    eprintln!("Ctrl+C again to force quit");
                }
                _ => {
                    // force quit
                    std::process::exit(1);
                }
            }
            ctrlc_times.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });
    }
    thread::spawn(move || {
        while !stop.is_stopped() {
            match event::poll(Duration::from_millis(20)) {
                Err(_) => {
                    // wait for a bit before polling again
                    thread::sleep(Duration::from_millis(100));
                }
                Ok(false) => {
                    continue;
                }
                _ => {}
            };
            match event::read() {
                Err(_) => {
                    // wait for a bit before polling again
                    thread::sleep(Duration::from_millis(100));
                }
                Ok(Event::Mouse(e)) => match e.kind {
                    MouseEventKind::Down(_) => {
                        let _ = send.send(Msg::Focus(e.column, e.row));
                    }
                    MouseEventKind::ScrollUp => {
                        let _ = send.send(Msg::Focus(e.column, e.row));
                        let _ = send.send(Msg::Key(Key::Up));
                    }
                    MouseEventKind::ScrollDown => {
                        let _ = send.send(Msg::Focus(e.column, e.row));
                        let _ = send.send(Msg::Key(Key::Down));
                    }
                    _ => {}
                },
                Ok(Event::Key(e)) if e.kind == KeyEventKind::Press => {
                    // handle key bindings
                    let msg = match e.code {
                        KeyCode::Char('h') | KeyCode::Left => Some(Msg::Key(Key::Left)),
                        KeyCode::Char('j') | KeyCode::Down => Some(Msg::Key(Key::Down)),
                        KeyCode::Char('k') | KeyCode::Up => Some(Msg::Key(Key::Up)),
                        KeyCode::Char('l') | KeyCode::Right => Some(Msg::Key(Key::Right)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Msg::Key(Key::Quit)),
                        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('o') => {
                            Some(Msg::Key(Key::Enter))
                        }
                        KeyCode::Char('v') => Some(Msg::Key(Key::View)),
                        KeyCode::Char('u') | KeyCode::PageUp | KeyCode::Char('[') => {
                            Some(Msg::Key(Key::PageUp))
                        }
                        KeyCode::Char('d') | KeyCode::PageDown | KeyCode::Char(']') => {
                            Some(Msg::Key(Key::PageDown))
                        }
                        KeyCode::Char('g') => Some(Msg::Key(Key::First)),
                        KeyCode::Char('G') => Some(Msg::Key(Key::Last)),
                        KeyCode::Char('c')
                            if (e.modifiers & KeyModifiers::CONTROL) != KeyModifiers::NONE =>
                        {
                            Some(Msg::Terminate)
                        }
                        KeyCode::Char(x) if x.is_ascii_digit() => {
                            Some(Msg::ActionCount(x as usize - '0' as usize))
                        }
                        KeyCode::Char('n') => Some(Msg::Key(Key::SearchNext)),
                        KeyCode::Char('N') => Some(Msg::Key(Key::SearchPrev)),
                        _ => None,
                    };
                    if let Some(msg) = msg {
                        let _ = send.send(msg);
                    }
                    match e.code {
                        KeyCode::Char(x) => {
                            let _ = send.send(Msg::Input(Input::Char(x)));
                        }
                        KeyCode::Backspace => {
                            let _ = send.send(Msg::Input(Input::Backspace));
                        }
                        KeyCode::Enter | KeyCode::Esc => {
                            let _ = send.send(Msg::Input(Input::Done));
                        }
                        _ => {}
                    }
                }
                Ok(Event::Resize(_, _)) => {
                    let _ = send.send(Msg::Rerender);
                }
                _ => {}
            }
        }
    })
}

/// Event loop for handling live connection
pub struct LiveModeEventLoop {
    recv: Receiver<Msg>,
    term_thread: Option<JoinHandle<()>>,
    live_thread: Option<JoinHandle<()>>,
}

impl LiveModeEventLoop {
    pub fn new(stop: &StopSignal, address: &str) -> Self {
        let (send, recv) = mpsc::channel();
        let term_thread = start_terminal_event_thread(stop, send.clone());
        let live_thread = start_live_client_thread(stop, send, address);
        Self {
            recv,
            term_thread: Some(term_thread),
            live_thread: Some(live_thread),
        }
    }
}

impl EventLoop for LiveModeEventLoop {
    fn recv(&self) -> Option<Msg> {
        self.recv.recv().ok()
    }
}

impl Drop for LiveModeEventLoop {
    fn drop(&mut self) {
        if let Some(thread) = self.term_thread.take() {
            thread.join().ok();
        }
        if let Some(thread) = self.live_thread.take() {
            thread.join().ok();
        }
    }
}

fn start_live_client_thread(stop: &StopSignal, send: Sender<Msg>, address: &str) -> JoinHandle<()> {
    let stop = stop.clone();
    let address = address.to_string();
    thread::spawn(move || {
        'main: while !stop.is_stopped() {
            // connection loop
            let _ = send.send(Msg::Status(StatusLevel::Info, "connecting...".to_string()));
            let stream = loop {
                if stop.is_stopped() {
                    break 'main;
                }
                match TcpStream::connect(&address) {
                    Ok(stream) => break stream,
                    Err(_) => {
                        for s in (1..=3).rev() {
                            let _ = send.send(Msg::Status(
                                StatusLevel::Error,
                                format!("failed to connect! retrying in {} seconds...", s),
                            ));
                            for _ in 0..5 {
                                thread::sleep(Duration::from_millis(200));
                                if stop.is_stopped() {
                                    break 'main;
                                }
                            }
                        }
                    }
                }
            };
            let _ = send.send(Msg::Status(StatusLevel::Info, "Connected!".to_string()));
            if stream.set_nonblocking(true).is_err() {
                let _ = send.send(Msg::Status(
                    StatusLevel::Warning,
                    "failed to set non-blocking".to_string(),
                ));
            }
            if stream
                .set_read_timeout(Some(Duration::from_millis(200)))
                .is_err()
            {
                let _ = send.send(Msg::Status(
                    StatusLevel::Warning,
                    "failed to set non-blocking".to_string(),
                ));
            }
            // read loop
            let mut reader = BufReader::new(stream);
            let mut buffer = Vec::new();

            loop {
                match trace::read_trace_event(&stop, &mut reader, &mut buffer) {
                    Err(e) => {
                        let _ = send.send(Msg::Status(
                            StatusLevel::Error,
                            format!("failed to read trace event: {}", e),
                        ));
                        if !stop.is_stopped() {
                            // reconnect after 2 seconds
                            for _ in 0..10 {
                                thread::sleep(Duration::from_millis(200));
                                if stop.is_stopped() {
                                    break 'main;
                                }
                            }
                        }
                        continue 'main;
                    }
                    Ok(payload) => {
                        let _ = send.send(Msg::Trace(payload));
                    }
                }
            }
        }
    })
}
