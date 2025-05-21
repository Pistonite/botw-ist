use crate::memory::{self as self_};

use enumset::EnumSet;


use self_::{Memory, MemLayout, Error, MemObject, Unsigned};

use super::RegionType;

pub use blueflame_macros::Ptr;

/// Wrapper around a raw physical address. i.e. a pointer
///
/// `PtrToSized<T, T::SIZE>` is equipvalent to `T*` in C/C++,
/// where `T` is a type that implements `MemObject`, and
/// can be used to load/store the object of type `T`. You can write
/// this type using the [`Ptr`] macro as `Ptr![T]`
///
/// The size information is carried on the type rather than
/// the value, so it can be fully optimized out at compile time,
/// and the size of this type remains the same as a u64.
///
/// When using `==` to compare two `Ptr`s, the size is ignored,
/// and the raw physical memory address is compared for equality.
pub struct PtrToSized<T, const SIZE: u32> {
    value: u64,
    _marker: std::marker::PhantomData<T>,
}
static_assertions::assert_eq_size!(PtrToSized<u8, 1>, u64);

/// Wrapper around a raw physical address, as the start of an array. i.e. a pointer
///
/// `PtrToArray<T, T::SIZE, L>` is equipvalent to the following in C/C++:
/// ```c
/// T array[L];
/// T* ptr = &array;
/// ```
///
/// where `T` is a type that implements `MemObject`, and
/// can be used to load/store the object of type `T`. You can write
/// this type using the [`Ptr`] macro as `Ptr![T[L]]`
pub struct PtrToArray<T, const ELEM_SIZE: u32, const LEN: usize> {
    value: u64,
    _marker: std::marker::PhantomData<T>,
}
static_assertions::assert_eq_size!(Ptr![u8], Ptr![u8[10]]);

impl<T, const SIZE: u32> PtrToSized<T, SIZE> {
    /// Generates a new ptr given an address for a type `T`
    #[inline(always)]
    pub fn new(addr: u64) -> Self {
        Self::new_const(addr)
    }

    /// Generates a new ptr given an address for a type `T`
    #[inline(always)]
    pub const fn new_const(addr: u64) -> Self {
        Self {
            value: addr,
            _marker: std::marker::PhantomData,
        }
    }

    /// Generates a nullptr
    #[inline(always)]
    pub const fn nullptr() -> Self {
        Self::new_const(0)
    }

    /// Check if the pointer is null
    #[inline(always)]
    pub const fn is_nullptr(self) -> bool {
        self.value == 0
    }

    /// Get the raw value of the pointer as a physical memory address
    #[inline(always)]
    pub const fn to_raw(self) -> u64 {
        self.value
    }

    /// Interpret the pointer as a pointer to an array of `LEN` elements
    #[inline(always)]
    pub const fn to_array<const LEN: usize>(self) -> PtrToArray<T, SIZE, LEN> {
        PtrToArray::new_const(self.value)
    }
}

impl<T, const ELEM_SIZE: u32, const LEN: usize> PtrToArray<T, ELEM_SIZE, LEN> {
    /// Generates a new ptr given an address for a type `T[LEN]`
    #[inline(always)]
    pub fn new(addr: u64) -> Self {
        Self::new_const(addr)
    }

    /// Generates a new ptr given an address for a type `T[LEN]`
    #[inline(always)]
    pub const fn new_const(addr: u64) -> Self {
        Self {
            value: addr,
            _marker: std::marker::PhantomData,
        }
    }

    /// Generates a nullptr
    #[inline(always)]
    pub const fn nullptr() -> Self {
        Self::new_const(0)
    }

    /// Check if the pointer is null
    #[inline(always)]
    pub const fn is_nullptr(self) -> bool {
        self.value == 0
    }

    /// Get the raw value of the pointer as a physical memory address
    #[inline(always)]
    pub const fn to_raw(self) -> u64 {
        self.value
    }

    /// Decay the array into just a pointer (`T*`) to the first element
    #[inline(always)]
    pub const fn decay(self) -> PtrToSized<T, ELEM_SIZE> {
        PtrToSized::new_const(self.value)
    }

    /// Get the ith element of the array as a pointer - with NO bounds check whatsoever,
    /// similar to C/C++
    ///
    /// The choice of not using the `[i]` syntax is because in C/C++, it will
    /// get the value, not the pointer. The choice of not using pointer arithmetic
    /// is because that is confusing when blended into other math.
    ///
    /// This can accept any unsigned type, use `ith_const` if you need to use in const context
    #[inline(always)]
    pub fn ith(self, i: impl Unsigned) -> PtrToSized<T, ELEM_SIZE> {
        self.ith_const(i.to_u64())
    }

