use core::str;
use std::ops::{Add, Sub};
use std::{ffi::CString, marker::PhantomData};

use super::{Memory, Reader};

use crate::error::Error;

use crate::memory::Writer;

/// A simple representation of ptr to a certain type in memory
#[derive(Clone, Copy)]
pub struct Ptr<T>(u64, std::marker::PhantomData<T>);

impl<T> Ptr<T> {
    /// Generates a new ptr given an address
    pub fn new(addr: u64) -> Self {
        Ptr::<T>(addr, PhantomData)
    }

    pub fn get_addr(&self) -> u64 {
        self.0
    }
}

impl<T> Add<usize> for Ptr<T> {
    type Output = Self;

    fn add(self, offset: usize) -> Self::Output {
        let size = std::mem::size_of::<T>();
        let new_addr = self.0 + (offset as u64 * size as u64);
        Ptr::new(new_addr)
    }
}

impl<T> Sub<usize> for Ptr<T> {
    type Output = Self;

    fn sub(self, offset: usize) -> Self::Output {
        let size = std::mem::size_of::<T>();
        let new_addr = self.0 - (offset as u64 * size as u64);
        Ptr::new(new_addr)
    }
}

impl<T> From<i64> for Ptr<T> {
    fn from(val: i64) -> Self {
        Ptr::<T>(u64::from_le_bytes(val.to_le_bytes()), PhantomData)
    }
}

impl<T> MemRead for Ptr<T> {
    fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
        let addr = reader.read_u64()?;
        Ok(Ptr::new(addr))
    }

    fn read_total_offset() -> u32 {
        size_of::<u64>() as u32
    }
}

impl<T> MemWrite for Ptr<T> {
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
        writer.write_u64(self.0).map_err(Error::Mem)
    }

    fn write_total_offset(&self) -> u32 {
        size_of::<u64>() as u32
    }
}

/// Trait used to represent objects that we want to read from memory
///
/// - Used with a derive macro on structs where you can specify the offset from the start of the struct
/// - Manually implemented for primitive types and types with unique behavior
///
/// # Example
///
/// ```
/// use mem_macro::MemRead;
///
/// #[derive(MemRead)]
/// struct ExampleRead {
///     #[offset(0x10)]
///     value_one: u64,
///     #[offset(0x60)]
///     value_two: i32
/// }
/// ```
///
/// This would allow for reading the object from memory with those specific offsets
pub trait MemRead: Sized {
    /// Given a reader pointed to the location where reading should take place, the object is read and returned
    fn read_from_mem(reader: &mut Reader) -> Result<Self, Error>;
    /// Represents the "size" of the object, i.e. how far the reader object is moved when reading
    /// - Used when chaining reads of an object to correctly offset values
    fn read_total_offset() -> u32;
}

impl<T: MemRead> Ptr<T> {
    /// Given a type that implements MemRead allows a ptr to be read from a provided memory instance
    pub fn deref(&self, memory: &Memory) -> Result<T, Error> {
        let mut reader = memory.read(self.0, None, false)?;
        T::read_from_mem(&mut reader)
    }
}

/// Trait used to represent objects that we want to write to memory
///
/// - Used with a derive macro on structs where you can specify the offset from the start of the struct
/// - Manually implemented for primitive types and types with unique behavior
///
/// # Example
///
/// ```
/// use mem_macro::MemWrite;
///
/// #[derive(MemWrite)]
/// struct ExampleWrite {
///     #[offset(0x10)]
///     value_one: u64,
///     #[offset(0x60)]
///     value_two: i32
/// }
/// ```
///
/// This would allow for writing the object from memory with those specific offsets
pub trait MemWrite: Sized {
    /// Given a writer pointed to the location where writing should take place, the object is placed in memory with correct offsets
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error>;
    /// Represents the "size" of the object, i.e. how far the writer object is moved when writing
    /// - Used when chaining writes of an object to correctly offset values
    fn write_total_offset(&self) -> u32;
}

impl<T: MemWrite> Ptr<T> {
    /// Given an instance of T write it to memory at the address of the ptr object
    pub fn store(&self, memory: &mut Memory, t: T) -> Result<(), Error> {
        let mut writer = memory.write(self.0, None)?;
        t.write_to_mem(&mut writer)
    }
}

/// Macro to make implementing MemRead and MemWrite for primitive types easy
macro_rules! primitive_mem_impl {
    ($type:ty, $reader_fn:ident, $writer_fn:ident) => {
        impl MemRead for $type {
            fn read_from_mem(reader: &mut Reader) -> Result<$type, Error> {
                (*reader).$reader_fn().map_err(Error::Mem)
            }
            fn read_total_offset() -> u32 {
                size_of::<$type>() as u32
            }
        }
        impl MemWrite for $type {
            fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
                writer.$writer_fn(self).map_err(Error::Mem)
            }
            fn write_total_offset(&self) -> u32 {
                size_of::<$type>() as u32
            }
        }
    };
}

