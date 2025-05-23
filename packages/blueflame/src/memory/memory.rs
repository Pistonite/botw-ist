use crate::memory::{self as self_, crate_};

use std::ops::DerefMut;
use std::sync::Arc;

// use blueflame_macros::enabled;
use derive_more::derive::Constructor;
use enumset::EnumSet;

use crate_::env::enabled;

use self_::{glue, AccessFlags, AccessFlag, MemAccess};

use super::error::Error;
use super::heap::SimpleHeap;
use super::page::Page;
use super::read::Reader;
use super::region::{Region, RegionType};
use super::write::Writer;

/// Memory of the simulated process
#[derive(Debug, Clone, Constructor)]
pub struct Memory {
    /// program region
    program: Arc<Region>,
    /// stack region
    stack: Arc<Region>,
    /// heap region
    pub heap: Arc<SimpleHeap>,
    /// pmdm address
    pmdm_addr: Option<u64>,
    // /// offset of the main module compared to program region start
    // main_offset: u32,
    /// trigger param addr
    trigger_param_addr: Option<u64>,
}

impl Memory {
    /// Get the physical starting address of the program region
    pub fn program_start(&self) -> u64 {
        self.program.start
    }

    pub fn stack_end(&self) -> u64 {
        self.stack.start + self.stack.len_bytes() as u64
    }

    /// Create a reader to start reading at address
    ///
    /// # Flags
    /// If any region bit is specified, then only those regions are allowed to
    /// be accessed. If the execute permission bit is set, the region being accessed
    /// also needs to have execute permission. Region permissions are still checked,
    /// of course.
    pub fn read(
        &self,
        address: u64,
        flags: AccessFlags,
    ) -> Result<Reader, Error> {
        let flags = convert_region_flags(flags);

        if let Some((region, page, page_idx, off)) = self.get_region_by_addr(address) {
            if !glue::access_flags_contains_region_type(flags, region.typ) {
                return Err(Error::DisallowedRegion(MemAccess {
                    flags,
                    addr: address,
                    bytes: 0,
                }));
            }
            // TODO --cleanup: remove execute from param
            return Ok(Reader::new(self, region, page, page_idx, off, flags.has_all(AccessFlag::Execute)));
        }
        // region read_by_addr will fail if the address is not allocated,
        // in those cases, we want to return Unallocated error
        if self.program.is_addr_in_region(address) {
            return Err(Error::Unallocated(address));
        }
        if self.stack.is_addr_in_region(address) {
            return Err(Error::Unallocated(address));
        }
        if self.heap.is_addr_in_region(address) {
            return Err(Error::Unallocated(address));
        }
        Err(Error::InvalidRegion(address))
    }

    /// Create a writer to start writing at address
    ///
    /// # Flags
    /// If any region bit is specified, then only those regions are allowed to
    /// be accessed. Otherwise all regions are allowed (permissions are still checked, of course)
    pub fn write(
        &mut self,
        address: u64,
        flags: AccessFlags,
    ) -> Result<Writer, Error> {
        let flags = convert_region_flags(flags);

        if let Some((region, _, page_idx, off)) = self.get_region_by_addr(address) {
            if !glue::access_flags_contains_region_type(flags, region.typ) {
                return Err(Error::DisallowedRegion(MemAccess {
                    flags,
                    addr: address,
                    bytes: 0,
                }));
            }
            return Ok(Writer::new(self, region.typ, page_idx, off));
        }
        // region read_by_addr will fail if the address is not allocated,
        // in those cases, we want to return Unallocated error
        if self.program.is_addr_in_region(address) {
            return Err(Error::Unallocated(address));
        }
        if self.stack.is_addr_in_region(address) {
            return Err(Error::Unallocated(address));
        }
        if self.heap.is_addr_in_region(address) {
            return Err(Error::Unallocated(address));
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
            RegionType::Heap => Arc::make_mut(&mut self.heap).deref_mut(),
        }
    }

    pub fn heap_mut(&mut self) -> &mut SimpleHeap {
        Arc::make_mut(&mut self.heap)
    }

    pub fn get_pmdm_addr(&self) -> u64 {
        self.pmdm_addr.unwrap_or(0)
    }


    pub fn set_trigger_param_addr(&mut self, address: u64) {
        self.trigger_param_addr = Some(address)
    }

    pub fn get_trigger_param_addr(&self) -> u64 {
        self.trigger_param_addr.unwrap_or(0)
    }
}

fn convert_region_flags(flags: AccessFlags) -> AccessFlags {
        let region_flags = if enabled!("mem-strict-region") {
            if flags.has_any(AccessFlags::region_all()) {
                flags
            } else {
                AccessFlags::region_all()
            }
        } else {
            AccessFlags::region_all() // TODO --cleanup: macro
        };
        flags | region_flags
}
