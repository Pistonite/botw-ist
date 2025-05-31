/// Information about singletons in the game
pub mod singleton;
pub use singleton::{singleton_info, singleton_instance, SingletonInfo};

mod structs;
pub use structs::*;

mod proxy;
pub use proxy::*;
