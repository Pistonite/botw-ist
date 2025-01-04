use derive_more::derive::{Deref, DerefMut};

use super::{align_up, error::Error, region::{Region, RegionType}};


/// A simple heap region implementation
///
/// It makes sure that:
/// - The region start is page-aligned
/// - Singleton allocations are allocated so they have the same
///   offsets relative to each other regardless of the heap start
/// - All newer allocations come after the singletons
/// 
/// Since the simulator doesn't make much heap allocation (usually),
/// freed memory are never reclaimed. This is fine, because
/// each re-run of the simulation will have a fresh heap.
///
/// It doesn't track which regions are freed, so UAF won't be detected
/// and is completely safe in the simulation since there is essentially
/// no free. However, it does track which region are not yet
/// allocated and throw error
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct SimpleHeap {
    /// Internal storage of the heap
    #[deref]
    #[deref_mut]
    region: Region,

    /// Address of the next allocation
    next_alloc: u64,
}

impl SimpleHeap {
    /// Create a new heap region. `start_alloc` is where
    /// the first allocation will be placed
    pub fn new(heap_start: u64, heap_size: u32, start_alloc: u64) -> Self {
        let region = Region::new_rw(RegionType::Heap, heap_start, heap_size);
        Self {
            region,
            next_alloc: start_alloc,
        }
    }

    /// Allocate new space in the heap
    ///
    /// To keep things simple, the alignment is assumed to be 8
    pub fn alloc(&mut self, size: u32) -> Result<u64, Error> {
        let start = align_up!(self.next_alloc, 8);
        if u64::MAX - start < size as u64 {
            return Err(Error::OutOfMemory(start, RegionType::Heap));
        }
        let end = start + size as u64;
        if end >= self.region.start + self.region.capacity as u64 {
            return Err(Error::OutOfMemory(end, RegionType::Heap));
        }
        self.next_alloc = end;
        Ok(start)
    }

    /// Return if the address is in the allocated region of the heap
    pub fn is_allocated(&self, addr: u64) -> bool {
        return addr < self.next_alloc;
    }
}
