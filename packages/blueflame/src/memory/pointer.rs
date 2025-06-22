use std::ffi::{CStr, CString};

use num_traits::Zero;

use crate::memory::{
    AccessFlags, Error, MemLayout, MemObject, Memory, Reader, Writer, access, assert_size_eq,
};

#[doc(inline)]
pub use blueflame_deps::{Ptr, mem, offsetof};

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

    /// Change the pointer type
    #[inline(always)]
    pub const fn reinterpret<T2: MemObject, const SIZE2: u32>(self) -> PtrToSized<T2, SIZE2> {
        PtrToSized::new_const(self.value)
    }

    /// Change the pointer type
    #[inline(always)]
    pub const fn reinterpret_array<T2: MemObject, const SIZE2: u32, const LEN: usize>(
        self,
    ) -> PtrToArray<T2, SIZE2, LEN> {
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
    pub fn ith(self, i: u64) -> PtrToSized<T, ELEM_SIZE> {
        self.ith_const(i)
    }

    /// See [`ith`](Self::ith)
    #[inline(always)]
    pub const fn ith_const(self, i: u64) -> PtrToSized<T, ELEM_SIZE> {
        PtrToSized::new_const(self.value + (i * ELEM_SIZE as u64))
    }

    /// Get the length of the array
    #[inline(always)]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(self) -> usize {
        LEN
    }

    /// Change the pointer type
    #[inline(always)]
    pub const fn reinterpret<T2: MemObject, const SIZE2: u32>(self) -> PtrToSized<T2, SIZE2> {
        PtrToSized::new_const(self.value)
    }

    /// Change the pointer type
    #[inline(always)]
    pub const fn reinterpret_array<T2: MemObject, const SIZE2: u32, const LEN2: usize>(
        self,
    ) -> PtrToArray<T2, SIZE2, LEN2> {
        PtrToArray::new_const(self.value)
    }
}

#[rustfmt::skip]
const _: () = {
    // Default
    impl<T, const SIZE: u32> Default for PtrToSized<T, SIZE> { fn default() -> Self { Self::nullptr() } }
    impl<T, const SIZE: u32, const LEN: usize> Default for PtrToArray<T, SIZE, LEN> { fn default() -> Self { Self::nullptr() } }
    // Copy
    impl<T, const SIZE: u32> Clone for PtrToSized<T, SIZE> { fn clone(&self) -> Self { *self } }
    impl<T, const SIZE: u32, const LEN: usize> Clone for PtrToArray<T, SIZE, LEN> { fn clone(&self) -> Self { *self } }
    impl<T, const SIZE: u32> Copy for PtrToSized<T, SIZE> {}
    impl<T, const SIZE: u32, const LEN: usize> Copy for PtrToArray<T, SIZE, LEN> {}
    // Conversion - 
    // note we don't allow directly converting size into array - just use to_array or decay
    impl<T, const SIZE: u32> From<u64> for PtrToSized<T, SIZE> { fn from(addr: u64) -> Self { Self::new(addr) } }
    impl<T, const SIZE: u32> From<i64> for PtrToSized<T, SIZE> { fn from(addr: i64) -> Self { Self::new(addr as u64) } }
    impl<T, const SIZE: u32, const LEN: usize> From<u64> for PtrToArray<T, SIZE, LEN> { fn from(addr: u64) -> Self { Self::new(addr) } }
    impl<T, const SIZE: u32, const LEN: usize> From<i64> for PtrToArray<T, SIZE, LEN> { fn from(addr: i64) -> Self { Self::new(addr as u64) } }

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

    // Pointer Arithmetic
    impl<T, const SIZE: u32> std::ops::Add<u64> for PtrToSized<T, SIZE> {
        type Output = Self;
        fn add(self, rhs: u64) -> Self::Output {
            Self::new(self.value + rhs * SIZE as u64)
        }
    }

    // Ord, Sub probably not worth the effort
};

// Get layout of the pointee type
#[doc(hidden)]
impl<T: MemLayout, const SIZE: u32> PtrToSized<T, SIZE> {
    #[inline(always)]
    pub fn __pointee_layout(self) -> <T as MemLayout>::Layout {
        T::__layout()
    }
}

// pointer itself can be load/store from memory
impl<T: MemObject, const S: u32> MemObject for PtrToSized<T, S> {
    const SIZE: u32 = std::mem::size_of::<Self>() as u32;

    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
        assert_size_eq::<Self>(size, Self::SIZE, "read_sized")?;
        let addr = <u64 as MemObject>::read_sized(reader, size)?;
        Ok(Self::new(addr))
    }

    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
        assert_size_eq::<Self>(size, Self::SIZE, "write_sized")?;
        <u64 as MemObject>::write_sized(&self.value, writer, size)
    }
}

impl<T: MemObject, const S: u32, const L: usize> MemObject for PtrToArray<T, S, L> {
    const SIZE: u32 = std::mem::size_of::<Self>() as u32;

    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
        assert_size_eq::<Self>(size, Self::SIZE, "read_sized")?;
        let addr = <u64 as MemObject>::read_sized(reader, size)?;
        Ok(Self::new(addr))
    }

    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
        assert_size_eq::<Self>(size, Self::SIZE, "write_sized")?;
        <u64 as MemObject>::write_sized(&self.value, writer, size)
    }
}

