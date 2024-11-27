use std::{collections::BTreeMap, fs::File, io::BufReader, path::Path};

use anyhow::{bail, ensure, Result};
use derive_more::derive::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::Error;

/// A trace event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceEvent {
    pub timestamp: u64,
    pub level: u64,
    pub message: String,
}

/// Node in the trace event tree
///
/// This is the event data plus pointers in the tree to help
/// with navigation
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, Serialize)]
pub struct TraceNode {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    event: TraceEvent,

    /// If the node is expanded
    #[serde(skip)]
    pub expanded: bool,

    /// Position of the parent in the tree
    ///
    /// Parent of a node is the first node before this node
    /// with a lower level.
    ///
    /// If None, the node has level = 0
    #[serde(skip)]
    pub parent: Option<usize>,

    /// Position of the previous sibling in the tree
    ///
    /// Previous sibling is the first node before this node
    /// with the same level and has the same parent
    ///
    /// If None, the node is its parent's first child
    #[serde(skip)]
    pub prev_sibling: Option<usize>,

    /// Position of the next sibling in the tree
    ///
    /// Next sibling is the first node after this node
    /// with the same level and has the same parent
    ///
    /// If None, the node is its parent's last child
    #[serde(skip)]
    pub next_sibling: Option<usize>,

    /// Position of the last child of this node
    ///
    /// This is the last node in the tree that has this node
    /// as its parent
    ///
    /// If None, the node has no children
    #[serde(skip)]
    pub last_child: Option<usize>,
}

impl TraceNode {
    pub fn new(event: TraceEvent, parent: Option<usize>, prev_sibling: Option<usize>) -> Self {
        Self {
            event,
            expanded: false,
            parent,
            prev_sibling,
            next_sibling: None,
            last_child: None,
        }
    }
}

/// Tree representation of the trace
#[derive(Debug, PartialEq, Default, Serialize)]
pub struct TraceTree {
    /// Current node stack at the end of the tree
    #[serde(skip)]
    stack: Vec<usize>,
    /// Current level at the end of the tree
    #[serde(skip)]
    level: u64,
    /// The nodes in the tree
    #[serde(flatten)]
    tree: Vec<TraceNode>,
}

impl TraceTree {
    pub fn new() -> Self {
        Default::default()
    }
    /// Construct a trace tree from list of events
    ///
    /// All nodes will be collapsed
    pub fn from_events(events: impl IntoIterator<Item = TraceEvent>) -> Result<Self> {
        let mut tree = Self::new();
        tree.add_events(events)?;
        Ok(tree)
    }
    /// Append events to the tree
    ///
    /// All nodes will be collapsed
    pub fn add_events(&mut self, events: impl IntoIterator<Item = TraceEvent>) -> Result<()> {
        let mut iter = events.into_iter();
        if let (_, Some(size)) = iter.size_hint() {
            self.tree.reserve(size);
        }

        if let Some(first) = iter.next() {
            self.add_event(first)?;

            for event in iter {
                self.append_event(event)?;
            }
        }

        Ok(())
    }

    /// Add a new event to the end of the tree.
    pub fn add_event(&mut self, event: TraceEvent) -> Result<()> {
        if self.tree.is_empty() {
            self.stack = vec![0];
            self.level = event.level;
            self.push_node(TraceNode::new(event, None, None));
            return Ok(());
        }

        self.append_event(event)
    }

    /// Add a new event to the end of the tree. The tree already has some nodes
    fn append_event(&mut self, event: TraceEvent) -> Result<()> {
        loop {
            match event.level.cmp(&self.level) {
                // current node is first child of stack top
                std::cmp::Ordering::Greater => {
                    let prev = match self.stack.last_mut() {
                        Some(x) => x,
                        None => {
                            bail!("invalid trace level: stack is empty");
                        }
                    };
                    let parent = Some(*prev);
                    // set this node to be the previous sibling of the next node
                    self.stack.push(self.tree.len());
                    self.level = event.level;
                    self.push_node(TraceNode::new(event, parent, None));
                    return Ok(());
                }
                // current node is sibling of stack top
                std::cmp::Ordering::Equal => {
                    let prev = match self.stack.last_mut() {
                        Some(x) => x,
                        None => {
                            bail!("invalid trace level: stack is empty");
                        }
                    };
                    // have same parent as previous sibling
                    let prev_sibling = Some(*prev);
                    let parent = self.tree[*prev].parent;
                    *prev = self.tree.len();
                    self.push_node(TraceNode::new(event, parent, prev_sibling));
                    return Ok(());
                }
                // stack top was last child of its parent
                std::cmp::Ordering::Less => {
                    // return up a level
                    ensure!(
                        self.stack.pop().is_some(),
                        "invalid trace level: cannot return to previous level"
                    );
                    let last = match self.stack.last() {
                        Some(x) => *x,
                        None => {
                            bail!("invalid trace level: cannot return to previous level");
                        }
                    };
                    self.level = self.tree[last].event.level;
                }
            }
        }
    }

