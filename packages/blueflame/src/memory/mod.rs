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
// TODO --cleanup will refactor this when refactoring the region stuff (the glue below)
#[allow(clippy::module_inception)]
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

pub use blueflame_proc_macros::MemObject;
pub use blueflame_deps::{align_up, align_down};

pub const PAGE_SIZE: u32 = 0x1000;
pub const REGION_ALIGN: u64 = 0x10000;

// TODO --cleanup: this needs to be removed
pub mod glue {
    pub fn access_flags_contains_region_type(flags: super::AccessFlags, region_type: super::RegionType) -> bool {
        match region_type {
            super::RegionType::Program => {
                flags.has_any(super::AccessFlags::region_program())
            }
            super::RegionType::Stack => {
                flags.has_any(super::AccessFlag::Stack)
            }
            super::RegionType::Heap => {
                flags.has_any(super::AccessFlag::Heap)
            }
        }
    }

    pub fn region_type_to_flags(region_type: super::RegionType) -> super::AccessFlags {
        match region_type {
            super::RegionType::Program => super::AccessFlags::region_program(),
            super::RegionType::Stack => super::AccessFlag::Stack.into(),
            super::RegionType::Heap => super::AccessFlag::Heap.into(),
        }
    }

    pub fn access_type_to_flags(access_type: super::AccessType) -> super::AccessFlags {
        match access_type {
            super::AccessType::Read => super::AccessFlag::Read.into(),
            super::AccessType::Write => super::AccessFlag::Write.into(),
            super::AccessType::Execute => super::AccessFlag::Execute.into(),
        }
    }
}
