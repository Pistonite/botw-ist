use derive_more::derive::Constructor;

#[layered_crate::import]
use memory::{
    super::env::enabled,
    self::{AccessType, MemAccess, Error, Page, PAGE_SIZE, Region, RegionType, Memory, glue}
};


/// Stream writer to memory
#[derive(Constructor)]
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

    /// Disable W permission check
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
            blueflame_deps::trace_memory!(concat!("st1  {}+0x{:08x}<= {}"), $tag, {$addr} - {$start}, if $value { "true" } else { "false" });
        } };
    ($len:expr, $tag:expr, $addr:expr, $start:expr, $value:expr, $width:literal) => { {
            blueflame_deps::trace_memory!(concat!("st{:2} {}+0x{:08x}<= 0x{:0", $width, "x}"), $len * 8, $tag, {$addr} - {$start}, $value);
        } };
    ($len:expr, $tag:expr, $addr:expr, $start:expr, $value:literal) => { {
            blueflame_deps::trace_memory!(concat!("st{:2} {}+0x{:08x}<= ", $value), $len * 8, $tag, {$addr} - {$start});
        } };
}

impl<'m> Writer<'m> {
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
        self.region().start
            + (self.region_page_idx as u64 * PAGE_SIZE as u64)
            + self.page_off as u64
    }

    pub fn trace_start_addr(&self) -> u64 {
        self.region().start + self.trace_offset
    }

    /// Write a `bool` to the memory, advance by 1 byte
    #[inline]
    pub fn write_bool(&mut self, val: impl Into<bool>) -> Result<(), Error> {
        self.checked_page_mut(1, |page, off, tag, tstart, addr| {
            let val: bool = val.into();
            trace!(bool, tag, addr,tstart, val);
            page.write_u8(off, if val { 1 } else { 0 })
        })
    }

    /// Write a `u8` to the memory, advance by 1 byte
    #[inline]
    pub fn write_u8(&mut self, val: impl Into<u8>) -> Result<(), Error> {
        self.checked_page_mut(1, |page, off, tag, tstart, addr| {
            let val: u8 = val.into();
            trace!(1, tag, addr, tstart, val, 2);
            page.write_u8(off, val)
        })
    }

    /// Read a `i8` from the memory, advance by 1 byte
    pub fn write_i8(&mut self, val: impl Into<i8>) -> Result<(), Error> {
        self.write_u8(val.into() as u8)
    }

    /// Write a `u16` to the memory, advance by 2 bytes
    #[inline]
    pub fn write_u16(&mut self, val: impl Into<u16>) -> Result<(), Error> {
        self.checked_page_mut(2, |page, off, tag, tstart, addr| {
            let val: u16 = val.into();
            trace!(2, tag, addr, tstart, val, 4);
            page.write_u16(off, val)
        })
    }

    /// Write a `i16` to the memory, advance by 2 bytes
    pub fn write_i16(&mut self, val: impl Into<i16>) -> Result<(), Error> {
        self.write_u16(val.into() as u16)
    }

    /// Write a `u32` to the memory, advance by 4 bytes
    #[inline]
    pub fn write_u32(&mut self, val: impl Into<u32>) -> Result<(), Error> {
        self.checked_page_mut(4, |page, off, tag, tstart, addr| { 
            let val: u32 = val.into();
            trace!(4, tag, addr, tstart, val, 8);
            page.write_u32(off, val) 
        })
    }

    /// Write a `i32` to the memory, advance by 4 bytes
    pub fn write_i32(&mut self, val: impl Into<i32>) -> Result<(), Error> {
        self.write_u32(val.into() as u32)
    }

    /// Write a `u64` to the memory, advance by 8 bytes
    #[inline]
    pub fn write_u64(&mut self, val: impl Into<u64>) -> Result<(), Error> {
        self.checked_page_mut(8, |page, off, tag, tstart, addr| {
            let val: u64 = val.into();
            trace!(8, tag, addr, tstart, val, 16);
            page.write_u64(off, val)
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
    fn checked_page_mut<F: FnOnce(&mut Page, u32, &'static str, u64, u64)>(&mut self, len: u32, f: F) -> Result<(), Error> {
        {
            // first check if we are still inside the region
            let region = self.region();
            let current_addr = region.start
                + (self.region_page_idx as u64 * PAGE_SIZE as u64)
                + self.page_off as u64;
            if current_addr >= region.get_end() {
                // advance to next region if possible
                match self.memory.get_region_by_addr(current_addr) {
                    None => {
                        trace!(len, "??", current_addr, 0, "invalid region");
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
            self.page_off %= PAGE_SIZE;
            let region = self.region();
            if region.get(self.region_page_idx).is_none() {
                let current_addr = self.current_addr();
                trace!(len, region.tag, current_addr, self.trace_start_addr(), "unallocated");
                return Err(Error::Unallocated(current_addr));
            }
        }

        // now check we can actually read `len` bytes at the current address
        if enabled!("mem-heap-check-allocated") && self.region().typ == RegionType::Heap {
            let current_addr = self.current_addr();
            if !self.memory.heap.is_allocated(current_addr) {
                trace!(len, self.region().tag, current_addr, self.trace_start_addr(), "heap unallocated");
                return Err(Error::Unallocated(current_addr));
            }
        }
        // copy these value out since we will lose immutable borrow to self
        let region_page_idx = self.region_page_idx;
        let page_off = self.page_off;
        let permission_check = !self.disable_permission_check;
        let trace_offset = self.trace_offset;
        // re-borrow the region as mutable and clone on write
        let region = self.region_mut();
        let region_tag = region.tag;
        let region_start = region.start;
        let trace_start = region.start + trace_offset;
        let Some(page) = region.get_mut(region_page_idx) else {
            let current_addr = self.current_addr();
            trace!(len, region_tag, current_addr, trace_start, "unallocated");
            return Err(Error::Unallocated(current_addr));
        };

        if permission_check && !page.has_permission(AccessType::Write) && enabled!("mem-permission"){
            let current_addr = self.current_addr();
            trace!(len, region_tag, current_addr, self.trace_start_addr(), "permission denied (w)");
            return Err(Error::PermissionDenied(MemAccess {
                flags: glue::access_type_to_flags(AccessType::Write),
                addr: current_addr,
                bytes: len,
            }));
        }

        if page_off + len > PAGE_SIZE {
            let current_addr = self.current_addr();
            trace!(len, region_tag, current_addr, self.trace_start_addr(), "page align fault");
            return Err(Error::PageBoundary(MemAccess {
                flags: glue::access_type_to_flags(AccessType::Write),
                addr: current_addr,
                bytes: len,
            }));
        }

        let current_addr = region_start + (region_page_idx as u64 * PAGE_SIZE as u64) + page_off as u64;
        f(page, page_off, region_tag, trace_start, current_addr);

        self.page_off += len;
        Ok(())
    }
}
