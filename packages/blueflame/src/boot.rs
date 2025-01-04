use std::sync::Arc;

use uking_relocate_lib::singleton::{Singleton, SingletonCreator};
use uking_relocate_lib::{Env, Program};

use crate::memory::{align_down, align_up, Memory, MemoryFlags, Region, RegionType, SimpleHeap, PAGE_SIZE, REGION_ALIGN};
use crate::processor::Processor;


/// Initialize memory for the process
///
/// Return the memory state after all singletons are created and initialized
pub fn init_memory(
    image: &Program,
    stack_start: u64,
    stack_size: u32,
    pmdm_address: u64,
    heap_free_size: u32,
) -> Arc<Memory> {


    let pmdm_info = image.singleton_by_id(Singleton::PauseMenuDataMgr).unwrap(); // TODO: error
    // type
    // heap_start + rel_start = pmdm_address
    // heap_start = pmdm_address - rel_start
    let pmdm_rel_start = pmdm_info.rel_start;
    if pmdm_rel_start as u64 > pmdm_address {
        panic!("pmdm_rel_start > pmdm_address"); // TODO: handle error
    }
    let heap_start = align_down!(pmdm_address - pmdm_rel_start as u64, REGION_ALIGN);
    let heap_adjustment = (pmdm_address - heap_start) - pmdm_rel_start as u64;

    // heap_start + heap_adjustment + singleton.rel_start = address for that singleton

    // calculate how much space will be needed for all the singletons
    let mut heap_end = heap_start;
    let singletons = image.singletons();
    for singleton in singletons {
        let singleton_end = heap_start + heap_adjustment + singleton.rel_start as u64 + singleton.size as u64;
        heap_end = heap_end.max(singleton_end);
    }
    // align up to the next page, and reserve 1 page for some spacing
    heap_end = align_up!(heap_end, PAGE_SIZE as u64) + PAGE_SIZE as u64;
    let heap_size = (heap_end - heap_start) as u32 + heap_free_size;
    let heap_start_alloc = heap_end + 0x428; // make it look random
    
    // check if program/stack/heap overlap (TODO: remove the panics)
    if overlaps(image.program_start, image.program_size, stack_start, stack_size) {
        panic!("program and stack overlap");
    }
    if overlaps(image.program_start, image.program_size, heap_start, heap_size) {
        panic!("program and heap overlap");
    }
    if overlaps(stack_start, stack_size, heap_start, heap_size) {
        panic!("stack and heap overlap");
    }

    // construct the memory


    let program_region = Arc::new(Region::new_program(
        image.program_start,
        image.program_size,
        image.regions()).unwrap()); // TODO: error type
    //
    let stack_region = Arc::new(Region::new_rw(RegionType::Stack, stack_start, stack_size));
    let heap_region = Arc::new(SimpleHeap::new(heap_start, heap_size, heap_start_alloc));

    let flags = MemoryFlags {
        enable_strict_region: true,
        enable_permission_check: true,
        enable_allocated_check: true,
    };

    let mut memory = Memory::new(flags, program_region, stack_region, heap_region);

    // create the processor to initialize the singletons

    let mut processor = Processor::default();

    let mut singleton_init = SingletonInit {
        env: image.env,
        program_start: image.program_start,
        processor: &mut processor,
        memory: &mut memory,
        heap_start_adjusted: heap_start + heap_adjustment,
    };

    for singleton in singletons {
        singleton.create_instance(&mut singleton_init).unwrap(); // TODO: error
    }

    Arc::new(memory)
}

fn overlaps(a_start: u64, a_size: u32, b_start: u64, b_size: u32) -> bool {
    let a_end = a_start + a_size as u64;
    let b_end = b_start + b_size as u64;
    (b_start < a_end && b_start >= a_start) || (b_end < a_end && b_end >= a_start)
}

pub struct SingletonInit<'p, 'm> {
    env: Env,
    program_start: u64,
    processor: &'p mut Processor,
    memory: &'m mut Memory,
    heap_start_adjusted: u64,
}

impl SingletonCreator for SingletonInit<'_, '_> {
    type Error = (); // TODO: error type from executing code

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
}
