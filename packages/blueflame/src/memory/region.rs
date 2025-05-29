use std::sync::Arc;

use bit_set::BitSet;
use enumset::{EnumSet, EnumSetType};

#[layered_crate::import]
use memory::{
    super::program::{ProgramRegion, ArchivedProgramRegion}, 
    self::{AccessFlags, Error, Page, PAGE_SIZE, align_down, align_up}
};

pub const REGION_ALIGN: u64 = 0x10000;

/// Memory region implementation
///
/// A region is a contiguous block of physical memory with
/// a starting address aligned to 0x10000 and a fixed size.
///
/// Cloning a region will clone the page table,
/// but the page contents are clone-on-write
#[derive(Debug, Clone)]
pub struct Region {
    /// Type of the region, used for tracking and debugging
    pub tag: &'static str,
    /// The flags for this region, which indicates its type and permissions
    pub flags: AccessFlags,
    /// Physical start address of the region. Must be aligned to 0x10000
    pub start: u64,
    /// Maximum capacity of the region in bytes
    pub capacity: u32,

    /// Clone-on-write page table
    pages: Vec<Arc<Page>>,
}

impl Region {
    /// Construct a program region from program region definition
    ///
    /// All pages in the program region are eagerly allocated. For simplicity,
    /// this also include blank pages excluded from the image, which are zeroed.
    /// The entire program region is around 70MB, which should be well under
    /// the memory limit for WASM32
    ///
    /// The program region is assumed to be page-aligned, but as fail-safe,
    /// it will be aligned up to the next page boundary
    ///
    /// `program_start` is the physical address where the program is loaded
    /// will be aligned down to the nearest 0x10000, if not already
    pub fn from_program_regions(program_start: u64, program_size: u32, regions: &[ArchivedProgramRegion]) -> Result<Vec<Self>, Error> {
        let start = align_down!(program_start, REGION_ALIGN);

        let mut region_meta = Vec::new();



        let num_pages = align_up!(program_size, PAGE_SIZE) / PAGE_SIZE;
    }

    // pub fn new_rx(
    //     start: u64, byte_size: u32, data: &[ArchivedProgramRegion]) -> Self {
    //
    //     todo!()
    //    
    // }
    /// Construct a program region from program region definition
    ///
    /// All pages in the program region are eagerly allocated. For simplicity,
    /// this also include blank pages excluded from the image, which are zeroed.
    /// The entire program region is around 70MB, which should be well under
    /// the memory limit for WASM32
    ///
    /// The program region is assumed to be page-aligned, but as fail-safe,
    /// it will be aligned up to the next page boundary
    ///
    /// `program_start` is the physical address where the program is loaded
    /// will be aligned down to the nearest 0x10000, if not already
    pub fn new_program(
        program_start: u64,
        program_size: u32,
        regions: &[ProgramRegion],
    ) -> Result<Self, Error> {
        let start = align_down!(program_start, REGION_ALIGN);
        let num_pages = align_up!(program_size, PAGE_SIZE) / PAGE_SIZE;
        let mut pages = Vec::with_capacity(num_pages as usize);
        // construct all the pages
        let mut current_start: u32 = 0;
        for region in regions {
            // align down just for safety, if the program image is not aligned,
            // it's bad anyway
            let region_start = align_down!(region.rel_start, PAGE_SIZE);
            if current_start > region_start {
                // should not happen unless the program image is bad
                return Err(Error::Unexpected(format!(
                    "program image has overlapping regions! current: 0x{current_start:08x} > next: 0x{region_start:08x}",
                )));
            }
            while current_start < region_start {
                // invalid regions are supposed to be inaccessible,
                // so we don't give any permission
                let page = Arc::new(Page::zeroed(EnumSet::empty()));
                pages.push(page);
                current_start += PAGE_SIZE;
            }
            let data_len = region.data().len() as u32;
            let permissions = AccessType::from_perms(region.permissions);
            // number of pages for this region in the image, align just for safety
            let num_pages_curr = align_up!(data_len, PAGE_SIZE) / PAGE_SIZE;
            for i in 0..num_pages_curr {
                // usize should be either 32 or 64 on our supported platforms
                let s = (i * PAGE_SIZE) as usize;
                let e = ((i + 1) * PAGE_SIZE).min(data_len) as usize;
                let page = Arc::new(Page::from_slice(region.data()[s..e].as_ref(), permissions));
                pages.push(page);
                current_start += PAGE_SIZE;
            }
        }

        while current_start < program_size {
            let page = Arc::new(Page::zeroed(EnumSet::empty()));
            pages.push(page);
            current_start += PAGE_SIZE;
        }
        Ok(Self {
            tag: "main", // TODO --cleanup: region for all modules
            typ: RegionType::Program,
            start,
            capacity: program_size,
            pages,
        })
    }

