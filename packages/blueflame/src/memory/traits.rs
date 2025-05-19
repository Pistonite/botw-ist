use core::str;
use std::ops::{Add, Sub};
use std::{ffi::CString, marker::PhantomData};

use super::{Memory, Reader};

use crate::error::Error;

use crate::memory::Writer;

// // TODO --cleanup: remove
// impl<T: MemRead, const N: usize> MemRead for [T; N] {
//     const SIZE: usize = T::SIZE * N;
//     fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
//         let mut r: [std::mem::MaybeUninit<T>; N] =
//             unsafe { std::mem::MaybeUninit::uninit::<>().assume_init() };
//
//         for v in r.iter_mut().take(N) {
//             *v = std::mem::MaybeUninit::new(T::read_from_mem(reader)?);
//         }
//
//         let init_self = unsafe { std::ptr::read(r.as_ptr() as *const [T; N]) };
//         Ok(init_self)
//     }
// }
//
// impl<T: MemWrite, const N: usize> MemWrite for [T; N] {
//     fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//         for t in self.into_iter() {
//             t.write_to_mem(writer)?;
//         }
//         Ok(())
//     }
//
//     fn write_total_offset(&self) -> u32 {
//         (N as u32) * T::write_total_offset(&self[0])
//     }
// }

// impl<T: MemRead> MemRead for (T, T) {
//     const SIZE: usize = T::SIZE * 2;
//     fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
//         let first = T::read_from_mem(reader)?;
//         let second = T::read_from_mem(reader)?;
//         Ok((first, second))
//     }
// }
//
// impl<T: MemWrite> MemWrite for (T, T) {
//     fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//         self.0.write_to_mem(writer)?;
//         self.1.write_to_mem(writer)?;
//         Ok(())
//     }
//
//     fn write_total_offset(&self) -> u32 {
//         self.0.write_total_offset() + self.1.write_total_offset()
//     }
// }
//
// impl<T: MemRead> MemRead for (T, T, T) {
//     const SIZE: usize = T::SIZE * 3;
//     fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
//         let first = T::read_from_mem(reader)?;
//         let second = T::read_from_mem(reader)?;
//         let third = T::read_from_mem(reader)?;
//         Ok((first, second, third))
//     }
// }
//
// impl<T: MemWrite> MemWrite for (T, T, T) {
//     fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//         self.0.write_to_mem(writer)?;
//         self.1.write_to_mem(writer)?;
//         self.2.write_to_mem(writer)?;
//         Ok(())
//     }
//
//     fn write_total_offset(&self) -> u32 {
//         self.0.write_total_offset() + self.1.write_total_offset() + self.2.write_total_offset()
//     }
// }
//
// impl<T: MemRead> MemRead for (T, T, T, T) {
//     const SIZE: usize = T::SIZE * 4;
//     fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
//         let first = T::read_from_mem(reader)?;
//         let second = T::read_from_mem(reader)?;
//         let third = T::read_from_mem(reader)?;
//         let fourth = T::read_from_mem(reader)?;
//         Ok((first, second, third, fourth))
//     }
// }
//
// impl<T: MemWrite> MemWrite for (T, T, T, T) {
//     fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//         self.0.write_to_mem(writer)?;
//         self.1.write_to_mem(writer)?;
//         self.2.write_to_mem(writer)?;
//         self.3.write_to_mem(writer)?;
//         Ok(())
//     }
//
//     fn write_total_offset(&self) -> u32 {
//         self.0.write_total_offset()
//             + self.1.write_total_offset()
//             + self.2.write_total_offset()
//             + self.3.write_total_offset()
//     }
// }

// impl<T: MemWrite> MemWrite for Box<[T]> {
//     fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//         let vec = self.into_vec();
//         for t in vec {
//             t.write_to_mem(writer)?;
//         }
//         Ok(())
//     }
//
//     fn write_total_offset(&self) -> u32 {
//         let mut offset = 0;
//         for t in self {
//             offset += t.write_total_offset();
//         }
//         offset
//     }
// }

// TODO --cleanup: string is not sized ...
// impl MemWrite for String {
//     fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//         let cstr = CString::new(self).map_err(|_e| {
//             Error::Mem(super::Error::Unexpected(String::from(
//                 "Null error when constructing CString",
//             )))
//         })?;
//         let bytes = cstr.to_bytes();
//         for byte in bytes {
//             writer.write_u8(*byte)?;
//         }
//         writer.write_u8(0)?; // null terminator
//         Ok(())
//     }
//
//     fn write_total_offset(&self) -> u32 {
//         self.len() as u32 // Always returns number of bytes
//     }
// }

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
