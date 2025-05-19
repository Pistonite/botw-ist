use super::GetLayout;

use crate::memory::{Ptr, MemObject, MemSized};

/// Get layout of the pointee type
impl<T: GetLayout> Ptr<T> {
    #[inline(always)]
    pub fn __layout(self) -> <T as GetLayout>::Layout {
        T::__layout()
    }
}

/// Make a Ptr from C-style `&ptr->field` to navigate the layout
/// of an object without reading everything
macro_rules! ptr {
    (& $ptr:ident -> $field:ident) => {
        $crate::memory::Ptr::__layout($ptr).$field.add($ptr.as_raw())
    };
}
pub(crate) use ptr;


/// Load/Store for pointer value
impl<T: MemSized> MemObject for Ptr<T> {
    fn read_sized(reader: &mut crate::memory::Reader, size: u32) -> Result<Self, crate::memory::Error> {
        Ok(Self::new(u64::read_sized(reader, size)?))
    }

    fn write_sized(&self, writer: &mut crate::memory::Writer, size: u32) -> Result<(), crate::memory::Error> {
        self.as_raw().write_sized(writer, size)
    }
}
impl<T> MemSized for Ptr<T> {
    const SIZE: u32 = std::mem::size_of::<u64>() as u32;
}

