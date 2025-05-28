use derive_more::derive::Constructor;

#[layered_crate::import]
use memory::{
    super::env::enabled,
    self::{AccessType, MemAccess, Error, Page, Region, RegionType, Memory, PAGE_SIZE, glue}
};

/// Stream reader from memory
#[derive(Constructor)]
pub struct Reader<'m> {
    /// Memory being read
    memory: &'m Memory,
    /// The current region being read
    region: &'m Region,
    /// The current page being read
    page: &'m Page,
    /// Index of the page in the current region
    region_page_idx: u32,
    /// Current offset into the current page
    page_off: u32,
    /// If the read is for execution, so the execute permission is checked
    execute: bool,
    /// Disable RX permission check
    disable_permission_check: bool,

    /// Offset to subtract while tracing
    ///
    /// This is mainly used for tracing `main` module, since
    /// the start of program isn't the start of the main module
    trace_offset: u64,
    // ^ TODO --cleanup: this isn't needed after region refactoring
}

macro_rules! trace {
    (bool, $tag:expr, $addr:expr, $start:expr, $value:expr) => { {
            blueflame_deps::trace_memory!(concat!("ld1  {}+0x{:08x} =>{}"), $tag, {$addr} - {$start}, if $value { "true" } else { "false" });
        } };
    ($len:expr, $tag:expr, $addr:expr, $start:expr, $value:expr, $width:literal) => { {
            blueflame_deps::trace_memory!(concat!("ld{:2} {}+0x{:08x} =>0x{:0", $width, "x}"), $len * 8, $tag, {$addr} - {$start}, $value);
        } };
    ($len:expr, $tag:expr, $addr:expr, $start:expr, $value:literal) => { {
            blueflame_deps::trace_memory!(concat!("ld{:2} {}+0x{:08x} =>", $value), $len * 8, $tag, {$addr} - {$start});
        } };
}

impl<'m> Reader<'m> {

    /// Skip `len` bytes in the memory
    #[inline]
    pub fn skip(&mut self, len: u32) {
        self.page_off += len;
        // checks are done in prep_read, the next time a read is performed
        // this is so that if we read the last data and advance into
        // invalid memory, no exception will be thrown
    }

    /// Get the current reading physical address
    pub fn current_addr(&self) -> u64 {
        self.region.start + (self.region_page_idx as u64 * PAGE_SIZE as u64) + self.page_off as u64
    }

    /// Read a `bool` from the memory, advance by 1 byte
    #[inline]
    pub fn read_bool<T: From<bool>>(&mut self) -> Result<T, Error> {
        self.prep_read(1)?;
        let val = self.page.read_u8(self.page_off) & 1 == 1;
        trace!(bool, self.region.tag, self.current_addr(), self.region.start + self.trace_offset, val);
        self.skip(1);

        Ok(val.into())
    }

    /// Read a `u8` from the memory, advance by 1 byte
    #[inline]
    pub fn read_u8<T: From<u8>>(&mut self) -> Result<T, Error> {
        self.prep_read(1)?;
        let val = self.page.read_u8(self.page_off);
        trace!(1, self.region.tag, self.current_addr(), self.region.start + self.trace_offset, val, 2);
        self.skip(1);

        Ok(val.into())
    }

    /// Read a `i8` from the memory, advance by 1 byte
    pub fn read_i8<T: From<i8>>(&mut self) -> Result<T, Error> {
        let val: u8 = self.read_u8()?;
        Ok((val as i8).into())
    }

    /// Read a `u16` from the memory, advance by 2 bytes
    #[inline]
    pub fn read_u16<T: From<u16>>(&mut self) -> Result<T, Error> {
        self.prep_read(2)?;
        let val = self.page.read_u16(self.page_off);
        trace!(2, self.region.tag, self.current_addr(), self.region.start + self.trace_offset, val, 4);
        self.skip(2);

        Ok(val.into())
    }

    /// Read a `i16` from the memory, advance by 2 bytes
    pub fn read_i16<T: From<i16>>(&mut self) -> Result<T, Error> {
        let val: u16 = self.read_u16()?;
        Ok((val as i16).into())
    }

    /// Read a `u32` from the memory, advance by 4 bytes
    #[inline]
    pub fn read_u32<T: From<u32>>(&mut self) -> Result<T, Error> {
        self.prep_read(4)?;
        let val = self.page.read_u32(self.page_off);
        trace!(4, self.region.tag, self.current_addr(), self.region.start + self.trace_offset, val, 8);
        self.skip(4);

        Ok(val.into())
    }

