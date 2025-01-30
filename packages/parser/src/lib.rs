
/// Command syntax
pub mod syn;
/// Command intermediate representation
pub mod cir;


mod error;

pub fn test_message(n: u64) -> String {
    format!("Hello from Rust! You passed in {}", n)
}

/// Item searcher
pub mod search;


mod util;

/// Generated data
mod generated {
    mod armor_upgrade;
    pub use armor_upgrade::ARMOR_UPGRADE;
    mod item_name;
    pub use item_name::ITEM_NAMES;
}
