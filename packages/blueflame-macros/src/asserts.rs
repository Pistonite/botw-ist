
/// Statically assert the type is zero sized
#[macro_export]
macro_rules! assert_zst {
    () => {
    const __ASSERT_ZST: fn() = || {
        let _ = std::mem::transmute::<Self, ()>;
    };
    };
    ($t:ty) => {
        blueflame::__re::static_assertions::assert_eq_size!($t, ());
    }
}

/// Statically assert the size of a type is less than a given value,
/// used to verify the layout of a derived MemObject
#[macro_export]
macro_rules! assert_size_less_than {
    ($actual_size:expr, $max_space:literal) => {
        blueflame::__re::static_assertions::const_assert!($max_space >= $actual_size);
    }
}
