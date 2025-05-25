use crate::memory::Memory;
use crate::memory::Error;

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
        Ok(u64::from_le_bytes(value.to_le_bytes()) as usize)
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
// impl<T> FromRegisterVal for Ptr<T> {
//     fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
//         Ok(value.into())
//     }
// }
impl FromRegisterVal for i32 {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        Ok((value & 0xffffffff) as i32)
    }
}
impl FromRegisterVal for f32 {
    fn from_register_val(value: i64, _: &mut Memory) -> Result<Self, Error> {
        let lower_bytes: [u8; 4] = (value as u32).to_le_bytes();
        Ok(f32::from_le_bytes(lower_bytes))
    }
}

// // Macro for creating implementation of FromRegisterVal for
// // values that have pointers (ex. String and Vector types)
// macro_rules! impl_from_reg_for_tuple {
//     ($typ:ty) => {
//         paste::item! {
//             impl<T> FromRegisterVal for $typ
//             where
//                 T: MemRead
//             {
//                 fn from_register_val(value: i64, mem: &mut Memory) -> Result<Self, Error> {
//                     let ptr: Ptr<$typ> = Ptr::<$typ>::new(u64::from_le_bytes(value.to_le_bytes()));
//                     ptr.deref(mem)
//                 }
//             }
//         }
//     };
// }
// impl_from_reg_for_tuple!((T, T));
// impl_from_reg_for_tuple!((T, T, T));
// impl_from_reg_for_tuple!((T, T, T, T));
//
// impl FromRegisterVal for String {
//     fn from_register_val(value: i64, mem: &mut Memory) -> Result<Self, Error> {
//         let addr = u64::from_le_bytes(value.to_le_bytes());
//         let mut reader = mem.read(addr, None, false)?;
//         let mut bytes = Vec::<u8>::new();
//         let mut byte = reader.read_u8()?;
//         while byte != 0 {
//             bytes.push(byte);
//             byte = reader.read_u8()?;
//         }
//         let s = str::from_utf8(&bytes).map_err(|_e| {
//             Error::Mem(super::Error::Unexpected(String::from(
//                 "Error reading string from memory",
//             )))
//         })?;
//         Ok(String::from(s))
//     }
// }
