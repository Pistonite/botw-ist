/// Information about singletons in the game
pub mod singleton;
pub use singleton::{SingletonInfo, singleton_info, singleton_instance};

mod structs;
pub use structs::*;

mod functions;
mod hooks;
