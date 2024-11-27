//! Layout of the application

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout definition
///
/// Call resolve() with the available rect to get the resolved layout
pub struct LayoutDefinition {
    /// Split between side area (left) and main area (right)
    side_main_split: Layout,
    /// Vertical split for side areas
    side_vertical_split: Layout,
    /// Vertical split for main areas
    main_vertical_split: Layout,
}

impl Default for LayoutDefinition {
    fn default() -> Self {
        let side_main_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(32), Constraint::Percentage(100)]);
        let side_vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Percentage(100),
                Constraint::Min(11),
            ]);
        let main_vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Percentage(100),
                Constraint::Min(10),
            ]);
        Self {
            side_main_split,
            side_vertical_split,
            main_vertical_split,
        }
    }
}

impl LayoutDefinition {
    pub fn resolve(&self, rect: Rect) -> ResolvedLayout {
        let side_main = self.side_main_split.split(rect);
        let side = self.side_vertical_split.split(side_main[0]);
        let main = self.main_vertical_split.split(side_main[1]);
        ResolvedLayout {
            mode: side[0],
            status: main[0],
            threads: side[1],
            events: main[1],
            help: side[2],
            message: main[2],
        }
    }
}

/// Resolved layout of the app
#[rustfmt::skip]
pub struct ResolvedLayout {
    pub mode:    Rect, pub status: Rect,
    pub threads: Rect, pub events: Rect,


    pub help:    Rect, pub message: Rect,
}
