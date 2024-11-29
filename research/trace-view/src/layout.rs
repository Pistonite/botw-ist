//! Layout of the application

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

/// Layout definition
///
/// Call resolve() with the available rect to get the resolved layout
pub struct LayoutDefinition {
    /// Split between status line and main area
    status_split: Layout,
    /// Split between side area (left) and main area (right)
    side_main_split: Layout,
    /// Vertical split for side areas
    side_vertical_split: Layout,
    /// Vertical split for main areas
    main_vertical_split: Layout,
}

impl Default for LayoutDefinition {
    fn default() -> Self {
        let status_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Percentage(100)]);
        let side_main_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(32), Constraint::Percentage(100)]);
        let side_vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(11)]);
        let main_vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(10)]);
        Self {
            status_split,
            side_main_split,
            side_vertical_split,
            main_vertical_split,
        }
    }
}

impl LayoutDefinition {
    pub fn resolve(&self, rect: Rect) -> ResolvedLayout {
        let status_main = self.status_split.split(rect);
        let side_main = self.side_main_split.split(status_main[1]);
        let side = self.side_vertical_split.split(side_main[0]);
        let main = self.main_vertical_split.split(side_main[1]);
        ResolvedLayout {
            status: status_main[0],
            threads: side[0],
            events: main[0],
            help: side[1],
            message: main[1],
        }
    }
}

/// Resolved layout of the app
#[rustfmt::skip]
pub struct ResolvedLayout {
    pub status:  Rect,
    pub threads: Rect, pub events: Rect,


    pub help:    Rect, pub message: Rect,
}

/// Make a block layout with properties
pub fn make_block(
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
        block.border_style(Style::new().fg(Color::LightGreen))
    } else {
        block
    }
}

pub fn highlight_message(message: &str, search: &str) -> Vec<Span<'static>> {
    if search.is_empty() {
        return vec![Span::raw(message.to_string())];
    }
    let mut spans = vec![];
    for part in message.split(search) {
        spans.push(Span::raw(part.to_string()));
        spans.push(Span::styled(
            search.to_string(),
            Style::default().fg(Color::Black).bg(Color::LightYellow),
        ));
    }
    spans.pop();
    spans
}
