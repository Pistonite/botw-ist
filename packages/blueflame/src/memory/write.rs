use derive_more::derive::Constructor;

use crate::memory::{AccessFlags, Error, Memory};

/// Stream writer to memory
#[derive(Constructor)]
pub struct Writer<'m> {
    /// Memory being written to
    ///
    /// Since only one mutable reference can exist in Rust,
    /// we have to get the mutable page reference
    /// for every write operation
    memory: &'m mut Memory,

    // we have to do 2 indirection for every write which is unfortunate,
    // this is due to Rust's safety measures
    // 1. use section_idx to mutably borrow the section in memory
    // 2. use page_idx to mutably borrow the page in that section
    /// Current region being written to
    section_idx: u32,
    /// Index of the page currently being written to
    page_idx: u32,
    /// Offset into the current page
    page_off: u32,
    max_page_off: u32,

    addr: u64,
    flags: AccessFlags,
}

macro_rules! trace {
    (bool, $addr_str:expr, $value:expr) => {{
        blueflame_deps::trace_memory!(
            "st1  {}<= {}",
            $addr_str,
            if $value { "true" } else { "false" }
        );
    }};
    ($len:expr, $addr_str:expr, $value:expr, $width:literal) => {{
        blueflame_deps::trace_memory!(
            concat!("st{:<2} {}<= 0x{:0", $width, "x}"),
            $len * 8,
            $addr_str,
            $value
        );
    }};
}

impl<'m> Writer<'m> {
    /// Skip `len` bytes in the memory
    pub fn skip(&mut self, len: u32) {
        self.page_off += len;
        self.addr += len as u64;
    }

    /// Write a `bool` to the memory, advance by 1 byte
    pub fn write_bool(&mut self, val: impl Into<bool>) -> Result<(), Error> {
        match self.prep_write(1) {
            Ok(_) => {}
            Err(Error::Bypassed) => return Ok(()),
            Err(e) => return Err(e),
        };
        let val: bool = val.into();
        trace!(bool, self.memory.format_addr(self.addr), val);
        let page = self
            .memory
            .page_by_indices_mut_unchecked(self.section_idx, self.page_idx);
        page.write_u8(self.page_off, if val { 1 } else { 0 });
        self.skip(1);
        Ok(())
    }

    /// Write a `u8` to the memory, advance by 1 byte
    pub fn write_u8(&mut self, val: impl Into<u8>) -> Result<(), Error> {
        match self.prep_write(1) {
            Ok(_) => {}
            Err(Error::Bypassed) => return Ok(()),
            Err(e) => return Err(e),
        };
        let val: u8 = val.into();
        trace!(1, self.memory.format_addr(self.addr), val, 2);
        let page = self
            .memory
            .page_by_indices_mut_unchecked(self.section_idx, self.page_idx);
        page.write_u8(self.page_off, val);
        self.skip(1);
        Ok(())
    }

    /// Read a `i8` from the memory, advance by 1 byte
    pub fn write_i8(&mut self, val: impl Into<i8>) -> Result<(), Error> {
        self.write_u8(val.into() as u8)
    }

    /// Write a `u16` to the memory, advance by 2 bytes
    pub fn write_u16(&mut self, val: impl Into<u16>) -> Result<(), Error> {
        match self.prep_write(2) {
            Ok(_) => {}
            Err(Error::Bypassed) => return Ok(()),
            Err(e) => return Err(e),
        };
        let val: u16 = val.into();
        trace!(2, self.memory.format_addr(self.addr), val, 4);
        let page = self
            .memory
            .page_by_indices_mut_unchecked(self.section_idx, self.page_idx);
        page.write_u16(self.page_off, val);
        self.skip(2);
        Ok(())
    }

    /// Write a `i16` to the memory, advance by 2 bytes
    pub fn write_i16(&mut self, val: impl Into<i16>) -> Result<(), Error> {
        self.write_u16(val.into() as u16)
    }

    /// Write a `u32` to the memory, advance by 4 bytes
    pub fn write_u32(&mut self, val: impl Into<u32>) -> Result<(), Error> {
        match self.prep_write(4) {
            Ok(_) => {}
            Err(Error::Bypassed) => return Ok(()),
            Err(e) => return Err(e),
        };
        let val: u32 = val.into();
        trace!(4, self.memory.format_addr(self.addr), val, 8);
        let page = self
            .memory
            .page_by_indices_mut_unchecked(self.section_idx, self.page_idx);
        page.write_u32(self.page_off, val);
        self.skip(4);
        Ok(())
    }

    /// Write a `i32` to the memory, advance by 4 bytes
    pub fn write_i32(&mut self, val: impl Into<i32>) -> Result<(), Error> {
        self.write_u32(val.into() as u32)
    }

    /// Write a `u64` to the memory, advance by 8 bytes
    pub fn write_u64(&mut self, val: impl Into<u64>) -> Result<(), Error> {
        match self.prep_write(8) {
            Ok(_) => {}
            Err(Error::Bypassed) => return Ok(()),
            Err(e) => return Err(e),
        };
        let val: u64 = val.into();
        trace!(8, self.memory.format_addr(self.addr), val, 16);
        let page = self
            .memory
            .page_by_indices_mut_unchecked(self.section_idx, self.page_idx);
        page.write_u64(self.page_off, val);
        self.skip(8);
        Ok(())
    }

    /// Write a `i64` to the memory, advance by 8 bytes
    pub fn write_i64(&mut self, val: impl Into<i64>) -> Result<(), Error> {
        self.write_u64(val.into() as u64)
    }

    /// Write a `f32` to the memory, advance by 4 bytes
    pub fn write_f32(&mut self, val: impl Into<f32>) -> Result<(), Error> {
        let val: u32 = val.into().to_bits();
        self.write_u32(val)
    }

    /// Write a `f64` to the memory, advance by 8 bytes
    pub fn write_f64(&mut self, val: impl Into<f64>) -> Result<(), Error> {
        let val: u64 = val.into().to_bits();
        self.write_u64(val)
    }

    /// Prepare a write, then operate on the page
    ///
    /// This must be done through a FnOnce closure because of borrowing rules
    fn prep_write(&mut self, len: u32) -> Result<(), Error> {
        if self.page_off >= self.max_page_off {
            // query the memory for the next page
            let (section_idx, page_idx, page_off, max_page_off) =
                self.memory.calculate(self.addr, self.flags)?;
            self.section_idx = section_idx;
            self.page_idx = page_idx;
            self.page_off = page_off;
            self.max_page_off = max_page_off;
        }

        if self.page_off + len > self.max_page_off {
            log::error!(
                "boundary hit at {}, writing {len} bytes",
                self.memory.format_addr(self.addr)
            );
            return Err(Error::Boundary(self.addr, self.flags));
        }
        Ok(())
    }
}
