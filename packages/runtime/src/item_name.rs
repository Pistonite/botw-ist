use textdistance::{Algorithm, LCSStr};


#[derive(Debug, PartialEq, Eq)]
pub struct SearchName {
    /// The string used to search for this item
    pub search_str: &'static str,
    /// The actor name for the item
    pub actor: &'static str,
    /// If the item is a material
    pub is_material: bool,
    /// Length of the id from search_str
    pub id_len: u8,
}

impl SearchName {
    pub const fn new(search_str: &'static str, actor: &'static str, is_material: bool, id_len: u8) -> Self {
        Self {
            search_str,
            actor,
            is_material,
            id_len,
        }
    }

    pub fn is_arrow(&self) -> bool {
        self.search_str.contains("arrow")
    }

    pub fn get_type_for_compare(&self) -> u8 {
        if self.is_arrow() {
            1
        } else if self.is_material {
            2
        } else {
            3
        }
    }

    pub fn get_priority(&self) -> u8 {
        match self.search_str {
            "treebranch" | "torch"
            | "soupladle" | "lizaltriboomerang"
            | "woodcuttersaxe" | "lightscaletrident"
            | "bokospear" | "normalarrow" |
            "potlid" | "hylianshroom" | "energeticrhinobeetle"
            => 1,
            _ => 2
        }
    }

    pub fn id(&self) -> &str {
        &self.search_str[..self.id_len as usize]
    }

    pub fn to_result<'a, 'b>(&'b self, search_input: &'a str) -> SearchResult<'a, 'b> {
        SearchResult {
            search_input,
            result: self,
        }
    }
}


#[derive(PartialEq, Eq)]
pub struct SearchResult<'a, 'b> {
    pub search_input: &'a str,
    pub result: &'b SearchName,
}

impl std::fmt::Debug for SearchResult<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchResult")
            // .field("search_input", &self.search_input)
            .field("result", &self.result.search_str)
            .finish()
    }
}

impl PartialOrd for SearchResult<'_, '_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // less = higher priority

        // Arrow > Material > Other
        match self.result.get_type_for_compare().partial_cmp(&other.result.get_type_for_compare()) {
            Some(std::cmp::Ordering::Less) => {
                return Some(std::cmp::Ordering::Less);
            }
            Some(std::cmp::Ordering::Greater) => {
                return Some(std::cmp::Ordering::Greater);
            }
            _ => {}
        };

        // Priority: some common items have higher priority
        match self.result.get_priority().partial_cmp(&other.result.get_priority()) {
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

        let lcs = LCSStr{};
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
