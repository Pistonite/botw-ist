//! Widget for showing status message

use std::path::Path;

use ratatui::prelude::*;

#[derive(Debug)]
pub struct StatusLine {
    mode: Mode,
    source: String,
    level: StatusLevel,
    text: String,
    action_count: Option<usize>,
}

impl StatusLine {
    pub fn file(source: &Path) -> Self {
        Self {
            mode: Mode::File,
            source: source.display().to_string(),
            level: StatusLevel::Info,
            text: "file opened successfully".to_string(),
            action_count: None,
        }
    }

    pub fn live(source: &str) -> Self {
        Self {
            mode: Mode::Live,
            source: source.to_string(),
            level: StatusLevel::Warning,
            text: "waiting...".to_string(),
            action_count: None,
        }
    }
    pub fn set(&mut self, level: StatusLevel, text: impl Into<String>) {
        self.text = text.into();
        self.level = level;
    }

    pub fn is_live(&self) -> bool {
        self.mode == Mode::Live
    }

    pub fn append_action_count(&mut self, count: usize) {
        match self.action_count {
            Some(c) => self.action_count = Some(c * 10 + count),
            None => {
                if count > 0 {
                    self.action_count = Some(count)
                }
            }
        }
    }

    pub fn take_action_count(&mut self) -> usize {
        self.action_count.take().unwrap_or(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    File,
    Live,
}

impl Widget for &StatusLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mode = match self.mode {
            Mode::File => Span::styled(
                format!("[file: {}]", self.source),
                Style::new().fg(Color::LightYellow),
            ),
            Mode::Live => Span::styled(
                format!("[live: {}]", self.source),
                Style::new().fg(Color::LightGreen),
            ),
        };
        let count = if let Some(count) = self.action_count {
            Span::styled(
                format!("[count: {}]", count),
                Style::new().fg(Color::LightMagenta).italic(),
            )
        } else {
            Span::raw("")
        };
        let status = match self.level {
            StatusLevel::Info => Span::styled(
                format!("[info: {}", self.text),
                Style::new().fg(Color::White),
            ),
            StatusLevel::Warning => Span::styled(
                format!("[warn: {}", self.text),
                Style::new().fg(Color::LightYellow),
            ),
            StatusLevel::Error => Span::styled(
                format!("[error: {}", self.text),
                Style::new().fg(Color::LightRed),
            ),
        };

        Line::from(vec![Span::raw("[skybook-trace-view]"), mode, count, status]).render(area, buf);
    }
}
