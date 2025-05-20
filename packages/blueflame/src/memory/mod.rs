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

pub mod traits;
// TODO --cleanup
// pub mod util;
pub mod wrapper;

/// Implementation for proc macros (don't use directly)
pub mod macro_impl;
pub use macro_impl::{MemObject, MemSized};

macro_rules! align_down {
    ($addr:expr, $align:expr) => {
        $addr & !($align - 1)
    };
}
pub(crate) use align_down;

macro_rules! align_up {
    ($addr:expr, $align:expr) => {{
        let align = $align;
        $crate::memory::align_down!($addr + align - 1, align)
    }};
}
pub(crate) use align_up;

macro_rules! assert_zst {
    () => {
    const __ASSERT_ZST: fn() = || {
        let _ = std::mem::transmute::<Self, ()>;
    };
    };
    ($t:ty) => {
        static_assertions::assert_eq_size!($t, ());
    }
}
pub(crate) use assert_zst;

macro_rules! assert_size_less_than {
    ($actual_size:expr, $max_space:literal) => {
        static_assertions::const_assert!($max_space >= $actual_size);
    }
}
pub(crate) use assert_size_less_than;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_align_down() {
        assert_eq!(align_down!(0x1000, 0x1000), 0x1000);
        assert_eq!(align_down!(0x1001, 0x1000), 0x1000);
        assert_eq!(align_down!(0x1456, 0x1000), 0x1000);
        assert_eq!(align_down!(0x14567, 0x10000), 0x10000);
        assert_eq!(align_down!(0x1fff, 0x1000), 0x1000);
        assert_eq!(align_down!(0x2000, 0x1000), 0x2000);
    }

    #[test]
    fn test_align_up() {
        assert_eq!(align_up!(0x1000, 0x1000), 0x1000);
        assert_eq!(align_up!(0x1001, 0x1000), 0x2000);
        assert_eq!(align_up!(0x1456, 0x1000), 0x2000);
        assert_eq!(align_up!(0x14567, 0x10000), 0x20000);
        assert_eq!(align_up!(0x1fff, 0x1000), 0x2000);
        assert_eq!(align_up!(0x2000, 0x1000), 0x2000);
    }
}
