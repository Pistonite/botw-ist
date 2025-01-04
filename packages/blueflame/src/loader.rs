use std::collections::HashMap;

use crate::Core;


// loader creates Memory and Processor instances
// it needs to do the following: (in this order)
// - create the executable memory pages and copy the executable in
// - relocate the executable, which means
//   - put the real addresses of the symbols in the .got.plt
//   - put the real addresses of functions in vtables
// <at this point the executable is runnable>
// - create and allocate the stack pages (4MB)
// - create the heap region
// - for each singleton:
//   - allocate the memory in the heap region for its instance
//     - where the memory is is defined by configuration, as we
//       need to put those singletons in the same spot the real game would put it
//   - simulate the createInstance function for the singleton, which
//     - allocates memory (this is already done, just return the pointer from previous step)
//     - call ctor of the Disposer - skip this for now and leave the disposer uninitialized, as we
//       don't really care about it
//     - call the ctor of the singleton - we DO care about this
//     - write the singleton address to the sInstance field (in .data section)
// <at this point, we are ready to run simulation>
// 
//   

pub trait LoaderInfo {


    /// Create a table of stub functions.
    /// 
    /// If the PC of the processor matches a key, then the corresponding
    /// function is called and returned
    fn create_stub_function_table(&self) -> HashMap<u64, Box<dyn FnMut(&Core) -> Result<(), ()>>>;

    /// physical address in .got.plt -> key for stub function table
    fn get_external_symbol_table(&self) -> HashMap<u64, u64>;

    /// Get the region information for the program, stack, and heap
    fn get_regions(&self) -> Regions;

    fn get_executable(&self) -> Executable;

    /// Get the relative addresses of the singletons
    /// to heap start 
    fn get_singletons(&self) -> Singletons;



}

pub struct Regions {
    pub program: RegionInfo,
    pub stack: RegionInfo,
    pub heap: RegionInfo,
}

pub struct RegionInfo {
    pub start: u64,
    pub size: usize,
}

pub struct Executable {
    data: Vec<ExecutableSegment>
}

pub struct ExecutableSegment {
    pub start: u32,
    pub data: Vec<u8>
}

pub struct Singletons {
    pub pmdm: u64,
}
