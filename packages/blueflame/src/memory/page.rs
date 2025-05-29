use enumset::EnumSet;

#[layered_crate::import]
use memory::AccessType;

const PAGE_SIZE: u32 = 0x1000;

/// A page in memory
#[derive(Debug, Clone)]
pub struct Page {
    data: [u8; PAGE_SIZE as usize],
    perm: EnumSet<AccessType>,
}

impl Page {
    /// Create a new page with the given permissions and zeroed data
    pub fn zeroed(perm: EnumSet<AccessType>) -> Self {
        Self {
            data: [0; PAGE_SIZE as usize],
            perm,
        }
    }

    pub fn from_slice(data: &[u8], perm: EnumSet<AccessType>) -> Self {
        let mut page = Self::zeroed(perm);
        page.data[..data.len()].copy_from_slice(data);
        page
    }

    /// Check if the page has all of the given permission set
    pub fn has_permission(&self, perm: impl Into<EnumSet<AccessType>>) -> bool {
        !self.perm.intersection(perm.into()).is_empty()
    }

    /// Read a u8 at offset without checking permissions
    #[inline]
    pub fn read_u8(&self, off: u32) -> u8 {
        self.data[off as usize]
    }

    /// Write a u8 at offset without checking permissions
    #[inline]
    pub fn write_u8(&mut self, off: u32, val: u8) {
        self.data[off as usize] = val;
    }

    /// Read a u16 at offset without checking permissions or bounds
    #[inline]
    pub fn read_u16(&self, off: u32) -> u16 {
        u16::from_le_bytes([self.data[off as usize], self.data[off as usize + 1]])
    }

    /// Write a u16 at offset without checking permissions or bounds
    #[inline]
    pub fn write_u16(&mut self, off: u32, val: u16) {
        for (i, b) in val.to_le_bytes().into_iter().enumerate() {
            self.data[off as usize + i] = b;
        }
    }

    /// Read a u32 at offset without checking permissions or bounds
    #[inline]
    pub fn read_u32(&self, off: u32) -> u32 {
        u32::from_le_bytes([
            self.data[off as usize],
            self.data[off as usize + 1],
            self.data[off as usize + 2],
            self.data[off as usize + 3],
        ])
    }

    /// Write a u32 at offset without checking permissions or bounds
    #[inline]
    pub fn write_u32(&mut self, off: u32, val: u32) {
        for (i, b) in val.to_le_bytes().into_iter().enumerate() {
            self.data[off as usize + i] = b;
        }
    }

    /// Read a u64 at offset without checking permissions or bounds
    #[inline]
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
    #[inline]
    pub fn write_u64(&mut self, off: u32, val: u64) {
        for (i, b) in val.to_le_bytes().into_iter().enumerate() {
            self.data[off as usize + i] = b;
        }
    }
}