    /// See [`ith`](Self::ith)
    #[inline(always)]
    pub const fn ith_const(self, i: u64) -> PtrToSized<T, ELEM_SIZE> {
        PtrToSized::new_const(self.value + (i * ELEM_SIZE as u64))
    }

    /// Get the length of the array
    #[inline(always)]
    pub const fn len(self) -> usize {
        LEN
    }
}

#[rustfmt::skip]
const _: () = {
    // Default
    impl<T, const SIZE: u32> Default for PtrToSized<T, SIZE> { fn default() -> Self { Self::nullptr() } }
    impl<T, const SIZE: u32, const LEN: usize> Default for PtrToArray<T, SIZE, LEN> { fn default() -> Self { Self::nullptr() } }
    // Copy
    impl<T, const SIZE: u32> Clone for PtrToSized<T, SIZE> { fn clone(&self) -> Self { Self::new(self.value) } }
    impl<T, const SIZE: u32, const LEN: usize> Clone for PtrToArray<T, SIZE, LEN> { fn clone(&self) -> Self { Self::new(self.value) } }
    impl<T, const SIZE: u32> Copy for PtrToSized<T, SIZE> {}
    impl<T, const SIZE: u32, const LEN: usize> Copy for PtrToArray<T, SIZE, LEN> {}
    // Conversion - 
    // note we don't allow directly converting size into array - just use to_array or decay
    impl<T, const SIZE: u32> From<u64> for PtrToSized<T, SIZE> { fn from(addr: u64) -> Self { Self::new(addr) } }
    impl<T, const SIZE: u32> From<i64> for PtrToSized<T, SIZE> { fn from(addr: i64) -> Self { Self::new(addr) } }
    impl<T, const SIZE: u32, const LEN: usize> From<u64> for PtrToArray<T, SIZE, LEN> { fn from(addr: u64) -> Self { Self::new(addr) } }
    impl<T, const SIZE: u32, const LEN: usize> From<i64> for PtrToArray<T, SIZE, LEN> { fn from(addr: i64) -> Self { Self::new(addr) } }

    // Comparison, we can compare any ptr to any other ptr
    impl<T, TRhs, const SIZE: u32, const SIZE_RHS: u32> PartialEq<PtrToSized<TRhs, SIZE_RHS>> for PtrToSized<T, SIZE> {
        fn eq(&self, other: &PtrToSized<TRhs, SIZE_RHS>) -> bool { self.value == other.value }
    }
    impl<T, TRhs, const SIZE: u32, const SIZE_RHS: u32, const LEN_RHS: usize> PartialEq<PtrToArray<TRhs, SIZE_RHS, LEN_RHS>> for PtrToSized<T, SIZE> {
        fn eq(&self, other: &PtrToArray<TRhs, SIZE_RHS, LEN_RHS>) -> bool { self.value == other.value }
    }
    impl<T, TRhs, const SIZE: u32, const LEN: usize, const SIZE_RHS: u32> PartialEq<PtrToSized<TRhs, SIZE_RHS>> for PtrToArray<T, SIZE, LEN> {
        fn eq(&self, other: &PtrToSized<TRhs, SIZE_RHS>) -> bool { self.value == other.value }
    }
    impl<T, TRhs, const SIZE: u32, const LEN: usize, const SIZE_RHS: u32, const LEN_RHS: usize> PartialEq<PtrToArray<TRhs, SIZE_RHS, LEN_RHS>> for PtrToArray<T, SIZE, LEN> {
        fn eq(&self, other: &PtrToArray<TRhs, SIZE_RHS, LEN_RHS>) -> bool { self.value == other.value }
    }
    impl<T, const SIZE: u32> Eq for PtrToSized<T, SIZE> {}
    impl<T, const SIZE: u32, const LEN: usize> Eq for PtrToArray<T, SIZE, LEN> {}

    // Ord, Add, Sub probably not worth the effort
};

/// Get layout of the pointee type
#[doc(hidden)]
impl<T: MemLayout, const SIZE: u32> PtrToSized<T, SIZE> {
    #[inline(always)]
    pub fn __pointee_layout(self) -> <T as MemLayout>::Layout {
        T::__layout()
    }
}

/// Load/store
impl<T: MemObject, const SIZE: u32> PtrToSized<T, SIZE> {
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