    /// Construct a dynamic RW region
    ///
    /// The dynamic regions are used for stack and heap. In the simulator,
    /// they are all pretty small, so we also pre-allocate all the pages
    /// for simplicity
    pub fn new_rw(region_type: RegionType, start: u64, size: u32) -> Self {
        let start = align_down!(start, REGION_ALIGN);
        let num_pages = align_up!(size, PAGE_SIZE) / PAGE_SIZE;
        let mut pages = Vec::with_capacity(num_pages as usize);
        for _ in 0..num_pages {
            let page = Arc::new(Page::zeroed(AccessType::Read | AccessType::Write));
            pages.push(page);
        }
        Self {
            tag: match region_type {
                RegionType::Program => "main",
                RegionType::Stack => "stack",
                RegionType::Heap => "heap",
            },
            typ: region_type,
            start,
            capacity: size,
            pages,
        }
    }

    /// Get the allocated size of the region in bytes
    ///
    /// In the current implementation, this is always equal to the capacity
    pub fn len_bytes(&self) -> u32 {
        self.pages.len() as u32 * PAGE_SIZE
    }

    /// Get the start physical address of the region
    pub const fn start(&self) -> u64 {
        self.start
    }

    /// Get the end physical address of the region (exclusive)
    pub fn end(&self) -> u64 {
        self.start + self.capacity as u64
    }

    /// Get page by page index, returns None if the page is not allocated
    pub fn get(&self, page_idx: u32) -> Option<&Arc<Page>> {
        self.pages.get(page_idx as usize)
    }

    /// Get mutable page reference by page index, returns None if the page is not allocated
    ///
    /// If the page is currently shared, it will be cloned (clone-on-write)
    pub fn get_mut(&mut self, page_idx: u32) -> Option<&mut Page> {
        self.pages.get_mut(page_idx as usize).map(Arc::make_mut)
    }

    /// Return true if the address is in addresses reserved for this region
    pub fn is_addr_in_region(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.start + self.capacity as u64
    }

    /// Get the page for the given address as (Page, page_index, page_offset)
    ///
    /// Returns `None` if the address is not in this region,
    /// or if the page is unallocated.
    pub fn read_at_addr(&self, addr: u64) -> Option<(&Page, u32, u32)> {
        if addr < self.start || addr >= self.start + self.len_bytes() as u64 {
            return None;
        }
        // relative address in the region
        let rel_addr = addr - self.start;
        let region_page_idx = (rel_addr / PAGE_SIZE as u64) as u32;
        let page = self.pages.get(region_page_idx as usize)?;
        let page_off = (rel_addr % PAGE_SIZE as u64) as u32;
        Some((page.as_ref(), region_page_idx, page_off))
    }

}

/// Type of the region used for tracking and debugging purposes
///
/// Unlike a regular OS where the regions are ordered program -> heap -> stack
/// from low to high virtual addresses. NX does not have such system
/// and these regions are physical memory that can be in any order.
#[derive(Debug, EnumSetType)]
pub enum RegionType {
    /// The program segments
    Program,
    /// The stack segment
    ///
    /// Usually this contains stacks for all threads,
    /// but the simulator will only have one thread
    Stack,
    /// The heap segment
    Heap, 

    //
    // do we need TLS (Thread Local)?
}

impl std::fmt::Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegionType::Program => write!(f, "program"),
            RegionType::Stack => write!(f, "stack"),
            RegionType::Heap => write!(f, "heap"),
        }
    }
}
