/// Information about singletons in the game
pub mod singleton;
pub use singleton::{SingletonInfo, singleton_info, singleton_instance};

mod structs;
pub use structs::*;

mod proxy;
pub use proxy::*;

pub use blueflame_deps::actor::{can_sell, can_stack, get_pouch_item_type, get_pouch_item_use};
