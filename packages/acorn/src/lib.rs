use std::{borrow::{Borrow, BorrowMut}, collections::HashMap, sync::Arc};

mod command;
mod error;
mod registers;
mod features;
mod util;
mod processor;

/// A read-only snapshot of memory pages at a certain
/// simulation step
pub struct MemorySnapshot {
}

const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone)]
pub struct Page {
    data: [u8; PAGE_SIZE],
    // other metadata, like permission., etc
}

impl Page {
    pub fn new() -> Self {
        Self {
            data: [0; PAGE_SIZE],
            // other ...
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageTable {
    pages: HashMap<u64, Arc<Page>>,
}

impl PageTable {
    /// Get read access to page at addr
    ///
    /// Returns None if the page is not allocated, i.e. a page fault
    pub fn read(&self, addr: u64) -> Option<&Page> {
        self.pages.get(&addr).map(|page| page.as_ref())
    }

    /// Get write access to page at addr
    ///
    /// Allocates the page if it's not allocated. Clones the page if it's shared
    pub fn write(&mut self, addr: u64) -> &mut Page {
        let arc_page = self.pages.entry(addr).or_insert_with(|| Arc::new(Page::new()));
        Arc::make_mut(arc_page)
    }
}


// pub struct Core<'m> {
//     registers: Registers,
//     // having lifetime here is OK. 
//     // Core is not meant to be a short-lived object, 
//     // only to associate the CPU functions with the memory
//     memory: &'m mut PageTable,
// }
//
// impl<'m> Core<'m> {
//     pub fn attach(memory: &'m mut PageTable) -> Self {
//         Self {
//             registers: Registers::new(),
//             memory,
//         }
//     }
//
//     pub fn init_memory(&mut self, executable: Executable) -> SingletonTable {
//         // load the executable, call ctors, etc...
//         // executable could be the partial one we provide
//         // or the full one user provides
//         //
//         // Returns a table of proxy to singletons, like PMDM,
//         // to provide read-only access to the memory
//     }
//
//     pub fn execute_command(&mut self, core_command: CoreCommand) {
//         // ...
//     }
// }