// load/store emulation
impl<T: MemObject, const SIZE: u32> PtrToSized<T, SIZE> {
    /// Load the object from emulated memory onto owned type of `T`
    ///
    /// This is equivalent to the C operation:
    /// ```c
    /// T* ptr; // some pointer
    /// T obj = *ptr;
    /// ```
    ///
    /// Any region is allowed. Use [`load_with`](Self::load_with) to specify extra flags.
    pub fn load(self, memory: &Memory) -> Result<T, Error> {
        self.load_with(memory, access!(default))
    }

    /// Load the object from emulated memory onto owned type of `T` with region restriction
    ///
    /// See [`load`](Self::load) for more details
    pub fn load_with(self, memory: &Memory, flags: AccessFlags) -> Result<T, Error> {
        let mut reader = memory.read(self.value, flags)?;
        <T as MemObject>::read_sized(&mut reader, SIZE)
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
    /// Any region is allowed. Use [`store_with`](Self::store_with) to specify extra flags.
    pub fn store(self, t: &T, memory: &mut Memory) -> Result<(), Error> {
        self.store_with(t, memory, access!(default))
    }

    /// Store the object into emulated memory with region restriction
    ///
    /// See [`store`](Self::store) for more details
    pub fn store_with(self, t: &T, memory: &mut Memory, flags: AccessFlags) -> Result<(), Error> {
        let mut writer = memory.write(self.value, flags)?;
        MemObject::write_sized(t, &mut writer, SIZE)
    }

    /// Store slice of the object into emulated memory as an array
    ///
    /// This is equivalent to the C operation:
    /// ```c
    /// T* array; // some pointer to an array
    /// int size; // size of the array
    /// T* ptr;   // ptr to store to
    /// for (int i = 0; i < size; i++) {
    ///     *ptr++ = array[i];
    /// }
    /// ```
    ///
    /// Any region is allowed. Use [`store_slice_with`](Self::store_slice_with) to specify extra flags.
    pub fn store_slice(self, t: &[T], memory: &mut Memory) -> Result<(), Error> {
        self.store_slice_with(t, memory, access!(default))
    }

    /// Store slice of the object into emulated memory as an array with region restriction
    ///
    /// See [`store_slice`](Self::store_slice) for more details
    pub fn store_slice_with(
        self,
        t: &[T],
        memory: &mut Memory,
        flags: AccessFlags,
    ) -> Result<(), Error> {
        let mut writer = memory.write(self.value, flags)?;
        for x in t {
            MemObject::write_sized(x, &mut writer, SIZE)?;
        }
        Ok(())
    }
}

// load/store array
impl<T: MemObject, const ELEM_SIZE: u32, const LEN: usize> PtrToArray<T, ELEM_SIZE, LEN> {
    /// Load all values in the array from emulated memory onto a Vec
    ///
    /// Any region is allowed. Use [`load_vec_with`](Self::load_vec_with) to specify extra flags.
    pub fn load_vec(self, memory: &Memory) -> Result<Vec<T>, Error> {
        self.load_vec_with(memory, access!(default))
    }

    /// Load all values in the array from emulated memory onto a Vec, with extra flags
    pub fn load_vec_with(self, memory: &Memory, flags: AccessFlags) -> Result<Vec<T>, Error> {
        let mut out = Vec::with_capacity(LEN);
        let mut reader = memory.read(self.value, flags)?;
        for _ in 0..LEN {
            out.push(T::read_sized(&mut reader, ELEM_SIZE)?);
        }
        Ok(out)
    }

    /// Load values from emulated memory into a slice.
    ///
    /// `min(LEN, out.len())` elements will be loaded into the slice,
    /// the number of elements loaded is returned.
    ///
    /// Any region is allowed. Use [`load_slice_with`](Self::load_slice_with) to specify extra flags.
    pub fn load_slice(self, out: &mut [T], memory: &Memory) -> Result<usize, Error> {
        self.load_slice_with(out, memory, access!(default))
    }

    /// See [`load_slice`](Self::load_slice)
    pub fn load_slice_with(
        self,
        out: &mut [T],
        memory: &Memory,
        flags: AccessFlags,
    ) -> Result<usize, Error> {
        let mut reader = memory.read(self.value, flags)?;
        let mut count = 0;
        for x in out.iter_mut().take(LEN) {
            *x = T::read_sized(&mut reader, ELEM_SIZE)?;
            count += 1;
        }
        Ok(count)
    }

    /// Store values from a slice to emulated memory
    ///
    /// `min(LEN, t.len())` elements will be stored from the slice,
    /// the number of elements stored is returned.
    ///
    /// Any region is allowed. Use [`store_with`](Self::store_with) to specify extra flags.
    pub fn store(self, t: &[T], memory: &mut Memory) -> Result<usize, Error> {
        self.store_with(t, memory, access!(default))
    }

