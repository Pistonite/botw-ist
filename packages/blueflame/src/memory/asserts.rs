#[layered_crate::import]
use memory::Error;

pub use blueflame_macros::{assert_zst, assert_size_less_than};

/// Assertion for size check in read() and write() implementations
pub fn assert_size_eq<T>(expected: u32, actual: u32, caller: &'static str) -> Result<(), Error> {
    if expected != actual {
        return Err(Error::SizeAssert(
            format!("{}::{}", std::any::type_name::<T>(), caller),
            expected,actual
        ));
    }
    Ok(())
}

/// Assertion for size range check in read() and write() implementations
pub fn assert_size_range<T>(min: u32, max: u32, actual: u32, caller: &'static str) -> Result<(), Error> {
    if actual < min || actual > max {
        return Err(Error::SizeRangeAssert(
            format!("{}::{}", std::any::type_name::<T>(), caller),
            min,max,actual
        ));
    }
    Ok(())
}
