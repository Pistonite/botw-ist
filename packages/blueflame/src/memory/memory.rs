use std::ops::DerefMut;
use std::sync::Arc;

use derive_more::derive::Constructor;
use enumset::EnumSet;

use super::error::Error;
use super::heap::SimpleHeap;
use super::page::Page;
use super::read::Reader;
use super::region::{Region, RegionType};
use super::write::Writer;

/// Memory of the simulated process
#[derive(Debug, Clone, Constructor)]
pub struct Memory {
    /// Memory feature flags
    pub flags: MemoryFlags,
    /// program region
    program: Arc<Region>,
    /// stack region
    stack: Arc<Region>,
    /// heap region
    pub heap: Arc<SimpleHeap>,
}

impl Memory {
    /// Create a reader to start reading at address
    ///
    /// Only allow reading from certain regions if `region` is specified
    pub fn read(&self, address: u64, region: Option<EnumSet<RegionType>>, execute: bool) -> Result<Reader, Error> {
        let regions = if self.flags.enable_strict_region {
            region.unwrap_or(EnumSet::all())
        } else {
            EnumSet::all()
        };
        if let Some((region, page, page_idx, off)) = self.get_region_by_addr(address) {
            if !regions.contains(region.typ) {
                return Err(Error::DisallowedRegion(address, regions));
            }
            return Ok(Reader::new(self, region, page, page_idx, off, execute));
        }
        // region read_by_addr will fail if the address is not allocated,
        // in those cases, we want to return Unallocated error
        if self.program.is_addr_in_region(address) {
            return Err(Error::Unallocated(address))
        }
        if self.stack.is_addr_in_region(address) {
            return Err(Error::Unallocated(address))
        }
        if self.heap.is_addr_in_region(address) {
            return Err(Error::Unallocated(address))
        }
        Err(Error::InvalidRegion(address))
    }

    /// Create a writer to start writing at address
    ///
    /// Only allow writing to certain regions if `region` is specified
    pub fn write(&mut self, address: u64, region: Option<EnumSet<RegionType>>) -> Result<Writer, Error> {
        let regions = if self.flags.enable_strict_region {
            region.unwrap_or(EnumSet::all())
        } else {
            EnumSet::all()
        };
        if let Some((region, _, page_idx, off)) = self.get_region_by_addr(address) {
            if !regions.contains(region.typ) {
                return Err(Error::DisallowedRegion(address, regions));
            }
            return Ok(Writer::new(self, region.typ, page_idx, off));
        }
        // region read_by_addr will fail if the address is not allocated,
        // in those cases, we want to return Unallocated error
        if self.program.is_addr_in_region(address) {
            return Err(Error::Unallocated(address))
        }
        if self.stack.is_addr_in_region(address) {
            return Err(Error::Unallocated(address))
        }
        if self.heap.is_addr_in_region(address) {
            return Err(Error::Unallocated(address))
        }
        Err(Error::InvalidRegion(address))

    }

    /// If `address` is inside a region, return the region, page, region page index, and page offset
    ///
    /// This does not check permissions or if the address
    /// is in an unallocated part of the region
    pub fn get_region_by_addr(&self, address: u64) -> Option<(&Region, &Page, u32, u32)> {
        if let Some((page, idx, off)) = self.program.read_at_addr(address) {
            return Some((self.program.as_ref(), page, idx, off));
        }
        if let Some((page, idx, off)) = self.stack.read_at_addr(address) {
            return Some((self.stack.as_ref(), page, idx, off));
        }
        if let Some((page, idx, off)) = self.heap.read_at_addr(address) {
            return Some((self.heap.as_ref(), page, idx, off));
        }
        None
    }

    // // remove this if not needed
    // pub fn get_cloest_next_region_by_addr(&self, address: u64) -> Option<&Region> {
    //     if let Some((region, _, _, _)) = self.get_region_by_addr(address) {
    //         return Some(region);
    //     }
    //     let mut closest = None;
    //     let mut closest_dist = u64::MAX;
    //     if address < self.program.start {
    //         let dist = self.program.start - address;
    //         if dist < closest_dist {
    //             closest = Some(self.program.as_ref());
    //             closest_dist = dist;
    //         }
    //     }
    //     if address < self.stack.start {
    //         let dist = self.stack.start - address;
    //         if dist < closest_dist {
    //             closest = Some(self.stack.as_ref());
    //             closest_dist = dist;
    //         }
    //     }
    //     if address < self.heap.start {
    //         let dist = self.heap.start - address;
    //         if dist < closest_dist {
    //             closest = Some(self.heap.as_ref());
    //         }
    //     }
    //     closest
    // }
    //
    pub fn get_region(&self, typ: RegionType) -> &Region {
        match typ {
            RegionType::Program => self.program.as_ref(),
            RegionType::Stack => self.stack.as_ref(),
            RegionType::Heap => self.heap.as_ref(),
        }
    }

    /// Get region by type for mutation. The region's page table
    /// will be cloned on write if it is shared
    pub fn mut_region(&mut self, typ: RegionType) -> &mut Region {
        match typ {
            RegionType::Program => Arc::make_mut(&mut self.program),
            RegionType::Stack => Arc::make_mut(&mut self.stack),
            RegionType::Heap => Arc::make_mut(&mut self.heap).deref_mut()
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryFlags {
    /// If enabled, region must be specified when accessing memory.
    /// If the address is not in the specified regions, an error will be thrown
    pub enable_strict_region: bool,

    /// If permission checks are enabled
    pub enable_permission_check: bool,

    /// If an address is in the heap region, check
    /// if it is in the allocated part of the region
    pub enable_allocated_check: bool,
}
