use crate::memory::{Error, MemObject, Reader, Writer, assert_size_eq, assert_size_range};

macro_rules! primitive_type_mem_object_impl {
    ($type:ty, $reader_fn:ident, $writer_fn:ident) => {
        impl MemObject for $type {
            const SIZE: u32 = std::mem::size_of::<$type>() as u32;
            fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
                assert_size_eq::<Self>(Self::SIZE, size, "read_sized")?;
                assert!(
                    size == Self::SIZE,
                    "Size mismatch: expected {}, got {}",
                    Self::SIZE,
                    size
                );
                reader.$reader_fn()
            }
            fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
                assert_size_eq::<Self>(Self::SIZE, size, "write_sized")?;
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

macro_rules! tuple_type_mem_object_impl {
    ($( $t:ident , )* ,$last:ident) => {
        impl< $( $t , )*  $last> MemObject for ( $($t ,)* $last ) where $( $t: MemObject, )* $last: MemObject
        {
            const SIZE: u32 = $( <$t>::SIZE + )* <$last>::SIZE;

            fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
                assert_size_range::<Self>(Self::SIZE - ( $( <$t>::SIZE +)* 0 ), Self::SIZE, size, "read_sized")?;
                Ok((
                    $( <$t as MemObject>::read_sized(reader, $t::SIZE)?,)*
                    <$last as MemObject>::read_sized(reader, size - ( $( <$t>::SIZE +)* 0 ))?
                ))
            }

            fn write_sized(&self, writer: &mut Writer, size: u32) -> Result<(), Error> {
                assert_size_range::<Self>(Self::SIZE - ( $( <$t>::SIZE +)* 0 ), Self::SIZE, size, "write_sized")?;
                #[allow(non_snake_case)]
                let ( $($t),* , $last) = &self;
                $( MemObject::write_sized($t, writer, <$t>::SIZE)?; )*
                MemObject::write_sized($last, writer, size - ( $( <$t>::SIZE +)* 0))?;
                Ok(())
            }
        }
    }
}

tuple_type_mem_object_impl!(A,,B);
tuple_type_mem_object_impl!(A,B,,C);
tuple_type_mem_object_impl!(A,B,C,,D);

impl<T: MemObject + Default, const N: usize> MemObject for [T; N] {
    const SIZE: u32 = T::SIZE * (N as u32);

    fn read_sized(reader: &mut Reader, size: u32) -> Result<Self, Error> {
        assert_size_eq::<Self>(Self::SIZE, size, "read_sized")?;
        let mut array = std::array::from_fn(|_| T::default());
        for v in array.iter_mut() {
            *v = <T as MemObject>::read(reader)?;
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
