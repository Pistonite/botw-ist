use super::{access::{AccessType, MemAccess}, error::Error, page::{Page, PAGE_SIZE}, region::{Region, RegionType}, Memory};

/// Stream writer to memory
pub struct Writer<'m> {
    /// Memory being written to
    ///
    /// Since only one mutable reference can exist in Rust,
    /// we have to get the mutable page reference
    /// for every write operation
    memory: &'m mut Memory,
    /// Current region being written to
    region_type: RegionType,
    /// Index of the page currently being written to
    region_page_idx: u32,
    /// Offset into the current page
    page_off: u32,
}

impl<'m> Writer<'m> {
    pub fn new(memory: &'m mut Memory, region_type: RegionType, region_page_idx: u32, page_off: u32) -> Self {
        Self {
            memory,
            region_type,
            region_page_idx,
            page_off,
        }
    }
    /// Skip `len` bytes in the memory
    pub fn skip(&mut self, len: u32) {
        self.page_off += len;
    }

    fn region(&self) -> &Region {
        self.memory.get_region(self.region_type)
    }

    fn region_mut(&mut self) -> &mut Region {
        self.memory.mut_region(self.region_type)
    }

    /// Get the current reading address
    pub fn current_addr(&self) -> u64 {
        self.region().start + (self.region_page_idx as u64 * PAGE_SIZE as u64) + self.page_off as u64
    }

    /// Write a `u8` to the memory, advance by 1 byte
    #[inline]
    pub fn write_u8(&mut self, val: impl Into<u8>) -> Result<(), Error> {
        self.checked_page_mut(1, |page, off| {
            page.write_u8(off, val.into())
        })
    }

    /// Read a `i8` from the memory, advance by 1 byte
    pub fn write_i8(&mut self, val: impl Into<i8>) -> Result<(), Error> {
        self.write_u8(val.into() as u8)
    }

    /// Write a `u16` to the memory, advance by 2 bytes
    #[inline]
    pub fn write_u16(&mut self, val: impl Into<u16>) -> Result<(), Error> {
        self.checked_page_mut(2, |page, off| {
            page.write_u16(off, val.into())
        })
    }

    /// Write a `i16` to the memory, advance by 2 bytes
    pub fn write_i16(&mut self, val: impl Into<i16>) -> Result<(), Error> {
        self.write_u16(val.into() as u16)
    }

    /// Write a `u32` to the memory, advance by 4 bytes
    #[inline]
    pub fn write_u32(&mut self, val: impl Into<u32>) -> Result<(), Error> {
        self.checked_page_mut(4, |page, off| {
            page.write_u32(off, val.into())
        })
    }

    /// Write a `i32` to the memory, advance by 4 bytes
    pub fn write_i32(&mut self, val: impl Into<i32>) -> Result<(), Error> {
        self.write_u32(val.into() as u32)
    }

    /// Write a `u64` to the memory, advance by 8 bytes
    #[inline]
    pub fn write_u64(&mut self, val: impl Into<u64>) -> Result<(), Error> {
        self.checked_page_mut(8, |page, off| {
            page.write_u64(off, val.into())
        })
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
    fn checked_page_mut<F: FnOnce(&mut Page, u32)>(
        &mut self, len: u32, f:F) -> Result<(), Error> {
        {
            // first check if we are still inside the region
            let region = self.region();
            let current_addr = region.start + (self.region_page_idx as u64 * PAGE_SIZE as u64) + self.page_off as u64;
            if current_addr >= region.get_end() {
                // advance to next region if possible
                match self.memory.get_region_by_addr(current_addr) {
                    None => {
                        return Err(Error::InvalidRegion(current_addr));
                    }
                    Some((region, _, idx, off)) => {
                        // fix up the state
                        self.region_type = region.typ;
                        self.region_page_idx = idx;
                        self.page_off = off;
                    }
                }
            }
        }
        // advance to the next page in the region if needed
        if self.page_off >= PAGE_SIZE {
            self.region_page_idx += self.page_off / PAGE_SIZE;
            self.page_off = self.page_off % PAGE_SIZE;
            if self.region().get(self.region_page_idx).is_none() {
                return Err(Error::Unallocated(self.current_addr()));
            }
        }

        // now check we can actually read `len` bytes at the current address
        if self.memory.flags.enable_allocated_check && self.region().typ == RegionType::Heap {
            let current_addr = self.current_addr();
            if !self.memory.heap.is_allocated(current_addr) {
                return Err(Error::Unallocated(current_addr));
            }
        }
        // copy these value out since we will lose immutable borrow to self
        let region_page_idx = self.region_page_idx;
        let check_permission = self.memory.flags.enable_permission_check;
        let page_off = self.page_off;
        // re-borrow the region as mutable and clone on write
        let region = self.region_mut();
        let page = match region.get_mut(region_page_idx) {
            Some(page) => page,
            None => return Err(Error::Unallocated(self.current_addr())),
        };

        if check_permission && !page.has_permission(AccessType::Write) {
            return Err(Error::PermissionDenied(MemAccess {
                typ: AccessType::Write,
                addr: self.current_addr(),
                bytes: len,
            }));
        } 

        if page_off + len > PAGE_SIZE {
            return Err(Error::PageBoundary(MemAccess {
                typ: AccessType::Write,
                addr: self.current_addr(),
                bytes: len,
            }));
        }

        f(page, page_off);

        self.page_off += len;
        Ok(())
    }
}
