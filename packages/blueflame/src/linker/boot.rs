use std::sync::Arc;

use rkyv::rancor;

use crate::env::{DlcVer, Environment, GameVer};
use crate::game::{Proxies, singleton};
use crate::linker::{GameHooks, patch_memory};
use crate::memory::{self, Memory, PAGE_SIZE, REGION_ALIGN, SimpleHeap, align_down, align_up};
use crate::processor::{Cpu1, Cpu3, CrashReport, Process};
use crate::program::ArchivedProgram;

/// Error that only happens during boot
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("program is not a valid archive: {0}")]
    BadImage(String),
    #[error("PMDM address is impossible to satisfy: 0x{0:016x}")]
    InvalidPmdmAddress(u64),
    #[error("region overlap: {0} and {1}")]
    RegionOverlap(String, String),
    #[error("memory error: {0}")]
    Memory(#[from] memory::Error),
    #[error("{0:?}")]
    Crash(#[from] CrashReport),
}

/// Initialize memory for the process
///
/// Return the memory state after all singletons are created and initialized
pub fn init_process(
    image: &ArchivedProgram,
    dlc_version: DlcVer,
    stack_start: u64,
    stack_size: u32,
    pmdm_address: u64,
    heap_free_size: u32,
) -> Result<Process, Error> {
    log::info!("initializing process");
    let ver = rkyv::deserialize::<GameVer, rancor::Error>(&image.ver)
        .map_err(|e| Error::BadImage(e.to_string()))?;
    let env = Environment::new(ver, dlc_version);
    // calculate heap start address
    // we need the heap to be as small as possible,
    // but the relative address of the singleton could be really big
    // (e.g. a few GBs), so we need to adjust the heap start accordingly
    //
    // 0                                         s1             pmdm            s2
    //          |<----------pmdm.rel_start--------------------->|
    //          |<----------min_rel_start------->|
    // |<------------------------pmdm_address------------------>|
    // |<----min_heap_region_start_unaligned---->|
    // |<---heap_region_start--->|
    // |<......>|
    //   ^heap_adjustment
    //          |<---------------------------max_rel_start--------------------->|
    //
    // for any singleton:
    // rel_start + heap_adjustment = address

    let pmdm_rel_start = singleton::pmdm::rel_start(env);
    if pmdm_rel_start as u64 > pmdm_address {
        log::error!(
            "pmdm rel_start is greater than its physical address: 0x{pmdm_rel_start:x} > 0x{pmdm_address:x}"
        );
        return Err(Error::InvalidPmdmAddress(pmdm_address));
    }

    let heap_adjustment = pmdm_address - pmdm_rel_start as u64;
    let min_rel_start = singleton::get_min_rel_start(env);
    let min_heap_region_start_unaligned = heap_adjustment + min_rel_start as u64;
    let heap_region_start = align_down!(min_heap_region_start_unaligned, REGION_ALIGN);

    // calculate how much space will be needed for all the singletons
    let max_rel_start = singleton::get_max_rel_start(env);
    let heap_singletons_end = heap_adjustment + max_rel_start as u64;
    // make the first alloc look random
    let page_off_alloc_start = 0x428;
    // align up to the next page, and reserve 1 page for some spacing
    let heap_start_alloc =
        align_up!(heap_singletons_end, PAGE_SIZE as u64) + PAGE_SIZE as u64 + page_off_alloc_start;
    let heap_size = align_up!(
        (heap_start_alloc - heap_region_start) as u32 + heap_free_size,
        PAGE_SIZE
    );
    log::debug!("heap start is 0x{heap_region_start:016x}, size is 0x{heap_size:08x}");

    let program_start = image.program_start.into();
    let program_size = image.program_size.into();

    // check the regions don't overlap before allocating memory
    if overlaps(program_start, program_size, stack_start, stack_size) {
        return Err(Error::RegionOverlap(
            "program".to_string(),
            "stack".to_string(),
        ));
    }

    if overlaps(program_start, program_size, heap_region_start, heap_size) {
        return Err(Error::RegionOverlap(
            "program".to_string(),
            "heap".to_string(),
        ));
    }
    if overlaps(stack_start, stack_size, heap_region_start, heap_size) {
        return Err(Error::RegionOverlap(
            "stack".to_string(),
            "heap".to_string(),
        ));
    }

    // construct the memory
    log::debug!("creating memory");
    let heap = SimpleHeap::new(heap_region_start, heap_size, heap_start_alloc);
    let mut memory = Memory::new_program_zc(
        env,
        program_start,
        program_size,
        &image.modules,
        heap,
        stack_start,
        stack_size,
    )?;
    // patch the memory
    log::debug!("patching memory");
    patch_memory(&mut memory, env)?;

    log::debug!("creating process");
    let mut proc = Process::new(
        Arc::new(memory),
        Arc::new(Proxies::default()),
        Arc::new(GameHooks),
    );

    // create a temporary processor to initialize the singletons
    log::debug!("creating cpu3");
    let mut cpu1 = Cpu1::default();
    let mut cpu3 = Cpu3::new(&mut cpu1, &mut proc, image, heap_adjustment);
    cpu3.reset_stack();
    cpu3.with_crash_report(|cpu| {
        log::debug!("initializing pmdm");
        singleton::pmdm::create_instance(cpu, env)?;
        log::debug!("initializing gdtm");
        singleton::gdtm::create_instance(cpu, env)?;
        log::debug!("initializing info_data");
        singleton::info_data::create_instance(cpu, env)?;
        log::debug!("initializing aocm");
        singleton::aocm::create_instance(cpu, env)?;
        Ok(())
    })?;

    log::debug!("process created");

    Ok(proc)
}

// TODO --cleanup: remove
// pub fn init_memory_simple(
//     image: &Program,
//     stack_start: u64,
//     stack_size: u32,
//     heap_size: u32,
// ) -> Result<(Memory, Proxies), CrateError> {
//     let program_region = Arc::new(Region::new_program(
//         image.program_start,
//         image.program_size,
//         image.regions(),
//     )?);
//     let stack_region = Arc::new(Region::new_rw(RegionType::Stack, stack_start, stack_size));
//     let heap_region = Arc::new(SimpleHeap::new(
//         stack_size as u64 + stack_start,
//         heap_size,
//         0_u64,
//     ));
//
//     let flags = MemoryFlags {
//         enable_strict_region: true,
//         enable_permission_check: true,
//         enable_allocated_check: true,
//     };
//
//     let memory = Memory::new(
//         flags,
//         program_region,
//         stack_region,
//         heap_region,
//         None,
//         None,
//         None,
//     );
//     let proxies = Proxies::default();
//     Ok((memory, proxies))
// }

fn overlaps(a_start: u64, a_size: u32, b_start: u64, b_size: u32) -> bool {
    let a_end = a_start + a_size as u64;
    let b_end = b_start + b_size as u64;
    (b_start < a_end && b_start >= a_start) || (b_end < a_end && b_end >= a_start)
}
