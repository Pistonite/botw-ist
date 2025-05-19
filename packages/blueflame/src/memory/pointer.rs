use enumset::EnumSet;

use crate::memory::{Memory, MemSized, Error, MemObject};

use super::RegionType;

/// A simple representation of ptr to a certain type in memory
pub struct Ptr<T> {
    /// The raw pointer value
    addr: u64,
    /// The size of the pointee (the size in emulated memory, NOT the size of the struct!!)
    size: u32,

    _marker: std::marker::PhantomData<T>,
}
impl<T: MemSized> Default for Ptr<T> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<T> Ptr<T> {
    /// Generates a new ptr given an address for a type `T`
    pub fn with_size(addr: u64, size: u32) -> Self {
        Self::with_size_const(addr, size)
    }
    #[inline(always)]
    pub const fn with_size_const(addr: u64, size: u32) -> Self {
        Ptr::<T> {
            addr,
            size,
            _marker: std::marker::PhantomData,
        }
    }

    pub const fn as_raw(&self) -> u64 {
        self.addr
    }
}

impl<T: MemSized> Ptr<T> {
    /// Generates a new ptr given an address for a type `T`
    pub fn new(addr: u64) -> Self {
        Self::new_const(addr)
    }
    #[inline(always)]
    pub const fn new_const(addr: u64) -> Self {
        Self::with_size_const(addr, T::SIZE)
    }
}

/// Regardless of T type, Ptr can be Clone and Copy, since it's just a u64
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self::with_size(self.addr, self.size)
    }
}
impl<T> Copy for Ptr<T> {}

// Conversion from raw integer
impl<T: MemSized> From<u64> for Ptr<T> {
    fn from(addr: u64) -> Self {
        Self::new(addr)
    }
}
impl<T: MemSized> From<i64> for Ptr<T> {
    fn from(addr: i64) -> Self {
        Self::new(addr as u64)
    }
}

// pointer arithmetic TODO --cleanup: do we need it?
impl<T: MemSized> std::ops::Add<usize> for Ptr<T> {
    type Output = Self;

    fn add(self, elements: usize) -> Self::Output {
        let new_addr = self.addr + (elements as u64 * T::SIZE as u64);
        Ptr::new(new_addr)
    }
}

impl<T: MemObject> Ptr<T> {
    /// Load the object from emulated memory onto owned type of `T`
    ///
    /// This is equivalent to the C operation:
    /// ```c
    /// T* ptr; // some pointer
    /// T obj = *ptr;
    /// ```
    ///
    /// Any region is allowed. Use [`load_in`](Self::load_in) to restrict the region
    ///
    /// This cannot be used to load instructions to execute. This will load
    /// the raw instruction bytes, but may not be valid to execute (because of stubs and patches)
    pub fn load(&self, memory: &Memory) -> Result<T, Error> {
        self.load_in(memory, None)
    }

    /// Load the object from emulated memory onto owned type of `T` with region restriction
    ///
    /// See [`load`](Self::load) for more details
    ///
    /// This cannot be used to load instructions to execute. This will load
    /// the raw instruction bytes, but may not be valid to execute (because of stubs and patches)
    pub fn load_in(&self, memory: &Memory, region: Option<EnumSet<RegionType>>) -> Result<T, Error> {
        let mut reader = memory.read(self.addr, region, false)?;
        <T as MemObject>::read_sized(&mut reader, self.size)
    }

    /// Store the object into emulated memory
    ///
    /// This is equivalent to the C operation:
    /// ```c
    /// T* ptr; // some pointer
    /// T obj; // some object
    /// *ptr = obj;
    /// ```
    ///
    /// Any region is allowed. Use [`store_in`](Self::store_in) to restrict the region
    pub fn store(&self, memory: &mut Memory, t: &T) -> Result<(), Error> {
        self.store_in(memory, None, t)
    }

    /// Store the object into emulated memory with region restriction
    ///
    /// See [`store`](Self::store) for more details
    pub fn store_in(&self, memory: &mut Memory, region: Option<EnumSet<RegionType>>, t: &T) -> Result<(), Error> {
        let mut writer = memory.write(self.addr, region)?;
        MemObject::write_sized(t, &mut writer, self.size)
    }
}

// TODO --cleanup: do we need this?
// impl<T> Sub<usize> for Ptr<T> {
//     type Output = Self;
//
//     fn sub(self, offset: usize) -> Self::Output {
//         let size = std::mem::size_of::<T>();
//         let new_addr = self.0 - (offset as u64 * size as u64);
//         Ptr::new(new_addr)
//     }
// }