    /// See [`store`](Self::store)
    pub fn store_with(
        self,
        t: &[T],
        memory: &mut Memory,
        flags: AccessFlags,
    ) -> Result<usize, Error> {
        let mut writer = memory.write(self.value, flags)?;
        let mut count = 0;
        for x in t.iter().take(LEN) {
            MemObject::write_sized(x, &mut writer, ELEM_SIZE)?;
            count += 1;
        }
        Ok(count)
    }
}

// load zero-terminated array
impl<T: MemObject + Zero + PartialEq + Copy, const SIZE: u32> PtrToSized<T, SIZE> {
    /// Load zero-terminated array starting from this pointer.
    ///
    /// **The returned vector DOES NOT include the zero terminator**
    ///
    /// Any region is allowed. Use [`load_zero_terminated_with`](Self::load_zero_terminated_with) to specify extra flags.
    pub fn load_zero_terminated(self, memory: &Memory) -> Result<Vec<T>, Error> {
        self.load_zero_terminated_with(memory, access!(default))
    }

    /// Load zero-terminated array starting from this pointer, with extra flags
    ///
    /// **The returned vector DOES NOT include the zero terminator**
    ///
    /// See [`load_zero_terminated`](Self::load_zero_terminated) for more details
    pub fn load_zero_terminated_with(
        self,
        memory: &Memory,
        flags: AccessFlags,
    ) -> Result<Vec<T>, Error> {
        let mut out = Vec::new();
        let mut reader = memory.read(self.value, flags)?;
        loop {
            let val = T::read_sized(&mut reader, SIZE)?;
            if val.is_zero() {
                break;
            }
            out.push(val);
        }
        Ok(out)
    }
}

// load/store string from char*
impl Ptr![u8] {
    /// Load a zero-terminated C string from the pointer as a char*
    pub fn load_c_string(self, memory: &Memory) -> Result<CString, Error> {
        let bytes = self.load_zero_terminated(memory)?;
        // we know the bytes only have one zero terminator at the end
        Ok(CString::new(bytes).unwrap())
    }

    /// Store a zero-terminated C string into the pointer as a char*
    pub fn store_c_string(self, s: impl AsRef<CStr>, memory: &mut Memory) -> Result<(), Error> {
        // store the bytes of the CString, including the zero terminator
        let bytes = s.as_ref().to_bytes_with_nul();
        self.store_slice(bytes, memory)?;
        Ok(())
    }

    /// Store a byte slice into the pointer as a char* plus a zero terminator
    pub fn store_bytes_plus_nul(
        self,
        s: impl AsRef<[u8]>,
        memory: &mut Memory,
    ) -> Result<(), Error> {
        let bytes = s.as_ref();
        self.store_slice(bytes, memory)?;
        Ptr!(<u8>(self.to_raw() + bytes.len() as u64)).store(&0u8, memory)?;
        Ok(())
    }

    /// Load a zero-terminated UTF-8 string from the pointer as a char*
    pub fn load_utf8_lossy(self, memory: &Memory) -> Result<String, Error> {
        let bytes = self.load_zero_terminated(memory)?;
        let utf8_error = match String::from_utf8(bytes) {
            Ok(s) => return Ok(s),
            Err(e) => e,
        };

        // FIXME: unstable
        // let lossy = utf8_error.into_utf8_lossy();
        let lossy = String::from_utf8_lossy(&utf8_error.into_bytes()).into_owned();

        log::warn!(
            "invalid utf-8 read from pointer: {:016x}, lossy value = {lossy}",
            self.to_raw()
        );
        // we know the bytes only have one zero terminator at the end
        Ok(lossy)
    }

    /// Store a string into memory as a zero-terminated UTF-8 string
    pub fn store_string(self, s: impl AsRef<str>, memory: &mut Memory) -> Result<(), Error> {
        self.store_bytes_plus_nul(s.as_ref().as_bytes(), memory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    pub fn test_pointers() -> anyhow::Result<()> {
        let mut mem = Memory::new_for_test();
        let ts = TestSub {
            one: 0x15,
            two: 0x20,
        };
        let ts2 = TestSubTwo { one: 0x4, two: 0x5 };
        let value_to_store = Test { t: ts, t2: ts2 };
        let p = Ptr!(<Test>(0x500));
        p.store(&value_to_store, &mut mem)?;
        let loaded_value = p.load(&mem)?;
        assert_eq!(loaded_value.t.one, 0x15);
        assert_eq!(loaded_value.t.two, 0x20);
        assert_eq!(loaded_value.t2.one, 0x4);
        assert_eq!(loaded_value.t2.two, 0x5);

        let array_to_store = [0x10u32; 20];
        let array_p = Ptr!(<u32[20]>(0x750));
        array_p.store(&array_to_store, &mut mem)?;

        let loaded_array = array_p.load_vec(&mem)?;
        assert_eq!(loaded_array.len(), 20);
        assert_eq!(loaded_array[10], 0x10u32);

        Ok(())
    }
}