    #[inline]
    fn push_node(&mut self, node: TraceNode) {
        if let Some(parent) = node.parent {
            self.tree[parent].last_child = Some(self.tree.len());
        }
        if let Some(prev_sibling) = node.prev_sibling {
            self.tree[prev_sibling].next_sibling = Some(self.tree.len());
        }
        self.tree.push(node);
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Get the event at the given index
    pub fn get(&self, idx: usize) -> Option<&TraceEvent> {
        self.tree.get(idx).map(|x| &x.event)
    }

    /// Get the node at the given index
    pub fn node(&self, idx: usize) -> Option<&TraceNode> {
        self.tree.get(idx)
    }

    /// Is the node at the given index expanded
    pub fn is_expanded(&self, idx: usize) -> bool {
        self.tree.get(idx).map(|x| x.expanded).unwrap_or(false)
    }

    /// Set the expanded state of the node at the given index
    pub fn set_expanded(&mut self, idx: usize, expanded: bool) {
        if let Some(node) = self.tree.get_mut(idx) {
            if node.last_child.is_none() {
                // cannot expand a node without children
                return;
            }
            node.expanded = expanded;
        }
    }

    /// Find the previous node in list order, depending on collapsed state
    ///
    /// Returns None if it's the first node in the list or if idx is invalid
    pub fn find_list_previous(&self, idx: usize) -> Option<usize> {
        let node = self.node(idx)?;
        let previous = match node.prev_sibling {
            None => {
                // if the node is first child, the previous node
                // in list order is definitely its parent
                return self.ensure_valid_idx(node.parent);
            }
            Some(x) => x,
        };
        Some(self.find_last_expanded_recursive(previous, self.node(previous)?))
    }

    /// Find the next node in list order, depending on collapsed state
    ///
    /// Returns None if it's the last node in the list or if idx is invalid
    pub fn find_list_next(&self, idx: usize) -> Option<usize> {
        let node = self.node(idx)?;
        if node.expanded {
            // if node is expanded, return the next one in list order
            return self.ensure_valid_idx(Some(idx + 1));
        }
        let mut node = node;
        loop {
            match node.next_sibling {
                Some(x) => {
                    // if node has a next sibling, return it
                    return self.ensure_valid_idx(Some(x))
                }
                None => {
                    // otherwise, use the next sibling of parent
                    node = self.node(node.parent?)?;
                }
            }
        }
    }

    pub fn find_list_last(&self) -> Option<usize> {
        // start with last, find the last top level node
        let mut last_top_level = match self.tree.len() {
            0 => return None,
            x => x - 1,
        };
        loop {
            let node = self.node(last_top_level)?;
            match node.parent {
                Some(x) => last_top_level = x,
                None => break,
            }
        }
        Some(self.find_last_expanded_recursive(last_top_level, self.node(last_top_level)?))
    }

    pub fn find_parent(&self, idx: usize) -> Option<usize> {
        let node = self.node(idx)?;
        self.ensure_valid_idx(node.parent)
    }

    /// Get the list of immediate parents, up to max. Return the
    /// indices in parent -> child order and does not include self
    pub fn get_context(&self, mut idx: usize, max: usize) -> Vec<usize> {
        let mut stack = Vec::with_capacity(max);
        for _ in 0..max {
            let parent = match self.find_parent(idx) {
                Some(x) => x,
                None => break,
            };
            stack.push(parent);
            idx = parent;
        }
        stack.reverse();
        stack
    }

    /// If node at idx is expanded, return find_last_expanded_recursive(last_child)
    fn find_last_expanded_recursive(&self, idx: usize, node: &TraceNode) -> usize {
        if node.expanded {
            if let Some(last_child) = node.last_child {
                if let Some(last_child_node) = self.tree.get(last_child) {
                    return self.find_last_expanded_recursive(last_child, last_child_node);
                }
            }
        }

        idx
    }

    fn ensure_valid_idx(&self, idx: Option<usize>) -> Option<usize> {
        match idx {
            Some(x) if x < self.len() => Some(x),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct TraceEventPayload {
    thread_id: String,
    #[deref]
    #[deref_mut]
    event: TraceEvent,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ThreadTrace {
    events: Vec<TraceEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trace {
    #[serde(flatten)]
    data: BTreeMap<String, ThreadTrace>,
}

pub type TraceThreadMap<T> = BTreeMap<String, T>;

pub fn load_trace_tree_file(path: impl AsRef<Path>) -> Result<TraceThreadMap<TraceTree>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let trace: TraceThreadMap<Vec<TraceEvent>> = serde_json::from_reader(buf_reader)?;

    let mut map = TraceThreadMap::new();

    for (thread_id, events) in trace {
        let tree = TraceTree::from_events(events)?;
        map.insert(thread_id, tree);
    }

    Ok(map)
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
    let timestamp = iter.next().ok_or(Error::TraceParseError)?;
    let timestamp = parse_hex(timestamp)?;
    let thread_id = iter.next().ok_or(Error::TraceParseError)?;
    let thread_id = format!("0x{}", std::str::from_utf8(thread_id)?);
    let level = iter.next().ok_or(Error::TraceParseError)?;
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! trace {
        ($timestamp:expr, $level:expr, $($arg:tt),*) => {
            TraceEvent {
                timestamp: $timestamp,
                level: $level,
                message: format!($($arg),*)
            }
        };
    }

    #[test]
    pub fn trace_tree_new_empty() {
        let tree = TraceTree::new();
        assert!(tree.is_empty());
    }

    #[test]
    pub fn trace_tree_init_flat() {
        let tree =
            TraceTree::from_events([trace!(0, 0, "0"), trace!(1, 0, "1"), trace!(2, 0, "2")])
                .unwrap();

        assert!(tree.node(0).unwrap().parent.is_none());
        assert!(tree.node(0).unwrap().prev_sibling.is_none());
        assert_eq!(tree.node(0).unwrap().next_sibling, Some(1));
        assert_eq!(tree.node(0).unwrap().last_child, None);

        assert!(tree.node(1).unwrap().parent.is_none());
        assert_eq!(tree.node(1).unwrap().prev_sibling, Some(0));
        assert_eq!(tree.node(1).unwrap().next_sibling, Some(2));
        assert_eq!(tree.node(1).unwrap().last_child, None);

        assert!(tree.node(2).unwrap().parent.is_none());
        assert_eq!(tree.node(2).unwrap().prev_sibling, Some(1));
        assert!(tree.node(2).unwrap().next_sibling.is_none());
        assert_eq!(tree.node(2).unwrap().last_child, None);
    }

    #[test]
    pub fn trace_tree_init_simple_nest() {
        let tree =
            TraceTree::from_events([trace!(0, 0, "0"), trace!(1, 1, "1"), trace!(2, 0, "2")])
                .unwrap();

        assert!(tree.node(0).unwrap().parent.is_none());
        assert!(tree.node(0).unwrap().prev_sibling.is_none());
        assert_eq!(tree.node(0).unwrap().next_sibling, Some(2));
        assert_eq!(tree.node(0).unwrap().last_child, Some(1));

        assert_eq!(tree.node(1).unwrap().parent, Some(0));
        assert_eq!(tree.node(1).unwrap().prev_sibling, None);
        assert_eq!(tree.node(1).unwrap().next_sibling, None);
        assert_eq!(tree.node(1).unwrap().last_child, None);

        assert!(tree.node(2).unwrap().parent.is_none());
        assert_eq!(tree.node(2).unwrap().prev_sibling, Some(0));
        assert_eq!(tree.node(2).unwrap().next_sibling, None);
        assert_eq!(tree.node(2).unwrap().last_child, None);
    }

    #[test]
    pub fn trace_tree_init_simple_nest_multiple_children() {
        let tree = TraceTree::from_events([
            trace!(0, 0, "0"),
            trace!(1, 1, "n1"),
            trace!(2, 1, "n2"),
            trace!(3, 0, "2"),
            trace!(4, 1, "n3"),
            trace!(5, 1, "n4"),
        ])
        .unwrap();

        assert!(tree.node(0).unwrap().parent.is_none());
        assert!(tree.node(0).unwrap().prev_sibling.is_none());
        assert_eq!(tree.node(0).unwrap().next_sibling, Some(3));
        assert_eq!(tree.node(0).unwrap().last_child, Some(2));

        assert_eq!(tree.node(1).unwrap().parent, Some(0));
        assert_eq!(tree.node(1).unwrap().prev_sibling, None);
        assert_eq!(tree.node(1).unwrap().next_sibling, Some(2));
        assert_eq!(tree.node(1).unwrap().last_child, None);
        assert_eq!(tree.node(2).unwrap().parent, Some(0));
        assert_eq!(tree.node(2).unwrap().prev_sibling, Some(1));
        assert_eq!(tree.node(2).unwrap().next_sibling, None);
        assert_eq!(tree.node(2).unwrap().last_child, None);

        assert_eq!(tree.node(3).unwrap().parent, None);
        assert_eq!(tree.node(3).unwrap().prev_sibling, Some(0));
        assert_eq!(tree.node(3).unwrap().next_sibling, None);
        assert_eq!(tree.node(3).unwrap().last_child, Some(5));

        assert_eq!(tree.node(4).unwrap().parent, Some(3));
        assert_eq!(tree.node(4).unwrap().prev_sibling, None);
        assert_eq!(tree.node(4).unwrap().next_sibling, Some(5));
        assert_eq!(tree.node(4).unwrap().last_child, None);
        assert_eq!(tree.node(5).unwrap().parent, Some(3));
        assert_eq!(tree.node(5).unwrap().prev_sibling, Some(4));
        assert_eq!(tree.node(5).unwrap().next_sibling, None);
        assert_eq!(tree.node(5).unwrap().last_child, None);
    }

    #[test]
    pub fn trace_tree_init_multiple_nest_multiple_children() {
        let tree = TraceTree::from_events([
            trace!(0, 0, ""),
            trace!(1, 1, ""),
            trace!(2, 2, ""),
            trace!(3, 2, ""),
            trace!(4, 1, ""),
            trace!(5, 0, ""),
            trace!(6, 1, ""),
            trace!(7, 2, ""),
            trace!(8, 2, ""),
            trace!(9, 0, ""),
        ])
        .unwrap();

        assert!(tree.node(0).unwrap().parent.is_none());
        assert!(tree.node(0).unwrap().prev_sibling.is_none());
        assert_eq!(tree.node(0).unwrap().next_sibling, Some(5));
        assert_eq!(tree.node(0).unwrap().last_child, Some(4));

        assert_eq!(tree.node(1).unwrap().parent, Some(0));
        assert_eq!(tree.node(1).unwrap().prev_sibling, None);
        assert_eq!(tree.node(1).unwrap().next_sibling, Some(4));
        assert_eq!(tree.node(1).unwrap().last_child, Some(3));
        assert_eq!(tree.node(2).unwrap().parent, Some(1));
        assert_eq!(tree.node(2).unwrap().prev_sibling, None);
        assert_eq!(tree.node(2).unwrap().next_sibling, Some(3));
        assert_eq!(tree.node(2).unwrap().last_child, None);
        assert_eq!(tree.node(3).unwrap().parent, Some(1));
        assert_eq!(tree.node(3).unwrap().prev_sibling, Some(2));
        assert_eq!(tree.node(3).unwrap().next_sibling, None);
        assert_eq!(tree.node(3).unwrap().last_child, None);

        assert_eq!(tree.node(4).unwrap().parent, Some(0));
        assert_eq!(tree.node(4).unwrap().prev_sibling, Some(1));
        assert_eq!(tree.node(4).unwrap().next_sibling, None);
        assert_eq!(tree.node(4).unwrap().last_child, None);

        assert_eq!(tree.node(5).unwrap().parent, None);
        assert_eq!(tree.node(5).unwrap().prev_sibling, Some(0));
        assert_eq!(tree.node(5).unwrap().next_sibling, Some(9));
        assert_eq!(tree.node(5).unwrap().last_child, Some(6));
        assert_eq!(tree.node(6).unwrap().parent, Some(5));
        assert_eq!(tree.node(6).unwrap().prev_sibling, None);
        assert_eq!(tree.node(6).unwrap().next_sibling, None);
        assert_eq!(tree.node(6).unwrap().last_child, Some(8));
        assert_eq!(tree.node(7).unwrap().parent, Some(6));
        assert_eq!(tree.node(7).unwrap().prev_sibling, None);
        assert_eq!(tree.node(7).unwrap().next_sibling, Some(8));
        assert_eq!(tree.node(7).unwrap().last_child, None);
        assert_eq!(tree.node(8).unwrap().parent, Some(6));
        assert_eq!(tree.node(8).unwrap().prev_sibling, Some(7));
        assert_eq!(tree.node(8).unwrap().next_sibling, None);
        assert_eq!(tree.node(8).unwrap().last_child, None);

        // return 2 levels up
        assert_eq!(tree.node(9).unwrap().parent, None);
        assert_eq!(tree.node(9).unwrap().prev_sibling, Some(5));
        assert_eq!(tree.node(9).unwrap().next_sibling, None);
        assert_eq!(tree.node(9).unwrap().last_child, None);
    }
}
