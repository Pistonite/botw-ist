use ratatui::widgets::ListState;
use ratatui::{prelude::*, widgets::List};

use crate::layout;
use crate::trace::{TraceThreadMap, TraceTree};

/// List view of threads
#[derive(Debug, Default)]
pub struct ThreadView {
    /// Threads sorted by number of events (name, count)
    list: Vec<(String, usize)>,
    /// State of the threads list, including which thread is selected
    state: ListState,
    /// Is the thread view focused
    focused: bool,
}

impl ThreadView {
    /// Update the thread list based on the trace data
    ///
    /// This should be called whenever the trace data is updated,
    /// since the list might need to be re-sorted
    pub fn update(&mut self, trace: &TraceThreadMap<TraceTree>) {
        // get the selected thread id
        let selected_id = Self::get_selected_impl(&self.list, &self.state);
        let mut new_list = trace
            .iter()
            .map(|(k, v)| (k.to_string(), v.len()))
            .collect::<Vec<_>>();
        // sort by number of events
        new_list.sort_by_key(|x| std::cmp::Reverse(x.1));

        // if there are no threads, clear the selection
        if new_list.is_empty() {
            self.state.select(None);
        } else if let Some(selected) = selected_id {
            // if the selected thread is not in the list, clear the selection
            if !trace.contains_key(selected) {
                self.state.select(None);
            }
        }

        // if there is no selection, select the first thread if possible
        if self.state.selected().is_none() && !new_list.is_empty() {
            self.state.select(Some(0));
        } else {
            // update the selection index based on the new list
            if let Some(selected) = selected_id {
                self.state
                    .select(new_list.iter().position(|x| x.0 == selected));
            }
        }

        self.list = new_list;
    }
    /// Get the currently selected thread id
    pub fn get_selected(&self) -> Option<&str> {
        Self::get_selected_impl(&self.list, &self.state)
    }

    fn get_selected_impl<'l>(list: &'l [(String, usize)], state: &ListState) -> Option<&'l str> {
        state
            .selected()
            .and_then(|x| list.get(x))
            .or_else(|| list.last())
            .map(|x| x.0.as_str())
    }

    /// Select previous line
    pub fn select_prev(&mut self, count: usize) {
        if count <= 1 {
            self.state.select_previous();
            return;
        }
        let previous = self
            .state
            .selected()
            .map_or(usize::MAX, |i| i.saturating_sub(count));
        self.state.select(Some(previous));
    }

    /// Select next line
    pub fn select_next(&mut self, count: usize) {
        if count <= 1 {
            self.state.select_next();
            return;
        }
        let next = self.state.selected().map_or(0, |i| {
            i.saturating_add(count)
                .min(self.list.len().saturating_sub(1))
        });
        self.state.select(Some(next));
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn selected(&self) -> Option<usize> {
        match self.state.selected() {
            Some(x) if x < self.list.len() => Some(x),
            _ => None,
        }
    }
}

impl Widget for &mut ThreadView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self
            .list
            .iter()
            .map(|(name, count)| format!("{} ({})", name, count));
        let thread_list = List::new(items).scroll_padding(4);
        let thread_list = if self.focused {
            thread_list.highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        } else {
            thread_list.highlight_style(Style::default().fg(Color::LightYellow))
        };
        let thread_list = thread_list.block(layout::make_block(
            "Threads",
            self.selected(),
            self.list.len(),
            self.focused,
            true,
        ));

        StatefulWidget::render(thread_list, area, buf, &mut self.state);
    }
}
