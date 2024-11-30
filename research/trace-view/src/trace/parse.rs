use std::io::{BufReader, Read};
use std::net::TcpStream;

use anyhow::{bail, Result};

use crate::event::StopSignal;

use super::{TraceEvent, TraceEventPayload};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("malformed event in input stream")]
    Malformed,
    #[error("failed to read timestamp from event")]
    Timestamp,
    #[error("failed to read thread id from event")]
    Thread,
    #[error("failed to read level from event")]
    Level,
}

macro_rules! handle {
    ($e:expr) => {
        match $e {
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            x => x?,
        }
    };
}

/// Read the trace event from the input stream, will block if the input reader blocks
///
/// The buf is a temporary buffer and will not contain meaningful data
pub fn read_trace_event(
    stop: &StopSignal,
    read: &mut BufReader<TcpStream>,
    buf: &mut Vec<u8>,
) -> Result<TraceEventPayload> {
    enum State {
        Accept,
        New,   // { -> Open, * -> New
        Open,  // { -> Body, * -> Open
        Body,  // } -> Close, * -> Body
        Close, // } -> Accept, * -> Body
    }
    let mut state = State::New;
    let mut temp = [0u8; 1];
    loop {
        if stop.is_stopped() {
            bail!("stop signal received");
        }
        match state {
            State::Accept => {
                if buf.is_empty() {
                    state = State::New;
                    continue;
                }
                return parse_trace_event(buf);
            }
            State::New => {
                handle!(read.read_exact(&mut temp));
                if temp[0] == b'{' {
                    state = State::Open;
                }
            }
            State::Open => {
                handle!(read.read_exact(&mut temp));
                if temp[0] == b'{' {
                    buf.clear();
                    state = State::Body;
                } else {
                    bail!(ParseError::Malformed)
                }
            }
            State::Body => {
                handle!(read.read_exact(&mut temp));
                if temp[0] == b'}' {
                    state = State::Close;
                } else {
                    buf.push(temp[0]);
                }
            }
            State::Close => {
                handle!(read.read_exact(&mut temp));
                if temp[0] == b'}' {
                    state = State::Accept;
                } else {
                    buf.push(temp[0]);
                    state = State::Body;
                }
            }
        }
    }
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
    let timestamp = iter.next().ok_or(ParseError::Timestamp)?;
    let timestamp = parse_hex(timestamp)?;
    let thread_id = iter.next().ok_or(ParseError::Thread)?;
    let thread_id = format!("0x{}", std::str::from_utf8(thread_id)?);
    let level = iter.next().ok_or(ParseError::Level)?;
    let level = parse_hex(level)?;

    let message = iter
        .map(|x| std::str::from_utf8(x))
        .collect::<std::result::Result<Vec<_>, _>>()?
        .join(" ");

    let event = TraceEvent {
        timestamp,
        level,
        message,
    };

    Ok(TraceEventPayload { thread_id, event })
}

fn parse_hex(buf: &[u8]) -> Result<u64> {
    let s = std::str::from_utf8(buf)?;
    Ok(u64::from_str_radix(s, 16)?)
}
