mod access;
pub use access::*;
mod asserts;
pub use asserts::*;
mod error;
pub use error::*;
mod heap;
pub use heap::*;
#[allow(clippy::module_inception)]
mod memory;
pub use memory::*;
mod page;
pub use page::*;
mod pointer;
pub use pointer::*;
mod proxy;
pub use proxy::*;
mod read;
pub use read::*;
mod write;
pub use write::*;

mod section;
pub use section::*;

#[doc(hidden)]
pub mod traits;
#[doc(inline)]
pub use traits::{MemLayout, MemObject};

pub use blueflame_deps::{align_down, align_up};
pub use blueflame_proc_macros::MemObject;

pub const PAGE_SIZE: u32 = 0x1000;
pub const REGION_ALIGN: u64 = 0x10000;
