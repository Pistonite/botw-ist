use super::error::Error;
use super::{Reader, Region, RegionType, SimpleHeap};
use crate::memory::util::ByteAble;
// use crate::processor::instruction_parse;
// use crate::processor::instruction_registry::ExecutableInstruction;
use crate::Memory;
use std::ffi::CString;
use std::sync::Arc;

impl Memory {
    /// Generates a memory instance entirely composed of stack memory for simple testing
    pub fn new_empty_mem(size: u32) -> Self {
        // let mem_flags = MemoryFlags {
        //     enable_strict_region: false,
        //     enable_permission_check: false,
        //     enable_allocated_check: false,
        // };
        let p = Arc::new(Region::new_rw(RegionType::Program, size as u64, 0));
        let s = Arc::new(Region::new_rw(RegionType::Stack, 0, size));
        let h = Arc::new(SimpleHeap::new(size as u64, 0, 0));
        Memory::new(p, s, h, None, None, None)
    }

    /// Writes a byte to memory a specific address
    pub fn mem_write_byte(&mut self, address: u64, value: u8) -> Result<(), Error> {
        self.mem_write_val::<u8>(address, value)
    }

    /// Reads a byte from memory at a specific address
    pub fn mem_read_byte(&mut self, address: u64) -> Result<u8, Error> {
        self.mem_read_val::<u8>(address)
    }

    /// Reads N bytes from memory starting at a specific address
    pub fn mem_read_bytes(&mut self, base_address: u64, n: usize) -> Result<Vec<u8>, Error> {
        let mut reader = self.read(base_address, None, false)?;
        let mut r: Vec<u8> = vec![];
        for _ in 0..n {
            r.push(reader.read_u8()?);
        }
        Ok(r)
    }

    /// Given a ByteAble type, writes that object to memory
    pub(crate) fn mem_write_val<T: ByteAble>(
        &mut self,
        base_address: u64,
        val_to_write: T,
    ) -> Result<(), Error> {
        self.mem_write_bytes(base_address, val_to_write.to_bytes())
    }

    /// Given a ByteAble type, reads an instance of that object from memory
    pub(crate) fn mem_read_val<T: ByteAble>(&mut self, base_address: u64) -> Result<T, Error> {
        Ok(T::from_bytes(
            &self.mem_read_bytes(base_address, size_of::<T>())?,
        ))
    }

    /// Given a vector of bytes and an address, writes the bytes into memory
    pub fn mem_write_bytes(
        &mut self,
        base_address: u64,
        bytes_to_write: Vec<u8>,
    ) -> Result<(), Error> {
        let mut writer = self.write(base_address, None)?;
        for b in bytes_to_write.into_iter() {
            writer.write_u8(b)?;
        }
        Ok(())
    }

    /// Reads an i64 from memory at a given address
    pub fn mem_read_i64(&mut self, base_address: u64) -> Result<i64, Error> {
        self.mem_read_val::<i64>(base_address)
    }

    /// Reads an i32 from memory at a given address
    pub fn mem_read_i32(&mut self, base_address: u64) -> Result<i32, Error> {
        self.mem_read_val::<i32>(base_address)
    }

    /// Reads a u16 from memory at a given address
    pub fn mem_read_u16(&mut self, base_address: u64) -> Result<u16, Error> {
        self.mem_read_val::<u16>(base_address)
    }

    /// Reads a u64 from memory at a given address
    pub fn mem_read_u64(&mut self, base_address: u64) -> Result<u64, Error> {
        self.mem_read_val::<u64>(base_address)
    }

    /// Reads an f32 from memory at a given address
    pub fn mem_read_f32(&mut self, base_address: u64) -> Result<f32, Error> {
        self.mem_read_val::<f32>(base_address)
    }

    /// Reads an f64 from memory at a given address
    pub fn mem_read_f64(&mut self, base_address: u64) -> Result<f64, Error> {
        self.mem_read_val::<f64>(base_address)
    }

    /// Writes an i64 to memory at a given address
    pub fn mem_write_i64(&mut self, base_address: u64, write_val: i64) -> Result<(), Error> {
        let temp = write_val.to_le_bytes().to_vec();
        self.mem_write_bytes(base_address, temp)
    }

    /// Writes an i32 to memory at a given address
    pub fn mem_write_i32(&mut self, base_address: u64, write_val: i32) -> Result<(), Error> {
        let temp = write_val.to_le_bytes().to_vec();
        self.mem_write_bytes(base_address, temp)
    }

    /// Writes an f32 to memory at a given address
    pub fn mem_write_f32(&mut self, base_address: u64, write_val: f32) -> Result<(), Error> {
        let temp = write_val.to_le_bytes().to_vec();
        self.mem_write_bytes(base_address, temp)
    }

    /// Writes an f64 to memory at a given address
    pub fn mem_write_f64(&mut self, base_address: u64, write_val: f64) -> Result<(), Error> {
        let temp = write_val.to_le_bytes().to_vec();
        self.mem_write_bytes(base_address, temp)
    }

    /// Writes a cstring to memory
    pub fn mem_write_cstring(
        &mut self,
        base_address: u64,
        val_to_write: CString,
    ) -> Result<(), Error> {
        let byte_vec: Vec<u8> = val_to_write.into_bytes_with_nul();
        self.mem_write_bytes(base_address, byte_vec)
    }

    // /// Reads an instruction object from memory
    // pub fn mem_read_inst(&mut self, address: u64) -> Result<Box<dyn ExecutableInstruction>, Error> {
    //     let mut reader = self.read(address, Some(RegionType::Program.into()), true)?;
    //     reader.read_instruction()
    // }

    /// Checks if a given address is aligned to 4 byte alignment
    pub fn verify_memory_alignment(&mut self, address: u64) -> bool {
        if address % 4 != 0 {
            // println!(
            //     "***CPU NOTE*** Reading misaligned memory at 0x{address:016x}, executing anyway"
            // );
            return true;
        }
        true
    }

    /// Simulates a memcpy calling moving n bytes from src to dst
    pub fn memcpy(&mut self, dst: u64, src: u64, n: usize) -> Result<(), Error> {
        let bytes = self.mem_read_bytes(src, n)?;
        self.mem_write_bytes(dst, bytes)
    }
}

// impl Reader<'_> {
//     pub fn read_instruction(&mut self) -> Result<Box<dyn ExecutableInstruction>, Error> {
//         let inst_bytes: u32 = self.read_u32()?;
//         match instruction_parse::byte_to_inst(inst_bytes) {
//             Ok(inst) => Ok(inst),
//             Err(e) => {
//                 let addr = self.current_addr();
//                 Err(Error::UnexpectedAt(addr, format!("{}", e)))
//             }
//         }
//     }
// }