    /// Read a `i32` from the memory, advance by 4 bytes
    pub fn read_i32<T: From<i32>>(&mut self) -> Result<T, Error> {
        let val: u32 = self.read_u32()?;
        Ok((val as i32).into())
    }

    /// Read a `u64` from the memory, advance by 8 bytes
    #[inline]
    pub fn read_u64<T: From<u64>>(&mut self) -> Result<T, Error> {
        self.prep_read(8)?;
        let val = self.page.read_u64(self.page_off);
        trace!(8, self.region.tag, self.current_addr(), self.region.start + self.trace_offset, val, 16);
        self.skip(8);

        Ok(val.into())
    }

    /// Read a `i64` from the memory, advance by 8 bytes
    pub fn read_i64<T: From<i64>>(&mut self) -> Result<T, Error> {
        let val: u64 = self.read_u64()?;
        Ok((val as i64).into())
    }

    /// Read a `f32` from the memory, advance by 4 bytes
    pub fn read_f32<T: From<f32>>(&mut self) -> Result<T, Error> {
        let val: u32 = self.read_u32()?;
        Ok(f32::from_bits(val).into())
    }

    /// Read a `f64` from the memory, advance by 8 bytes
    pub fn read_f64<T: From<f64>>(&mut self) -> Result<T, Error> {
        let val: u64 = self.read_u64()?;
        Ok(f64::from_bits(val).into())
    }

    /// Prepare a read
    ///
    /// First it will make sure the region and page reference are valid,
    /// then, it will check if the read is allowed
    fn prep_read(&mut self, len: u32) -> Result<(), Error> {
        // first check if we are still inside the region
        let current_addr = self.current_addr();
        if current_addr >= self.region.get_end() {
            // advance to next region if possible
            match self.memory.get_region_by_addr(current_addr) {
                None => {
                    trace!(len, "??", current_addr, 0, "invalid region");
                    return Err(Error::InvalidRegion(current_addr));
                }
                Some((region, page, idx, off)) => {
                    // fix up the state
                    self.region = region;
                    self.page = page;
                    self.region_page_idx = idx;
                    self.page_off = off;
                }
            }
        }
        // advance to the next page in the region if needed
        if self.page_off >= PAGE_SIZE {
            self.region_page_idx += self.page_off / PAGE_SIZE;
            self.page_off %= PAGE_SIZE;
            self.page = match self.region.get(self.region_page_idx) {
                Some(page) => page.as_ref(),
                None => {
                    trace!(len, self.region.tag, current_addr, self.region.start + self.trace_offset, "unallocated");
                    return Err(Error::Unallocated(current_addr));
                }
            }
        }

        // now check we can actually read `len` bytes at the current address
        if self.region.typ == RegionType::Heap && enabled!("mem-heap-check-allocated")
            && !self.memory.heap.is_allocated(current_addr)
        {
            trace!(len, self.region.tag, current_addr, self.region.start + self.trace_offset, "heap unallocated");
            return Err(Error::Unallocated(current_addr));
        }

        if !self.disable_permission_check && enabled!("mem-permission") {
            if self.execute {
                if !self
                    .page
                    .has_permission(AccessType::Read | AccessType::Execute)
                {
                    trace!(len, self.region.tag, current_addr, self.region.start + self.trace_offset, "permission denied (rx)");
                    return Err(Error::PermissionDenied(MemAccess {
                        flags: glue::access_type_to_flags(AccessType::Execute),
                        addr: self.current_addr(),
                        bytes: len,
                    }));
                }
            } else if !self.page.has_permission(AccessType::Read) {
                trace!(len, self.region.tag, current_addr, self.region.start + self.trace_offset, "permission denied (r)");
                return Err(Error::PermissionDenied(MemAccess {
                    flags: glue::access_type_to_flags(AccessType::Read),
                    addr: self.current_addr(),
                    bytes: len,
                }));
            }
        }

        if self.page_off + len > PAGE_SIZE {
            trace!(len, self.region.tag, current_addr, self.region.start + self.trace_offset, "page align fault");
            return Err(Error::PageBoundary(MemAccess {
                flags: glue::access_type_to_flags(AccessType::Read),
                addr: self.current_addr(),
                bytes: len,
            }));
        }

        Ok(())
    }

}
