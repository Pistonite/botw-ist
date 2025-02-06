use textdistance::{Algorithm, LCSStr};

use super::SearchName;

/// A comparible search result associating the search input with the found name
#[derive(PartialEq, Eq)]
pub struct SearchResult<'a, 'b> {
    pub search_input: &'a str,
    pub result: &'b SearchName,
}

impl std::fmt::Debug for SearchResult<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchResult")
            // .field("search_input", &self.search_input)
            .field("result", &self.result.extended_item_name)
            .finish()
    }
}

impl PartialOrd for SearchResult<'_, '_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // less = higher priority

        // Arrow > Material > Other
        match self
            .result
            .get_type_for_compare()
            .partial_cmp(&other.result.get_type_for_compare())
        {
            Some(std::cmp::Ordering::Less) => {
                return Some(std::cmp::Ordering::Less);
            }
            Some(std::cmp::Ordering::Greater) => {
                return Some(std::cmp::Ordering::Greater);
            }
            _ => {}
        };

        // Priority: some common items have higher priority
        match self
            .result
            .get_priority()
            .partial_cmp(&other.result.get_priority())
        {
            Some(std::cmp::Ordering::Less) => {
                return Some(std::cmp::Ordering::Less);
            }
            Some(std::cmp::Ordering::Greater) => {
                return Some(std::cmp::Ordering::Greater);
            }
            _ => {}
        };

        // LCS id with input
        let self_id = self.result.id();
        let other_id = other.result.id();

        let lcs = LCSStr {};
        let self_input_dist = lcs.for_str(self_id, self.search_input).dist();
        let other_input_dist = lcs.for_str(other_id, other.search_input).dist();
        match self_input_dist.partial_cmp(&other_input_dist) {
            Some(std::cmp::Ordering::Less) => {
                return Some(std::cmp::Ordering::Less);
            }
            Some(std::cmp::Ordering::Greater) => {
                return Some(std::cmp::Ordering::Greater);
            }
            _ => {}
        };

        match self_id.len().partial_cmp(&other_id.len()) {
            Some(std::cmp::Ordering::Less) => {
                return Some(std::cmp::Ordering::Less);
            }
            Some(std::cmp::Ordering::Greater) => {
                return Some(std::cmp::Ordering::Greater);
            }
            _ => {}
        };

        self_id.partial_cmp(&other_id)
    }
}

impl Ord for SearchResult<'_, '_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
