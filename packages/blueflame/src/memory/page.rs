#[layered_crate::import]
use memory::PAGE_SIZE;

/// A page in emulated memory is a simple container of bytes
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Page {
    data: [u8; PAGE_SIZE as usize],
}

impl Page {
    /// Create a new page with zeroed data
    pub fn zeroed() -> Self {
        Self {
            data: [0; PAGE_SIZE as usize],
        }
    }

    /// Create a new page with the given data padded with zeros to the page size
    pub fn from_slice(data: &[u8]) -> Self {
        // TODO --optimize: how much boot perf can be get if we uninit this?
        let mut page = Self::zeroed();
        page.data[..data.len()].copy_from_slice(data);
        page
    }

    /// Read a u8 at offset without checking permissions
    #[inline(always)]
    pub fn read_u8(&self, off: u32) -> u8 {
        self.data[off as usize]
    }

    /// Write a u8 at offset without checking permissions
    #[inline(always)]
    pub fn write_u8(&mut self, off: u32, val: u8) {
        self.data[off as usize] = val;
    }

    /// Read a u16 at offset without checking permissions or bounds
    #[inline(always)]
    pub fn read_u16(&self, off: u32) -> u16 {
        u16::from_le_bytes([self.data[off as usize], self.data[off as usize + 1]])
    }

    /// Write a u16 at offset without checking permissions or bounds
    #[inline(always)]
    pub fn write_u16(&mut self, off: u32, val: u16) {
        for (i, b) in val.to_le_bytes().into_iter().enumerate() {
            self.data[off as usize + i] = b;
        }
    }

    /// Read a u32 at offset without checking permissions or bounds
    #[inline(always)]
    pub fn read_u32(&self, off: u32) -> u32 {
        u32::from_le_bytes([
            self.data[off as usize],
            self.data[off as usize + 1],
            self.data[off as usize + 2],
            self.data[off as usize + 3],
        ])
    }

    /// Write a u32 at offset without checking permissions or bounds
    #[inline(always)]
    pub fn write_u32(&mut self, off: u32, val: u32) {
        for (i, b) in val.to_le_bytes().into_iter().enumerate() {
            self.data[off as usize + i] = b;
        }
    }

    /// Read a u64 at offset without checking permissions or bounds
    #[inline(always)]
    pub fn read_u64(&self, off: u32) -> u64 {
        u64::from_le_bytes([
            self.data[off as usize],
            self.data[off as usize + 1],
            self.data[off as usize + 2],
            self.data[off as usize + 3],
            self.data[off as usize + 4],
            self.data[off as usize + 5],
            self.data[off as usize + 6],
            self.data[off as usize + 7],
        ])
    }

    /// Write a u64 at offset without checking permissions or bounds
    #[inline(always)]
    pub fn write_u64(&mut self, off: u32, val: u64) {
        for (i, b) in val.to_le_bytes().into_iter().enumerate() {
            self.data[off as usize + i] = b;
        }
    }
}
