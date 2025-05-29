#[layered_crate::import]
use memory::{Reader, Writer, PtrToSized, Error, assert_zst};

/// Implementation of traits for primitive types
mod _impl;

/// An object with a fixed size in emulated memory.
///
/// Automatically derived on a struct with the MemObject macro,
/// which also generate the MemLayout implementation
pub trait MemObject: Sized {
    /// Size of the object in emulated memory, NOT the size of the struct!!
    const SIZE: u32;

    /// Read the object from memory
    fn read(reader: &mut Reader) -> Result<Self, Error> {
        Self::read_sized(reader, Self::SIZE)
    }

    /// Read the object from memory with the given read size.
    ///
    /// If the object has tail padding, C++ may optimize the layout
    /// of the object in other structs to be smaller than the size of the struct.
    /// In those cases, the size is needed explicitly.
    ///
    /// Incorrect size will result in an error, which usually indicates a bug
    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error>;

    /// Write the object to memory
    fn write(&self, writer: &mut Writer) -> Result<(), Error> {
        self.write_sized(writer, Self::SIZE)
    }

    /// Write the object to memory with the given write size.
    ///
    /// If the object has tail padding, C++ may optimize the layout
    /// of the object in other structs to be smaller than the size of the struct.
    /// In those cases, the size is needed explicitly.
    ///
    /// Incorrect size will result in an error, which usually indicates a bug
    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error>;
}

/// Trait for query the layout of memory objects to access members
/// as pointers, like the `->` operator in C++.
///
/// This is separate from MemObject because implementation for
/// primitive types don't have a layout
pub trait MemLayout {
    /// Type to carry the layout information (zero sized type)
    type Layout;

    fn __layout() -> Self::Layout;
    fn __layout_self(&self) -> Self::Layout {
        Self::__layout()
    }
}

/// This is used in implementations of MemLayout
#[doc(hidden)]
pub struct FieldMetadata<T, const OFFSET: u32, const SIZE: u32>(std::marker::PhantomData<T>);
impl<T, const OFFSET: u32, const SIZE: u32> FieldMetadata<T, OFFSET, SIZE> {
    assert_zst!();
    #[inline(always)]
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        FieldMetadata(std::marker::PhantomData)
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
    pub const fn add(&self, base: u64) -> PtrToSized<T, SIZE> {
        PtrToSized::new_const(base + OFFSET as u64)
    }
}

/// Used internally to make function accept any unsigned type
#[doc(hidden)]
pub trait Unsigned: Copy {
    fn to_u64(self) -> u64;
}

/// Used internally to make function accept any unsigned type <= 32 bits
#[doc(hidden)]
pub trait Unsigned32: Copy {
    fn to_u32(self) -> u32;
    fn to_usize(self) -> usize;
}

impl<T: Unsigned32> Unsigned for T {
    #[inline(always)]
    fn to_u64(self) -> u64 {
        self.to_u32() as u64
    }
}