primitive_mem_impl!(u32, read_u32, write_u32);
primitive_mem_impl!(u64, read_u64, write_u64);
primitive_mem_impl!(u8, read_u8, write_u8);
primitive_mem_impl!(i32, read_i32, write_i32);
primitive_mem_impl!(bool, read_bool, write_bool);
primitive_mem_impl!(f32, read_f32, write_f32);

impl<T: MemRead, const N: usize> MemRead for [T; N] {
    fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
        let mut r: [std::mem::MaybeUninit<T>; N] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for v in r.iter_mut().take(N) {
            *v = std::mem::MaybeUninit::new(T::read_from_mem(reader)?);
        }

        let init_self = unsafe { std::ptr::read(r.as_ptr() as *const [T; N]) };
        Ok(init_self)
    }

    fn read_total_offset() -> u32 {
        (N as u32) * T::read_total_offset()
    }
}

impl<T: MemWrite, const N: usize> MemWrite for [T; N] {
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
        for t in self.into_iter() {
            t.write_to_mem(writer)?;
        }
        Ok(())
    }

    fn write_total_offset(&self) -> u32 {
        (N as u32) * T::write_total_offset(&self[0])
    }
}

impl<T: MemRead> MemRead for (T, T) {
    fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
        let first = T::read_from_mem(reader)?;
        let second = T::read_from_mem(reader)?;
        Ok((first, second))
    }

    fn read_total_offset() -> u32 {
        T::read_total_offset() * 2
    }
}

impl<T: MemWrite> MemWrite for (T, T) {
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
        self.0.write_to_mem(writer)?;
        self.1.write_to_mem(writer)?;
        Ok(())
    }

    fn write_total_offset(&self) -> u32 {
        self.0.write_total_offset() + self.1.write_total_offset()
    }
}

impl<T: MemRead> MemRead for (T, T, T) {
    fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
        let first = T::read_from_mem(reader)?;
        let second = T::read_from_mem(reader)?;
        let third = T::read_from_mem(reader)?;
        Ok((first, second, third))
    }

    fn read_total_offset() -> u32 {
        T::read_total_offset() * 3
    }
}

impl<T: MemWrite> MemWrite for (T, T, T) {
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
        self.0.write_to_mem(writer)?;
        self.1.write_to_mem(writer)?;
        self.2.write_to_mem(writer)?;
        Ok(())
    }

    fn write_total_offset(&self) -> u32 {
        self.0.write_total_offset() + self.1.write_total_offset() + self.2.write_total_offset()
    }
}

impl<T: MemRead> MemRead for (T, T, T, T) {
    fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
        let first = T::read_from_mem(reader)?;
        let second = T::read_from_mem(reader)?;
        let third = T::read_from_mem(reader)?;
        let fourth = T::read_from_mem(reader)?;
        Ok((first, second, third, fourth))
    }

    fn read_total_offset() -> u32 {
        T::read_total_offset() * 4
    }
}

impl<T: MemWrite> MemWrite for (T, T, T, T) {
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
        self.0.write_to_mem(writer)?;
        self.1.write_to_mem(writer)?;
        self.2.write_to_mem(writer)?;
        self.3.write_to_mem(writer)?;
        Ok(())
    }

    fn write_total_offset(&self) -> u32 {
        self.0.write_total_offset()
            + self.1.write_total_offset()
            + self.2.write_total_offset()
            + self.3.write_total_offset()
    }
}

impl<T: MemWrite> MemWrite for Box<[T]> {
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
        let vec = self.into_vec();
        for t in vec {
            t.write_to_mem(writer)?;
        }
        Ok(())
    }

    fn write_total_offset(&self) -> u32 {
        let mut offset = 0;
        for t in self {
            offset += t.write_total_offset();
        }
        offset
    }
}

impl MemWrite for String {
    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
        let cstr = CString::new(self).map_err(|_e| {
            Error::Mem(super::Error::Unexpected(String::from(
                "Null error when constructing CString",
            )))
        })?;
        let bytes = cstr.to_bytes();
        for byte in bytes {
            writer.write_u8(*byte)?;
        }
        writer.write_u8(0)?; // null terminator
        Ok(())
    }

    fn write_total_offset(&self) -> u32 {
        self.len() as u32 // Always returns number of bytes
    }
}

