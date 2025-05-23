use crate::processor::{self as self_, crate_};

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate_::memory::Memory;
use crate_::env::no_panic;

use self_::{Error, Cpu0, Process};

pub trait Execute: Send + Sync + std::panic::UnwindSafe + 'static {
    /// Execute this code the middle. `step` is number of instructions
    /// from the beginning
    fn execute_from(&self, cpu: &mut Cpu0, proc: &mut Process, step: u32) -> Result<(), Error>;
}


impl<F> Execute for F where 
F: 
Fn(&mut Cpu0, &mut Process) -> Result<(), Error> + Send + Sync 
+ std::panic::UnwindSafe + 'static {
    fn execute_from(&self, cpu: &mut Cpu0, proc: &mut Process, step: u32) -> Result<(), Error> {
        if step != 0 {
            // TODO --cleanup: core: get main offset
            return Err(Error::StrictReplacement { main_offset: 0 })
        }
        self(cpu, proc)
    }
}

pub fn box_execute<F>(f: F) -> Box<dyn Execute> 
where
F: 
Fn(&mut Cpu0, &mut Process) -> Result<(), Error> + Send + Sync 
+ std::panic::UnwindSafe + 'static
{
    Box::new(f)
}

/// The execute cache is a per-processor cache for saving instruction
/// fetch results in blocks, so we can avoid duplicated memory reads
/// and instruction decodes
pub struct ExecuteCache {
    /// Entries in the cache, sorted by starting addresses
    /// and cannot overlap
    entries: Vec<ExecuteCacheEntry>,
}

struct ExecuteCacheEntry {
    is_permanent: bool,
    start: u64,
    size: u32,
    f: Box<dyn Execute>,
}

impl ExecuteCacheEntry {
    #[inline(always)]
    fn end(&self) -> u64 {
        self.start.saturating_add(self.size as u64)
    }
}

impl ExecuteCache {
    /// Delete the temporary cache entries, which will cause instructions
    /// to be refetched from memory
    pub fn flush(&mut self) {
        self.entries.retain(|e| e.is_permanent);
    }

    /// Insert new entry into the cache.
    ///
    /// Returns ExecuteCacheOverlap if the entry overlaps with existing
    /// entries
    pub fn insert(&mut self, 
        is_permanent: bool,
        main_start: u64,
        start: u64,
        size: u32,
        f: Box<dyn Execute>,
    ) -> Result<(), Error> {
        match self.find(start, size) {
            Ok(i) => {
                Err(Error::ExecuteCacheOverlap { new_start: (start - main_start) as u32,
                    existing_start: (self.entries[i].start - main_start) as u32 })
            },
            Err(i) => {
                self.entries.insert(i, ExecuteCacheEntry {
                    is_permanent,
                    start,
                    size,
                    f
                });
                Ok(())
            }
        }
    }

    /// Get the cached execution entry for the given PC.
    ///
    /// If found, 2 values are returned: the first value is the Execute object and the second value
    /// is the step to execute it at (number of instructions, i.e. byte size / 4)
    ///
    /// If not found, return the maximum byte size of instructions to be loaded
    /// from memory and inserted later
    pub fn get(&self, pc: u64) -> Result<(&dyn Execute, u32), u32> {
        match self.find(pc, 4) {
            Ok(i) => {
                let entry = &self.entries[i];
                let step = ((pc - entry.start) / 4) as u32;
                Ok((entry.f.as_ref(), step))
            }
            Err(i) => {
                const MAX: u64 = 0x200000;
                match self.entries.get(i.saturating_add(1)) {
                    Some(next_entry) => {
                        Err((next_entry.start - pc).min(MAX) as u32)
                    }
                    None => {
                        Err(MAX as u32)
                    }
                }
            }
        }
    }

