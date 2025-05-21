mod asserts;
pub use asserts::*;


mod page;
pub use page::*;
mod error;
pub use error::*;
mod heap;
pub use heap::*;
mod region;
pub use region::*;
mod access;
pub use access::*;
mod read;
pub use read::*;
mod write;
pub use write::*;
mod memory;
pub use memory::*;
mod pointer;
pub use pointer::*;
mod proxy;
pub use proxy::*;

#[doc(hidden)]
pub mod traits;
#[doc(inline)]
pub use traits::{MemLayout, MemObject, Unsigned32, Unsigned};
// TODO --cleanup
// pub mod util;
pub mod wrapper;

mod from_reg_val;

pub use blueflame_proc_macros::MemObject;
pub use blueflame_macros::{align_up, align_down};

pub const PAGE_SIZE: u32 = 0x1000;
pub const REGION_ALIGN: u64 = 0x10000;