impl Memory {
    /// Returns a String representing the string representation of the SafeString stored at address ptr
    pub fn mem_read_safe_string(&mut self, ptr: u64) -> Result<String, Error> {
        let m_string_top_addr = self.mem_read_u64(ptr + 8)?;
        let mut addr = m_string_top_addr;
        let mut byte = self.mem_read_byte(addr)?;
        let mut bytes = Vec::<u8>::new();
        while byte != 0 {
            bytes.push(byte);
            byte = self.mem_read_byte(addr)?;
            addr += 1;
        }
        let s = str::from_utf8(&bytes).map_err(|_e| {
            crate::memory::Error::Unexpected(String::from("Unexpected error reading SafeString"))
        })?;
        Ok(String::from(s))
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod mem_test {
    use std::sync::Arc;

    use super::{MemRead, MemWrite, Ptr};

    use mem_macro::{MemRead, MemWrite};

    use crate::memory::{Memory, MemoryFlags, Reader, Region, RegionType, SimpleHeap, Writer};

    use crate::error::Error;

    #[derive(MemRead, MemWrite)]
    struct TestSub {
        #[offset(0x10)]
        one: u32,
        #[offset(0x30)]
        two: u64,
    }

    #[derive(MemRead, MemWrite)]
    struct TestSubTwo {
        #[offset(0x0)]
        one: u64,
        #[offset(0x15)]
        two: u64,
    }

    #[derive(MemRead, MemWrite)]
    struct Test {
        #[offset(0x40)]
        t: TestSub,
        #[offset(0x80)]
        t2: TestSubTwo,
    }

    #[test]
    pub fn test_memread() {
        let mem_flags = MemoryFlags {
            enable_strict_region: false,
            enable_permission_check: false,
            enable_allocated_check: false,
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
        let t = Test { t: ts, t2: ts2 };
        let p: Ptr<Test> = Ptr::new(0x500);
        p.store(&mut mem, t).unwrap();
        let r = p.deref(&mut mem);
        if let Ok(test_struct) = r {
            assert_eq!(test_struct.t.one, 0x15);
            assert_eq!(test_struct.t.two, 0x20);
            assert_eq!(test_struct.t2.one, 0x4);
            assert_eq!(test_struct.t2.two, 0x5);
        }

        let test_array = [0x10u32; 20];
        let array_p: Ptr<[u32; 20]> = Ptr::new(0x750);
        array_p.store(&mut mem, test_array).unwrap();

        let r_array = array_p.deref(&mut mem);
        if let Ok(a) = r_array {
            assert_eq!(a.len(), 20);
            assert_eq!(a[10], 0x10u32);
        }
    }
}

// We can't implement Into<T> for types we didn't create (ex. can't impl Into<bool> for i64)
// So we use this instead, essentially copying Into<T> for our own usage
// Used by stub functions
// (Cannot override `as` either)
pub trait FromRegisterVal
where
    Self: Sized,
{
    fn from_register_val(value: i64, mem: &mut Memory) -> Result<Self, Error>;
}

impl FromRegisterVal for bool {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        Ok(value > 0)
    }
}
impl FromRegisterVal for usize {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        Ok(usize::from_le_bytes(value.to_le_bytes()))
    }
}
impl FromRegisterVal for u64 {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        Ok(u64::from_le_bytes(value.to_le_bytes()))
    }
}
impl FromRegisterVal for i64 {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        Ok(value)
    }
}
impl<T> FromRegisterVal for Ptr<T> {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        Ok(value.into())
    }
}
impl FromRegisterVal for i32 {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        Ok((value & 0xffff) as i32)
    }
}
impl FromRegisterVal for f32 {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        let lower_bytes: [u8; 4] = (value as u32).to_le_bytes();
        Ok(f32::from_le_bytes(lower_bytes))
    }
}

// Macro for creating implementation of FromRegisterVal for
// values that have pointers (ex. String and Vector types)
macro_rules! impl_from_reg_for_tuple {
    ($typ:ty) => {
        paste::item! {
            impl<T> FromRegisterVal for $typ
            where
                T: MemRead
            {
                fn from_register_val(value: i64, mem: &mut Memory) -> Result<Self, Error> {
                    let ptr: Ptr<$typ> = Ptr::<$typ>::new(u64::from_le_bytes(value.to_le_bytes()));
                    ptr.deref(mem)
                }
            }
        }
    };
}
impl_from_reg_for_tuple!((T, T));
impl_from_reg_for_tuple!((T, T, T));
impl_from_reg_for_tuple!((T, T, T, T));

impl FromRegisterVal for String {
    fn from_register_val(value: i64, mem: &mut Memory) -> Result<Self, Error> {
        let addr = u64::from_le_bytes(value.to_le_bytes());
        let mut reader = mem.read(addr, None, false)?;
        let mut bytes = Vec::<u8>::new();
        let mut byte = reader.read_u8()?;
        while byte != 0 {
            bytes.push(byte);
            byte = reader.read_u8()?;
        }
        let s = str::from_utf8(&bytes).map_err(|_e| {
            Error::Mem(super::Error::Unexpected(String::from(
                "Error reading string from memory",
            )))
        })?;
        Ok(String::from(s))
    }
}
