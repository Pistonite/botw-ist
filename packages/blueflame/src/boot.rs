use std::sync::Arc;

use uking_relocate_lib::singleton::{ObjType, Singleton, SingletonCreator};
use uking_relocate_lib::{Env, Program};

use crate::error::Error as CrateError;
use crate::memory::{align_down, align_up, Memory, MemoryFlags, Proxies, Region, RegionType, SimpleHeap, PAGE_SIZE, REGION_ALIGN};
use crate::processor::Processor;
use crate::Core;

/// Error that only happens during boot
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("no PMDM singleton found in program image")]
    NoPmdm,
    #[error("PMDM address is impossible to satisfy: 0x{0:08x}")]
    InvalidPmdmAddress(u64),
    #[error("heap is too small: need at least {0} bytes")]
    HeapTooSmall(u32),
    #[error("region overlap: {0} and {1}")]
    RegionOverlap(RegionType, RegionType),
}


/// Initialize memory for the process
///
/// Return the memory state after all singletons are created and initialized
pub fn init_memory(
    image: &Program,
    stack_start: u64,
    stack_size: u32,
    pmdm_address: u64,
    heap_size: u32,
) -> Result<(Memory, Proxies), CrateError> {

    // calculate heap start address
    // we need the heap to be as small as possible,
    // but the relative address of the singleton could be really big
    // (e.g. a few GBs), so we need to adjust the heap start accordingly
    //
    // 0                              heap_start s1             pmdm            s2
    //          |<--heap_adjustment-->|
    //                                |<----pmdm_rel_address--->|
    //          |<----------pmdm.rel_start--------------------->|
    //          |<----------min_rel_start------->|
    //          |<---------------------------max_rel_start--------------------->|
    // |<------------------------pmdm_address------------------>|
    // |<min_hs>|
    //
    // for any singleton:
    // rel_start - heap_adjustment + heap_start = address
    //
    // heap_adjustment is positive and guarateed to be less than rel_start
    // of any singleton
    let pmdm = image.singleton_by_id(Singleton::PauseMenuDataMgr).ok_or(Error::NoPmdm)?;
    if pmdm.rel_start as u64 > pmdm_address {
        return Err(Error::InvalidPmdmAddress(pmdm_address).into());
    }
    let min_heap_start = pmdm_address - pmdm.rel_start as u64;
    let min_rel_start = image.singletons().iter().map(|s| s.rel_start).min().unwrap_or_default();
    let max_heap_start = min_heap_start + min_rel_start as u64;
    let heap_start = align_down!(max_heap_start, REGION_ALIGN);
    if heap_start < min_heap_start {
        // somehow align down made it smaller
        // maybe possible with some pmdm_address
        return Err(Error::InvalidPmdmAddress(pmdm_address).into());
    }
    let heap_adjustment = heap_start - min_heap_start;

    // calculate how much space will be needed for all the singletons
    let max_rel_start = image.singletons().iter().map(|s| s.rel_start).max().unwrap_or_default();
    let heap_end = min_heap_start + max_rel_start as u64;
    // align up to the next page, and reserve 1 page for some spacing
    let heap_singletons_end = align_up!(heap_end, PAGE_SIZE as u64) + PAGE_SIZE as u64;
    let heap_singletons_size = (heap_singletons_end - heap_start) as u32;
    // make the first alloc look random
    let page_off_alloc_start = 0x428;
    let heap_min_size = heap_singletons_size + page_off_alloc_start;
    let heap_size = align_up!(heap_size, PAGE_SIZE);

    if heap_size < heap_min_size {
        return Err(Error::HeapTooSmall(heap_min_size).into());
    }
    
    // check the regions don't overlap before allocating memory
    if overlaps(image.program_start, image.program_size, stack_start, stack_size) {
        return Err(Error::RegionOverlap(RegionType::Program, RegionType::Stack).into());
    }
    if overlaps(image.program_start, image.program_size, heap_start, heap_size) {
        return Err(Error::RegionOverlap(RegionType::Program, RegionType::Heap).into());
    }
    if overlaps(stack_start, stack_size, heap_start, heap_size) {
        return Err(Error::RegionOverlap(RegionType::Stack, RegionType::Heap).into());
    }

    // construct the memory


    let program_region = Arc::new(Region::new_program(
        image.program_start,
        image.program_size,
        image.regions()).unwrap()); // TODO: error type
    //
    let stack_region = Arc::new(Region::new_rw(RegionType::Stack, stack_start, stack_size));
    let heap_region = Arc::new(SimpleHeap::new(
        heap_start, heap_size, heap_min_size as u64 + heap_start));

    let flags = MemoryFlags {
        enable_strict_region: true,
        enable_permission_check: true,
        enable_allocated_check: true,
    };

    let mut memory = Memory::new(flags, program_region, stack_region, heap_region);

    // create a temporary processor to initialize the singletons

    let mut processor = Processor::default();
    let mut proxies = Proxies::default();

    let mut singleton_init = SingletonInit {
        env: image.env,
        program_start: image.program_start,
        core: processor.attach(&mut memory, &mut proxies),
        heap_start_adjusted: heap_start - heap_adjustment,
    };

    for singleton in image.singletons() {
        singleton.create_instance(&mut singleton_init)?;
    }

    Ok((memory, proxies))
}

fn overlaps(a_start: u64, a_size: u32, b_start: u64, b_size: u32) -> bool {
    let a_end = a_start + a_size as u64;
    let b_end = b_start + b_size as u64;
    (b_start < a_end && b_start >= a_start) || (b_end < a_end && b_end >= a_start)
}

pub struct SingletonInit<'p, 'm, 'x> {
    env: Env,
    program_start: u64,
    core: Core<'p, 'm, 'x>,
    heap_start_adjusted: u64,
}

impl SingletonCreator for SingletonInit<'_, '_, '_> {
    type Error = CrateError;

    fn set_main_rel_pc(&mut self, pc: u32) -> Result<(), Self::Error> {
        let main_offset = self.env.main_offset();
        // physical address of the instruction we need to set PC to
        let address = self.program_start + main_offset as u64 + pc as u64;

        todo!()
    }

    fn enter(&mut self, target: u32) -> Result<(), Self::Error> {
        todo!()
    }

    fn execute_until(&mut self, target: u32) -> Result<(), Self::Error> {
        todo!()
    }

    fn allocate(&mut self, rel_start: u32, size: u32) -> Result<(), Self::Error> {
        let singleton_address = self.heap_start_adjusted + rel_start as u64;
        todo!() // store the address in X0
    }

    fn execute_to_return(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn make_proxy(&mut self, obj_type: ObjType, reg: u32) -> Result<(), Self::Error> {
        todo!()
    }
}
