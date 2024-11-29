use chrono::{Local, TimeZone};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use super::{TraceNode, TraceTree};

use crate::layout;

/// Display a trace tree using a list-like virtualized view
#[derive(Debug, Clone, PartialEq)]
pub struct TraceView {
    too_small: bool,
    dirty: bool,
    last_width: u16,
    last_height: u16,
    last_focused: bool,
    last_len: usize,
    last_search: Option<String>,

    /// If the view should auto scroll to the last line as new events
    /// come in
    auto_scroll: bool,

    /// index of the first line in the view
    offset: usize,
    /// index of the selected line
    selected: usize,
    rendered: Vec<Line<'static>>,

    debug: String,
}

impl Default for TraceView {
    fn default() -> Self {
        Self {
            too_small: false,
            dirty: true,
            last_width: 0,
            last_height: 0,
            last_focused: false,
            last_len: 0,
            last_search: Default::default(),
            auto_scroll: true,
            offset: 0,
            selected: 0,
            rendered: Default::default(),
            debug: Default::default(),
        }
    }
}

impl TraceView {
    pub fn get_selected(&self, tree: &TraceTree) -> Option<usize> {
        if self.selected < tree.len() {
            Some(self.selected)
        } else {
            None
        }
    }

    pub fn get_selected_event<'t>(&self, tree: &'t TraceTree) -> Option<&'t TraceNode> {
        tree.node(self.selected)
    }

    /// Select previous line
    pub fn select_previous(&mut self, tree: &TraceTree, count: usize) {
        self.set_selected(Self::find_previous_from(self.selected, tree, count));
        self.set_auto_scroll(false);
    }

    fn find_previous_from(from: usize, tree: &TraceTree, count: usize) -> usize {
        let mut new = from;
        for _ in 0..count {
            match tree.find_list_previous(new) {
                Some(x) => {
                    new = x;
                }
                None => {
                    break;
                }
            }
        }
        new
    }

    /// Select next line
    pub fn select_next(&mut self, tree: &TraceTree, count: usize) {
        let new = Self::find_next_from(self.selected, tree, count);
        if !self.auto_scroll {
            if let Some(x) = tree.find_list_last() {
                if new == x {
                    self.set_auto_scroll(true);
                }
            }
        }
        self.set_selected(new);
    }

    fn find_next_from(from: usize, tree: &TraceTree, count: usize) -> usize {
        let mut new = from;
        for _ in 0..count {
            match tree.find_list_next(new) {
                Some(x) => {
                    new = x;
                }
                None => {
                    break;
                }
            }
        }
        new
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
                self.set_auto_scroll(false);
                true
            }
            None => false,
        }
    }

    /// Select the last line
    pub fn select_last(&mut self, tree: &TraceTree) {
        self.set_selected(tree.find_list_last().unwrap_or_default());
        self.set_auto_scroll(true);
    }

    pub fn select_first(&mut self, tree: &TraceTree) {
        self.set_selected(0);
        if let Some(x) = tree.find_list_last() {
            if x == 0 {
                self.set_auto_scroll(true);
            } else {
                self.set_auto_scroll(false);
            }
        }
    }

    pub fn select_search_next(&mut self, tree: &mut TraceTree) {
        if let Some(x) = self.last_search.as_deref() {
            if let Some(new) = tree.find_next_contains(self.selected, x) {
                self.set_selected(new);
                if let Some(x) = tree.find_list_last() {
                    if x == new {
                        self.set_auto_scroll(true);
                    }
                }
            }
        }
    }

    pub fn select_search_prev(&mut self, tree: &mut TraceTree) {
        if let Some(x) = self.last_search.as_deref() {
            if let Some(new) = tree.find_previous_contains(self.selected, x) {
                self.set_selected(new);
                self.set_auto_scroll(false);
            }
        }
    }

    fn set_selected(&mut self, selected: usize) {
        if self.selected != selected {
            self.selected = selected;
            self.dirty = true;
        }
    }

    fn set_auto_scroll(&mut self, auto_scroll: bool) {
        if self.auto_scroll != auto_scroll {
            self.auto_scroll = auto_scroll;
            self.dirty = true;
        }
    }

    pub fn is_auto_scroll(&self) -> bool {
        self.auto_scroll
    }

    fn render_extra_text(&self, tree: &TraceTree, live: bool) -> Line<'static> {
        if !self.debug.is_empty() {
            return Line::styled(
                format!("DEBUG: {}", self.debug),
                Style::new().fg(Color::LightMagenta),
            );
        }

        if !self.auto_scroll && live {
            return Line::styled(">>> Scroll Paused <<<", Style::new().fg(Color::LightYellow));
        }

        let percentage = match tree.len() {
            0 => 0,
            x => (self.selected * 100) / x,
        };

        let search_text = if let Some(x) = &self.last_search {
            format!(" [search: {}]", x)
        } else {
            "".to_string()
        };

        Line::styled(
            format!("--- {}% ---{}", percentage, search_text),
            Style::new().fg(Color::LightYellow),
        )
    }

    /// Update the view with the new dimension and selection
    pub fn update(
        &mut self,
        search: Option<&str>,
        tree: &TraceTree,
        width: u16,
        height: u16,
        focused: bool,
        live: bool,
    ) {
        if tree.is_empty() {
            self.rendered.clear();
            return;
        }
        if !self.too_small
            && !self.dirty
            && self.last_len == tree.len()
            && self.last_search.as_deref() == search
            && self.last_width == width
            && self.last_height == height
            && self.last_focused == focused
        {
            return;
        }
        self.debug.clear();
        self.last_len = tree.len();
        self.dirty = false;
        self.too_small = false;
        self.last_width = width;
        self.last_height = height;
        self.last_focused = focused;
        self.last_search = search.map(|x| x.to_string());

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
        let mut offset: usize;
        // render in a loop, if a line in the context becomes visible as
        // a result of rendering, adjust the context and try again
        'render: loop {
            context_size = if context.is_empty() {
                0
            } else {
                context.len() + 1
            };
            let scroll_padding = ((height as usize - context_size) / 8).clamp(1, 4);
            // plus 2 for main content and debug line
            if (height as usize) < scroll_padding * 2 + context_size + 2 {
                self.too_small = true;
                return;
            }
            let height = height as usize;
            // minus 1 for debug line
            main_height = height - context_size - 1;
            // update offset if scrolled out of view
            // we cannot update self.offset until after the loop
            offset = self.offset;
            {
                let lower = Self::find_previous_from(self.selected, tree, scroll_padding);
                if self.offset > lower {
                    offset = lower;
                } else {
                    let upper1 = Self::find_next_from(self.selected, tree, scroll_padding);
                    let upper = Self::find_previous_from(upper1, tree, main_height - 1);
                    if self.offset < upper {
                        offset = upper;
                    }
                }
            }
            // compute indices and relative line number for the main content
            indices.clear();
            let mut i = Some(offset);
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
        self.offset = offset;
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
            match idx.cmp(&self.selected) {
                std::cmp::Ordering::Equal => {
                    // line numbers before
                    for j in (0..line_number_before).rev() {
                        line_numbers.push(format!("{:>width$}", j + 1, width = line_number_size))
                    }
                    // line number of selected line
                    line_numbers.push(format!("{:>width$}", "-", width = line_number_size));
                }
                std::cmp::Ordering::Greater => {
                    // line numbers after
                    line_numbers.push(format!(
                        "{:>width$}",
                        line_number_after,
                        width = line_number_size
                    ));
                    line_number_after += 1;
                }
                _ => {}
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

        // render extra text indicator
        self.rendered.push(self.render_extra_text(tree, live));
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

        let message = node.single_line_message();
        let (message, ellipsis) = truncate_message(&message, available - nest_space - 2);

        let mut spans = vec![
            Span::raw(format!("{}|", line_number)),
            Span::raw(timestamp),
            Span::raw("| "),
            Span::raw(" ".repeat(nest_space)),
            Span::raw(expand_indicator),
        ];

        spans.extend(layout::highlight_message(
            message,
            self.last_search.as_deref().unwrap_or_default(),
        ));
        spans.push(Span::raw(ellipsis));

        let line = Line::from(spans);
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

impl TraceView {
    pub fn render_empty(area: Rect, buf: &mut Buffer) {
        Paragraph::new("No thread selected")
            .block(
                Block::default()
                    .title("Events")
                    .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT),
            )
            .render(area, buf);
    }
    pub fn update_and_render(
        &mut self,
        search: Option<&str>,
        tree: &TraceTree,
        area: Rect,
        buf: &mut Buffer,
        focused: bool,
        live: bool,
    ) {
        let block = layout::make_block(
            "Events",
            self.get_selected(tree),
            tree.len(),
            focused,
            false,
        );
        let inner_area = block.inner(area);

        self.update(
            search,
            tree,
            inner_area.width,
            inner_area.height,
            focused,
            live,
        );
        block.render(area, buf);
        self.render(inner_area, buf);
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
