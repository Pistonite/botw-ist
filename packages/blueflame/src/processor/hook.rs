use crate::processor::{self as self_, crate_};

use self_::{Execute, Error};

use crate_::env::no_panic;

pub struct Hooks {
    entries: Vec<HookEntry>,
}

pub struct HookEntry {
    /// Describes the hook for reporting
    name: &'static str,
    /// Physical starting address to hook
    start: u64,
    /// Size of the hook
    size: u32,
    /// Code to execute in place of the original code
    hook: Box<dyn Execute>,
}

impl HookEntry {
    #[inline(always)]
    fn end(&self) -> u64 {
        self.start.saturating_add(self.size as u64)
    }
}

impl Hooks {
    pub fn register(
        &mut self, 
        name: &'static str,
        main_start: u64,
        start: u64, 
        size: u32, 
        hook: Box<dyn Execute>
    ) -> Result<(), Error> {
        let idx = match self.find(start, size) {
            Ok(idx) => {
                return Err(Error::HookOverlap {
                    new_name: name,
                    new_start: (start - main_start) as u32,
                    existing_name: self.entries[idx].name,
                    existing_start: (self.entries[idx].start - main_start) as u32,
                });
            }
            Err(idx) => idx,
        };
            self.entries.insert(idx, HookEntry {
                name,
                start,
                size,
                hook
            });

        Ok(())
    }

    #[no_panic]
    pub fn get(&self, pc: u64) -> Option<&dyn Execute> {
        let idx = self.find( pc, 0).ok()?;
        Some(self.entries[idx].hook.as_ref())
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
        let lower_bound = match self.entries.binary_search_by_key(&start, HookEntry::end) {
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

