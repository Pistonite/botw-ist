use crate::memory::{align_down, align_up, perm, region, Error, Section, PAGE_SIZE, REGION_ALIGN};

/// A simple heap region implementation
///
/// Since the simulator doesn't make much heap allocation (usually),
/// freed memory are never reclaimed. This is fine, because
/// each re-run of the simulation will have a fresh heap.
///
/// It doesn't track which regions are freed, so UAF won't be detected
/// and is completely safe in the simulation since there is essentially
/// no free.
///
/// The heap structure only tracks the pointers. The actual
/// data is still stored in the memory object
#[derive(Clone)]
pub struct SimpleHeap {
    /// Physical address of the start of the heap region
    start: u64,
    /// Size of the heap region in bytes
    size: u32,
    /// Address of the next allocation
    next_alloc: u64,
}

impl SimpleHeap {
    /// Create a new heap region. `start_alloc` is where
    /// the first allocation will be placed
    pub fn new(heap_start: u64, heap_size: u32, start_alloc: u64) -> Self {
        let start = align_down!(heap_start, REGION_ALIGN);
        let size = align_up!(heap_size, PAGE_SIZE);
        Self {
            start,
            size,
            next_alloc: start_alloc,
        }
    }

    /// Create a section of memory that corresponds to this heap region
    pub fn create_section(&self) -> Section {
        Section::new_region("heap", self.start, self.size, perm!(rw) | region!(heap))
    }

    /// If the address is in the last page of the allocated region
    /// in the heap, then return the maximum page offset that is the boundary
    /// of the allocated region. Otherwise, return None.
    pub fn check_max_page_offset(&self, addr: u64) -> Option<u32> {
        if addr < self.start || addr >= self.start + self.size as u64 {
            return None; // outside the heap region
        }
        let rel_addr = addr - self.start;
        let page_idx = rel_addr / PAGE_SIZE as u64;
        let max_rel_addr = self.next_alloc - self.start;
        let max_page_idx = max_rel_addr / PAGE_SIZE as u64;
        match page_idx.cmp(&max_page_idx) {
            std::cmp::Ordering::Greater => return Some(0), // cannot read anything
            std::cmp::Ordering::Less => return None, // not at the last page yet, can read everything
            std::cmp::Ordering::Equal => {}
        }
        // on the last page of heap
        let max_page_off = max_rel_addr % PAGE_SIZE as u64;
        return Some(max_page_off as u32);
    }

    /// Allocate new space in the heap
    ///
    /// To keep things simple, the alignment is assumed to be 8
    pub fn alloc(&mut self, size: u32) -> Result<u64, Error> {
        let start = align_up!(self.next_alloc, 8);
        if u64::MAX - start < size as u64 {
            return Err(Error::HeapOutOfMemory);
        }
        let end = start + size as u64;
        if end >= self.start + self.size as u64 {
            return Err(Error::HeapOutOfMemory);
        }
        self.next_alloc = end;
        Ok(start)
    }

    /// Return false if:
    /// - The address is in the heap region
    /// - The address is in the unallocated part of the heap
    pub fn check_allocated(&self, addr: u64) -> bool {
        if addr < self.start || addr >= self.start + self.size as u64 {
            return true; // ok: outside the heap region
        }
        addr < self.next_alloc
    }
}
