use derive_more::derive::Constructor;

use crate::memory::{AccessFlags, Error, Memory, Page};

/// Stream reader from memory
#[derive(Constructor)]
pub struct Reader<'m> {
    /// Memory being read
    memory: &'m Memory,
    /// The current page being read (so we only need to check memory when crossing page boundary)
    page: &'m Page,

    // note that both page_off and addr are needed,
    // because we need to know when we are crossing page boundary
    /// Current offset into the current page
    page_off: u32,
    /// Maximum page offset this reader can read to (exclusive)
    /// until it has to ask the memory again to check stuff
    max_page_off: u32,
    /// Current physical address being read
    addr: u64,

    flags: AccessFlags,
}

macro_rules! trace {
    (bool, $addr:expr, $addr_str:expr, $value:expr) => {{
        #[cfg(feature = "trace-memory")]
        {
            record_read($addr);
        }
        blueflame_deps::trace_memory!(
            "ld1  {} =>{}",
            $addr_str,
            if $value { "true" } else { "false" }
        );
    }};
    ($len:expr, $addr:expr, $addr_str:expr, $value:expr, $width:literal) => {{
        #[cfg(feature = "trace-memory")]
        {
            record_read($addr);
        }
        blueflame_deps::trace_memory!(
            concat!("ld{:<2} {} =>0x{:0", $width, "x}"),
            $len * 8,
            $addr_str,
            $value
        );
    }};
}

impl<'m> Reader<'m> {
    /// Skip `len` bytes in the memory
    #[inline]
    pub fn skip(&mut self, len: u32) {
        self.page_off += len;
        self.addr += len as u64;
        // checks are done in prep_read, the next time a read is performed
        // this is so that if we read the last data and advance into
        // invalid memory, no exception will be thrown
    }

    /// Read a `bool` from the memory, advance by 1 byte
    pub fn read_bool<T: From<bool>>(&mut self) -> Result<T, Error> {
        let val = match self.prep_read(1) {
            Ok(_) => self.page.read_u8(self.page_off) & 1 == 1,
            Err(Error::Bypassed) => false,
            Err(e) => return Err(e),
        };
        trace!(bool, self.addr, self.memory.format_addr(self.addr), val);
        self.skip(1);

        Ok(val.into())
    }

    /// Read a `u8` from the memory, advance by 1 byte
    pub fn read_u8<T: From<u8>>(&mut self) -> Result<T, Error> {
        let val = match self.prep_read(1) {
            Ok(_) => self.page.read_u8(self.page_off),
            Err(Error::Bypassed) => 0,
            Err(e) => return Err(e),
        };
        trace!(1, self.addr, self.memory.format_addr(self.addr), val, 2);
        self.skip(1);

        Ok(val.into())
    }

    /// Read a `i8` from the memory, advance by 1 byte
    pub fn read_i8<T: From<i8>>(&mut self) -> Result<T, Error> {
        let val: u8 = self.read_u8()?;
        Ok((val as i8).into())
    }

    /// Read a `u16` from the memory, advance by 2 bytes
    pub fn read_u16<T: From<u16>>(&mut self) -> Result<T, Error> {
        let val = match self.prep_read(2) {
            Ok(_) => self.page.read_u16(self.page_off),
            Err(Error::Bypassed) => 0,
            Err(e) => return Err(e),
        };
        trace!(2, self.addr, self.memory.format_addr(self.addr), val, 4);
        self.skip(2);

        Ok(val.into())
    }

    /// Read a `i16` from the memory, advance by 2 bytes
    pub fn read_i16<T: From<i16>>(&mut self) -> Result<T, Error> {
        let val: u16 = self.read_u16()?;
        Ok((val as i16).into())
    }

    /// Read a `u32` from the memory, advance by 4 bytes
    pub fn read_u32<T: From<u32>>(&mut self) -> Result<T, Error> {
        let val = match self.prep_read(4) {
            Ok(_) => self.page.read_u32(self.page_off),
            Err(Error::Bypassed) => 0,
            Err(e) => return Err(e),
        };
        trace!(4, self.addr, self.memory.format_addr(self.addr), val, 8);
        self.skip(4);

        Ok(val.into())
    }

    /// Read a `i32` from the memory, advance by 4 bytes
    pub fn read_i32<T: From<i32>>(&mut self) -> Result<T, Error> {
        let val: u32 = self.read_u32()?;
        Ok((val as i32).into())
    }

    /// Read a `u64` from the memory, advance by 8 bytes
    pub fn read_u64<T: From<u64>>(&mut self) -> Result<T, Error> {
        let val = match self.prep_read(8) {
            Ok(_) => self.page.read_u64(self.page_off),
            Err(Error::Bypassed) => 0,
            Err(e) => return Err(e),
        };
        trace!(8, self.addr, self.memory.format_addr(self.addr), val, 16);
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
        // are we still on the same page?
        if self.page_off >= self.max_page_off {
            // query the memory for the next page
            let (section_idx, page_idx, page_off, max_page_off) =
                self.memory.calculate(self.addr, self.flags)?;
            self.page = self.memory.page_by_indices_unchecked(section_idx, page_idx);
            self.page_off = page_off;
            self.max_page_off = max_page_off;
        }

        if self.page_off + len > self.max_page_off {
            log::error!(
                "boundary hit at {}, reading {len} bytes",
                self.memory.format_addr(self.addr)
            );
            return Err(Error::Boundary(self.addr, self.flags));
        }

        Ok(())
    }
}

#[cfg(feature = "trace-memory")]
mod trace_read_impl {
    use crate::memory::PAGE_SIZE;
    use std::cell::RefCell;
    use std::collections::BTreeSet;
    use std::sync::{Arc, LazyLock, Mutex};
    static GLOBAL_TRACE: LazyLock<Arc<Mutex<BTreeSet<u64>>>> =
        LazyLock::new(|| Arc::new(Mutex::new(Default::default())));
    thread_local! {
        static LOCAL_TRACE: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    }
    pub fn record_read(addr: u64) {
        let page = addr / (PAGE_SIZE as u64);
        if cfg!(feature = "trace-memory-no-auto-commit") {
            LOCAL_TRACE.with_borrow_mut(|x| x.push(page))
        } else {
            LazyLock::force(&GLOBAL_TRACE).lock().unwrap().insert(page);
        }
    }
    pub fn commit_read_trace() {
        #[cfg(feature = "trace-memory-no-auto-commit")]
        {
            LOCAL_TRACE.with_borrow_mut(|x| {
                let mut global = LazyLock::force(&GLOBAL_TRACE).lock().unwrap();
                global.extend(std::mem::take(x))
            });
        }
    }
    pub fn get_read_page_ranges() -> Vec<(u64, u64)> {
        let mut ranges = Vec::new();
        {
            let pages = LazyLock::force(&GLOBAL_TRACE).lock().unwrap();
            for addr in pages.iter().copied() {
                let Some(range) = ranges.last_mut() else {
                    ranges.push((addr * PAGE_SIZE as u64, (addr + 1) * PAGE_SIZE as u64));
                    continue;
                };
                let start = addr * PAGE_SIZE as u64;
                let end = (addr + 1) * PAGE_SIZE as u64;
                if start == range.1 {
                    range.1 = end;
                    continue;
                }
                ranges.push((start, end));
            }
        };

        ranges
    }
}
#[cfg(feature = "trace-memory")]
pub use trace_read_impl::*;