    /// Find hook index in the storage
    ///
    /// If size is 0, find the index of the hook that starts at exactly `start`.
    ///
    /// Otherwise, find the first index of the hook that overlaps with the range
    ///
    /// If not found, return Err with the index that a new hook with start
    /// can be inserted.
    #[no_panic]
    fn find(&self, start: u64, size: u32) -> Result<usize, usize> {
        if size == 0 {
            return self.entries.binary_search_by_key(&start, |hook| hook.start);
        }

        let end = start.saturating_add(size as u64);
        let lower_bound = match self.entries.binary_search_by_key(&start, ExecuteCacheEntry::end) {
            Ok(idx) | Err(idx) => idx
        };
        
        for (offset, hook) in self.entries[lower_bound..].iter().enumerate() {
            if hook.start >= end {
                return Err(lower_bound + offset);
            }
            if hook.end() > start {
                let x = lower_bound + offset;
                return Ok(x)
            }
        }
        Err(self.entries.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(start: u64, size: u32) -> ExecuteCacheEntry {
        ExecuteCacheEntry { is_permanent: false, start, size, f: self_::box_execute(|_, _| Ok(())) }
    }

    fn make_vec() -> ExecuteCache {
        ExecuteCache {
            entries: vec![
                make_entry(10, 5), // [10, 15)
                make_entry(20, 10), // [20, 30)
                make_entry(35, 5), // [35, 40)
            ],
        }
    }

    #[test]
    fn test_register_get() {
        let mut hv = make_vec();

        assert!(hv.insert(true, 0, 40, 80, self_::box_execute(|_, _| {
            // this is a test so we can execute it and see the result
            return Err( Error::StrictReplacement { main_offset: 42 })
        })) .is_ok());
        assert!(hv.insert(true, 0, 52, 16, self_::box_execute(|_, _| Ok(()))) .is_err());
        let (_entry, step) = hv.get(40).unwrap();
        assert_eq!(step, 0);
        // TODO --cleanup: core - check if the entry is the same
        let (_, step) = hv.get(48).unwrap();
        assert_eq!(step, 2);
    }

    #[test]
    fn test_find_size_zero_found() {
        let hv = make_vec();
        // Exact start matches
        assert_eq!(hv.find(10, 0), Ok(0));
        assert_eq!(hv.find(20, 0), Ok(1));
        assert_eq!(hv.find(35, 0), Ok(2));
    }

    #[test]
    fn test_find_size_zero_not_found() {
        let hv = make_vec();
        // Before all
        assert_eq!(hv.find(5, 0), Err(0));
        // Between hooks
        assert_eq!(hv.find(15, 0), Err(1));
        assert_eq!(hv.find(30, 0), Err(2));
        // After all
        assert_eq!(hv.find(50, 0), Err(3));
    }

    #[test]
    fn test_find_size_nonzero_found() {
        let hv = make_vec();
        // Overlaps with first hook
        assert_eq!(hv.find(8, 5), Ok(0));   // [8,13) overlaps [10,15)
        assert_eq!(hv.find(12, 2), Ok(0));  // [12,14) overlaps [10,15)
        assert_eq!(hv.find(14, 10), Ok(0)); // [14,24) overlaps [10,15) and [20,30), but should return first overlap (0)
        // Overlaps with second hook
        assert_eq!(hv.find(25, 2), Ok(1));  // [25,27) overlaps [20,30)
        assert_eq!(hv.find(29, 10), Ok(1)); // [29,39) overlaps [20,30) and [35,40), should return 1
        // Overlaps with third hook
        assert_eq!(hv.find(36, 1), Ok(2));  // [36,37) overlaps [35,40)
    }

    #[test]
    fn test_find_size_nonzero_not_found() {
        let hv = make_vec();
        // Before all
        assert_eq!(hv.find(0, 5), Err(0));
        // Between hooks, not overlapping or touching boundaries
        assert_eq!(hv.find(15, 2), Err(1)); // [15,17) strictly between [10,15) and [20,30)
        assert_eq!(hv.find(17, 2), Err(1)); // [17,19) strictly between [10,15) and [20,30)
        assert_eq!(hv.find(30, 2), Err(2)); // [30,32) strictly between [20,30) and [35,40)
        assert_eq!(hv.find(32, 2), Err(2)); // [32,34) strictly between [20,30) and [35,40)
        // Between hooks, not overlapping, but touching previous end or next start (already covered in boundary test)
        // After all
        assert_eq!(hv.find(45, 5), Err(3));
    }

    #[test]
    fn test_find_size_nonzero_boundary() {
        let hv = make_vec();
        // End exactly at start of a hook (should not overlap)
        assert_eq!(hv.find(15, 5), Err(1)); // [15,20) ends at 20, which is start of second hook
        // Start exactly at end of a hook (should not overlap)
        assert_eq!(hv.find(15, 5), Err(1)); // [15,20) after [10,15)
        // Range exactly matches a hook
        assert_eq!(hv.find(10, 5), Ok(0));
        assert_eq!(hv.find(20, 10), Ok(1));
        assert_eq!(hv.find(35, 5), Ok(2));
        // Range covers multiple hooks, should return first overlapping
        assert_eq!(hv.find(10, 30), Ok(0)); // [10,40) covers all hooks, returns 0
    }
}
