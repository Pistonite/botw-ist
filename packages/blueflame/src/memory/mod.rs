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
mod proxy;
pub use proxy::*;

pub mod traits;
pub mod util;
pub mod wrapper;

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
