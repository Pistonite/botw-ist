
/// Command syntax
pub mod syn;
/// Command intermediate representation
pub mod cir;


mod error;
pub mod item_search;

pub fn test_message(n: u64) -> String {
    format!("Hello from Rust! You passed in {}", n)
}


mod util;

/// Generated data
mod generated {
    mod armor_upgrade;
    pub use armor_upgrade::ARMOR_UPGRADE;
}
