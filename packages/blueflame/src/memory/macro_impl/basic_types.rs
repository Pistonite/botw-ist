use crate::memory::{MemObject, MemSized, Reader, Writer, Error};

use super::{assert_size_eq, assert_size_range};

/// Macro to make implementing MemObject for primitive types easy
macro_rules! primitive_type_mem_object_impl {
    ($type:ty, $reader_fn:ident, $writer_fn:ident) => {
        impl $crate::memory::MemSized for $type {
            const SIZE: u32 = std::mem::size_of::<$type>() as u32;
        }
        impl $crate::memory::MemObject for $type {
            fn read_sized(reader: &mut $crate::memory::Reader, size: u32) -> std::result::Result<Self, $crate::memory::Error> {
                $crate::memory::macro_impl::assert_size_eq::<Self>(Self::SIZE, size, "read_sized")?;
                assert!(size == Self::SIZE, "Size mismatch: expected {}, got {}", Self::SIZE, size);
                reader.$reader_fn()
            }
            fn write_sized(&self, writer: &mut $crate::memory::Writer, size: u32) -> std::result::Result<(), $crate::memory::Error> {
                $crate::memory::macro_impl::assert_size_eq::<Self>(Self::SIZE, size, "write_sized")?;
                writer.$writer_fn(*self)
            }
        }
    };
}
primitive_type_mem_object_impl!(u8, read_u8, write_u8);
primitive_type_mem_object_impl!(u16, read_u16, write_u16);
primitive_type_mem_object_impl!(u32, read_u32, write_u32);
primitive_type_mem_object_impl!(u64, read_u64, write_u64);
primitive_type_mem_object_impl!(i8, read_i8, write_i8);
primitive_type_mem_object_impl!(i16, read_i16, write_i16);
primitive_type_mem_object_impl!(i32, read_i32, write_i32);
primitive_type_mem_object_impl!(i64, read_i64, write_i64);
primitive_type_mem_object_impl!(bool, read_bool, write_bool);
primitive_type_mem_object_impl!(f32, read_f32, write_f32);
primitive_type_mem_object_impl!(f64, read_f64, write_f64);



impl<A: MemObject, B: MemObject> MemObject for (A, B) {
    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
        assert_size_range::<Self>(A::SIZE, Self::SIZE, size, "read_sized")?;
        let a = A::read_sized(reader, A::SIZE)?;
        let b = B::read_sized(reader, size - A::SIZE)?;
        Ok((a, b))
    }

    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
        assert_size_range::<Self>(A::SIZE, Self::SIZE, size, "write_sized")?;
        self.0.write_sized(writer, A::SIZE)?;
        self.1.write_sized(writer, size - A::SIZE)?;
        Ok(())
    }
}
impl<A: MemSized, B: MemSized> MemSized for (A, B) {
    const SIZE: u32 = A::SIZE + B::SIZE;
}

impl<A: MemObject, B: MemObject, C: MemObject> MemObject for (A, B ,C) {
    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
        assert_size_range::<Self>(A::SIZE, Self::SIZE, size, "read_sized")?;
        let a = A::read_sized(reader, A::SIZE)?;
        let b = B::read_sized(reader, B::SIZE)?;
        let c = C::read_sized(reader, size - A::SIZE - B::SIZE)?;
        Ok((a, b, c))
    }

    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
        assert_size_range::<Self>(A::SIZE, Self::SIZE, size, "write_sized")?;
        self.0.write_sized(writer, A::SIZE)?;
        self.1.write_sized(writer, B::SIZE)?;
        self.2.write_sized(writer, size - A::SIZE - B::SIZE)?;
        Ok(())
    }
}
impl<A: MemSized, B: MemSized, C: MemSized> MemSized for (A, B, C) {
    const SIZE: u32 = A::SIZE + B::SIZE + C::SIZE;
}


impl<A: MemObject, B: MemObject, C: MemObject, D: MemObject> MemObject for (A, B, C, D) {
    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
        assert_size_range::<Self>(A::SIZE, Self::SIZE, size, "read_sized")?;
        let a = A::read_sized(reader, A::SIZE)?;
        let b = B::read_sized(reader, B::SIZE)?;
        let c = C::read_sized(reader, C::SIZE)?;
        let d = D::read_sized(reader, size - A::SIZE - B::SIZE - C::SIZE)?;
        Ok((a, b, c, d))
    }

    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
        assert_size_range::<Self>(A::SIZE, Self::SIZE, size, "write_sized")?;
        self.0.write_sized(writer, A::SIZE)?;
        self.1.write_sized(writer, B::SIZE)?;
        self.2.write_sized(writer, C::SIZE)?;
        self.3.write_sized(writer, size - A::SIZE - B::SIZE - C::SIZE)?;
        Ok(())
    }
}
impl<A: MemSized, B: MemSized, C: MemSized, D: MemSized> MemSized for (A, B, C ,D) {
    const SIZE: u32 = A::SIZE + B::SIZE + C::SIZE + D::SIZE;
}

impl<T: MemObject + Default, const N: usize> MemObject for [T; N] {
    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
        assert_size_eq::<Self>(Self::SIZE, size, "read_sized")?;
        let mut array = std::array::from_fn(|_| T::default());
        for v in array.iter_mut() {
            *v = T::read(reader)?;
        }
        Ok(array)
    }

    fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
        assert_size_eq::<Self>(Self::SIZE, size, "write_sized")?;
        for v in self.iter() {
            v.write(writer)?;
        }
        Ok(())
    }
}

impl<T: MemSized, const N: usize> MemSized for [T; N] {
    const SIZE: u32 = T::SIZE * (N as u32);
}
