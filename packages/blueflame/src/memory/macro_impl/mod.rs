use crate::memory::{Reader, Writer, Ptr, Error};

pub use static_assertions as sa;

/// Implementation for primitive and basic types
mod basic_types;

/// Implementation for string types
mod string;

/// Implementation for Ptr
mod pointer;
pub(crate) use pointer::ptr;

/// An object in emulated memory. Automatically derived on a struct with macro
pub trait MemObject: Sized + MemSized {
    /// Read the object from memory
    fn read(reader: &mut Reader) -> Result<Self, Error> {
        Self::read_sized(reader, Self::SIZE)
    }
    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error>;
    /// Write the object to memory
    fn write(&self, writer: &mut Writer) -> Result<(), Error> {
        self.write_sized(writer, Self::SIZE)
    }
    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error>;
}

/// Marker for objects that have a fixed size in emulated memory
pub trait MemSized {
    /// Size of the object in emulated memory, NOT the size of the struct!!
    const SIZE: u32;
}

/// Trait for query the layout of memory objects to perform pointer arithmetic
///
/// Automatically derived on a struct with macro
pub trait GetLayout {
    /// Type to carry the layout information (zero sized type)
    type Layout;

    fn __layout() -> Self::Layout;
    fn __layout_self(&self) -> Self::Layout {
        Self::__layout()
    }
}

macro_rules! assert_zst {
    () => {
    const __ASSERT_ZST: fn() = || {
        let _ = std::mem::transmute::<Self, ()>;
    };
    };
    ($t:ty) => {
        $crate::memory::macro_impl::sa::assert_eq_size!($t, ());
    }
}
pub(crate) use assert_zst;

macro_rules! assert_size_less_than {
    ($actual_size:expr, $max_space:literal) => {
        $crate::memory::macro_impl::sa::const_assert!($max_space >= $actual_size);
    }
}
pub(crate) use assert_size_less_than;


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

pub struct FieldMetadata<T, const OFFSET: u32, const SIZE: u32>(std::marker::PhantomData<T>);
impl<T, const OFFSET: u32, const SIZE: u32> FieldMetadata<T, OFFSET, SIZE> {
    assert_zst!();
    #[inline(always)]
    pub const fn new() -> Self {
        FieldMetadata(std::marker::PhantomData)//, std::marker::PhantomData)
    }
    #[inline(always)]
    pub const fn offset() -> u32 {
        OFFSET
    }
    #[inline(always)]
    pub const fn size() -> u32 {
        SIZE
    }
    #[inline(always)]
    pub const fn add(&self, base: u64) -> Ptr<T> {
        Ptr::with_size_const(base + OFFSET as u64, SIZE)
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::memory::{MemObject, Ptr};
    use mem_macro::MemObject;

    use crate::memory::{Memory, MemoryFlags, Reader, Region, RegionType, SimpleHeap, Writer};

    use crate::error::Error;

    #[derive(MemObject)]
    #[size(0x40)]
    struct TestSub {
        #[offset(0x10)]
        one: u32,
        #[offset(0x30)]
        two: u64,
    }

    #[derive(MemObject)]
    #[size(0x40)]
    struct TestSubTwo {
        #[offset(0x0)]
        one: u64,
        #[offset(0x15)]
        two: u64,
    }

    #[derive(MemObject)]
    #[size(0xC0)]
    struct Test {
        #[offset(0x40)]
        t: TestSub,
        #[offset(0x80)]
        t2: TestSubTwo,
    }

    #[test]
    pub fn test_memread() -> anyhow::Result<()> {
        let mem_flags = MemoryFlags {
            enable_strict_region: true,
            enable_permission_check: true,
            enable_allocated_check: true,
        };
        let p = Arc::new(Region::new_rw(RegionType::Program, 0, 0));
        let s = Arc::new(Region::new_rw(RegionType::Stack, 0x100, 0x1000));
        let h = Arc::new(SimpleHeap::new(0x2000, 0, 0));
        let mut mem = Memory::new(mem_flags, p, s, h, None, None, None);
        let ts = TestSub {
            one: 0x15,
            two: 0x20,
        };
        let ts2 = TestSubTwo { one: 0x4, two: 0x5 };
        let value_to_store = Test { t: ts, t2: ts2 };
        let p: Ptr<Test> = Ptr::new(0x500);
        p.store(&mut mem, &value_to_store)?;
        let loaded_value = p.load(&mem)?;
        assert_eq!(loaded_value.t.one, 0x15);
        assert_eq!(loaded_value.t.two, 0x20);
        assert_eq!(loaded_value.t2.one, 0x4);
        assert_eq!(loaded_value.t2.two, 0x5);
    
        let array_to_store = [0x10u32; 20];
        let array_p: Ptr<[u32; 20]> = Ptr::new(0x750);
        array_p.store(&mut mem, &array_to_store)?;
    
        let loaded_array = array_p.load(&mut mem)?;
        assert_eq!(loaded_array.len(), 20);
        assert_eq!(loaded_array[10], 0x10u32);
    
        Ok(())
    }
}
