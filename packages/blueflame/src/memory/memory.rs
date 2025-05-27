use std::ops::DerefMut;
use std::sync::Arc;

use derive_more::derive::Constructor;

#[layered_crate::import]
use memory::{
    super::env::{Environment, enabled},
    self::{glue, AccessFlags, AccessFlag, MemAccess, access, Ptr, Error, SimpleHeap, Page, Reader, Region, RegionType, Writer}
};

/// Memory of the simulated process
#[derive(Debug, Clone, Constructor)]
pub struct Memory {
    env: Environment,
    /// program region
    program: Arc<Region>,
    /// stack region
    stack: Arc<Region>,
    /// heap region
    pub heap: Arc<SimpleHeap>,
}

impl Memory {
    /// Get the emulated environment constants
    pub fn env(&self) -> Environment {
        self.env
    }

    /// Get the physical starting address of the program region
    pub fn program_start(&self) -> u64 {
        self.program.start
    }

    /// Get the physical starting address of the main module
    pub fn main_start(&self) -> u64 {
        self.program.start + self.env.main_offset() as u64
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
            let is_execute = flags.has_all(AccessFlag::Execute);
            let disable_permission_check = flags.has_all(access!(force));

            let trace_offset = if region.tag == "main" {
                self.env.main_offset()
            } else {
                0
            };
            // TODO --cleanup: remove execute from param
            return Ok(Reader::new(
                self, 
                region, 
                page, 
                page_idx, off, is_execute, disable_permission_check,
                trace_offset as u64
            ));
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
            let disable_permission_check = flags.has_all(access!(force));
            let trace_offset = if region.tag == "main" {
                self.env.main_offset()
            } else {
                0
            };
            return Ok(Writer::new(self, region.typ, page_idx, off, disable_permission_check, trace_offset as u64));
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

    /// Allocate space on the heap for the given byte slice,
    /// and copy the slice to the allocated space.
    ///
    /// Return the pointer to the slice
    pub fn alloc_with(&mut self, data: &[u8]) -> Result<u64, Error> {
        let heap = self.heap_mut();
        let ptr = heap.alloc(data.len() as u32)?;
        Ptr!(<u8>(ptr)).store_slice(data, self)?;
        Ok(ptr)
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
