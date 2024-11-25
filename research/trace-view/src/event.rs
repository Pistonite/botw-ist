//! Event messaging of the application

use std::{sync::{atomic::AtomicBool, mpsc::{self, Receiver, Sender}, Arc}, thread::{self, JoinHandle}, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

/// Event messages
pub enum Msg {
    /// Terminate the application
    Terminate,
    /// Keyboard event from terminal
    Key(Key),
    /// Request rerender without changing the state
    Rerender,
}

/// Key events
///
/// These map to OS key events. What actions they map to depends on the application
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

/// A synchronized event receiver
///
/// Upon receiving an event, the receiver should call resume() to signal
/// the other side that it can start blocking again
pub struct EventReceiver {
    recv: Receiver<Msg>,
    resume: Sender<()>,
}

impl EventReceiver {
    pub fn recv(&self) -> Option<Msg> {
        self.recv.recv().ok()
    }

    /// Signal the event sender that it can start blocking
    pub fn resume(&self) {
        let _ = self.resume.send(());
    }
}

/// A synchronized event sender
///
/// After sending an event, the sender should wait for resume() before
/// blocking again on event sources
pub struct EventSender {
    send: Sender<Msg>,
    resume: Receiver<()>,
}

impl EventSender {
    /// Send the event and wait for the resume signal
    ///
    /// If send or recv fails, return false
    #[must_use]
    pub fn send(&self, msg: Msg) -> bool {
        if self.send.send(msg).is_ok() {
            // wait for signal to start blocking again
            self.resume.recv().is_ok()
        } else {
            false
        }
    }
}

/// Create a synchronized event channel
fn event_channel() -> (EventSender, EventReceiver) {
    let (send, recv) = mpsc::channel();
    let (send2, recv2) = mpsc::channel();
    (EventSender { send, resume: recv2 }, EventReceiver { recv, resume: send2 })
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

    /// Signal the event loop that it can start blocking
    fn resume(&self);
}

/// Event loop for handling open a dump file
pub struct FileModeEventLoop {
    recv: EventReceiver,
    thread: Option<JoinHandle<()>>,
}

impl FileModeEventLoop {
    pub fn new(stop: &StopSignal) -> Self {
        let (send, recv) = event_channel();
        let thread = start_terminal_event_thread(stop, send);
        Self {
            recv,
            thread: Some(thread),
        }
    }
}

impl EventLoop for FileModeEventLoop {
    fn recv(&self) -> Option<Msg> {
        self.recv.recv()
    }

    fn resume(&self) {
        self.recv.resume();
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

fn start_terminal_event_thread(stop: &StopSignal, send: EventSender) -> JoinHandle<()> {
    let stop = stop.clone();
    {
        let send = send.send.clone();
        let _ = ctrlc::set_handler(move || {
            let _ = send.send(Msg::Terminate);
        });
    }
    thread::spawn(move||{
        while !stop.is_stopped() {
            match event::read() {
                Err(_) => {
                    // wait for a bit before polling again
                    thread::sleep(Duration::from_millis(100));
                }
                Ok(Event::Key(e)) if e.kind == KeyEventKind::Press => {
                    // handle key bindings
                    let msg = match e.code {
                        KeyCode::Char('h') | KeyCode::Left => {
                            Some(Msg::Key(Key::Left))
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            Some(Msg::Key(Key::Down))
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            Some(Msg::Key(Key::Up))
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            Some(Msg::Key(Key::Right))
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            Some(Msg::Key(Key::Quit))
                        }
                        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('o') => {
                            Some(Msg::Key(Key::Enter))
                        }
                        KeyCode::Char('v') => {
                            Some(Msg::Key(Key::View))
                        }
                        KeyCode::Char('u') | KeyCode::PageUp | KeyCode::Char('[') => {
                            Some(Msg::Key(Key::PageUp))
                        }
                        KeyCode::Char('d') | KeyCode::PageDown | KeyCode::Char(']') => {
                            Some(Msg::Key(Key::PageDown))
                        }
                        KeyCode::Char('g') => {
                            Some(Msg::Key(Key::First))
                        }
                        KeyCode::Char('G') => {
                            Some(Msg::Key(Key::Last))
                        }
                        _ => None,
                    };
                    if let Some(msg) = msg {
                        if !send.send(msg) {
                            break;
                        }
                    }
                }
                Ok(Event::Resize(_, _)) => {
                        if !send.send(Msg::Rerender) {
                            break;
                        }
                }
                _ => {}
            }
        }
    })
}
