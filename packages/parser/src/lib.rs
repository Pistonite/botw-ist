
/// Command syntax
mod syn;
/// Command intermediate representation
mod cir;

mod error;
mod item_search;

pub fn test_message(n: u64) -> String {
    format!("Hello from Rust! You passed in {}", n)
}
