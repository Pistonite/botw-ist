/// Round down first arg to the next multiple of second arg.
///
/// Second arg can be `page` or `region`
///
/// (This is a macro to easily support any integer type)
#[macro_export]
macro_rules! align_down {
    ($addr:expr, $align:expr) => {
        $addr & !($align - 1)
    };
}

/// Round up first arg to the next multiple of second arg.
///
/// (This is a macro to easily support any integer type)
#[macro_export]
macro_rules! align_up {
    ($addr:expr, $align:expr) => {{
        let align = $align;
        $crate::align_down!($addr + align - 1, align)
    }};
}

#[cfg(test)]
mod test {

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
