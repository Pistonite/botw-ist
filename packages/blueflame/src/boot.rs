use std::sync::Arc;

use blueflame_program::Program;
use blueflame_singleton::VirtualMachine;
use blueflame_utils::{DataType, Environment, ProxyType};

use crate::error::Error as CrateError;
use crate::memory::{
    align_down, align_up, Memory, MemoryFlags, Proxies, Region, RegionType, SimpleHeap, PAGE_SIZE,
    REGION_ALIGN,
};
use crate::processor::{Processor, RegIndex};
use crate::Core;

/// Error that only happens during boot
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("PMDM address is impossible to satisfy: 0x{0:08x}")]
    InvalidPmdmAddress(u64),
    #[error("heap is too small: need at least {0} bytes")]
    HeapTooSmall(u32),
    #[error("region overlap: {0} and {1}")]
    RegionOverlap(RegionType, RegionType),
    #[error("no data found for type")]
    NoData,
    #[error("proxy failed to allocate")]
    ProxyError,
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

    let pmdm = blueflame_singleton::pmdm(image.env);
    let pmdm_rel_start = pmdm.rel_start;
    if pmdm_rel_start as u64 > pmdm_address {
        return Err(Error::InvalidPmdmAddress(pmdm_address).into());
    }

    let singletons = [
        pmdm,
        blueflame_singleton::gdt_manager(image.env),
        blueflame_singleton::info_data(image.env),
        blueflame_singleton::aoc_manager(image.env),
    ];

    let min_heap_start = pmdm_address - pmdm_rel_start as u64;
    let min_rel_start = singletons
        .iter()
        .map(|s| s.rel_start)
        .min()
        .unwrap_or_default();

    let max_heap_start = min_heap_start + min_rel_start as u64;
    let heap_start = align_down!(max_heap_start, REGION_ALIGN);
    if heap_start < min_heap_start {
        // somehow align down made it smaller
        // maybe possible with some pmdm_address
        return Err(Error::InvalidPmdmAddress(pmdm_address).into());
    }
    let heap_adjustment = heap_start - min_heap_start;

    // calculate how much space will be needed for all the singletons
    let max_rel_start = singletons
        .iter()
        .map(|s| s.rel_start)
        .max()
        .unwrap_or_default();
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
    if overlaps(
        image.program_start,
        image.program_size,
        stack_start,
        stack_size,
    ) {
        return Err(Error::RegionOverlap(RegionType::Program, RegionType::Stack).into());
    }

    if overlaps(
        image.program_start,
        image.program_size,
        heap_start,
        heap_size,
    ) {
        return Err(Error::RegionOverlap(RegionType::Program, RegionType::Heap).into());
    }
    if overlaps(stack_start, stack_size, heap_start, heap_size) {
        return Err(Error::RegionOverlap(RegionType::Stack, RegionType::Heap).into());
    }

    // construct the memory

    let program_region = Arc::new(Region::new_program(
        image.program_start,
        image.program_size,
        image.regions(),
    )?);

    let stack_region = Arc::new(Region::new_rw(RegionType::Stack, stack_start, stack_size));
    let heap_region = Arc::new(SimpleHeap::new(
        heap_start,
        heap_size,
        heap_min_size as u64 + heap_start, //
    ));

    let flags = MemoryFlags {
        enable_strict_region: true,
        enable_permission_check: true,
        enable_allocated_check: true,
    };

    let mut memory = Memory::new(
        flags,
        program_region,
        stack_region,
        heap_region,
        Some(pmdm_address),
        Some(image.env.main_offset()),
        None,
    );

    // create a temporary processor to initialize the singletons

    let mut processor = Processor::default();
    let mut proxies = Proxies::default();

    let mut singleton_init = SingletonInit {
        env: image.env,
        core: processor.attach(&mut memory, &mut proxies),
        heap_start_adjusted: heap_start - heap_adjustment,
        image,
    };

    for singleton in singletons {
        singleton.create(&mut singleton_init)?;
    }

    Ok((memory, proxies))
}

pub fn init_memory_simple(
    image: &Program,
    stack_start: u64,
    stack_size: u32,
    heap_size: u32,
) -> Result<(Memory, Proxies), CrateError> {
    let program_region = Arc::new(Region::new_program(
        image.program_start,
        image.program_size,
        image.regions(),
    )?);
    let stack_region = Arc::new(Region::new_rw(RegionType::Stack, stack_start, stack_size));
    let heap_region = Arc::new(SimpleHeap::new(
        stack_size as u64 + stack_start,
        heap_size,
        0_u64,
    ));

    let flags = MemoryFlags {
        enable_strict_region: true,
        enable_permission_check: true,
        enable_allocated_check: true,
    };

    let memory = Memory::new(
        flags,
        program_region,
        stack_region,
        heap_region,
        None,
        None,
        None,
    );
    let proxies = Proxies::default();
    Ok((memory, proxies))
}

fn overlaps(a_start: u64, a_size: u32, b_start: u64, b_size: u32) -> bool {
    let a_end = a_start + a_size as u64;
    let b_end = b_start + b_size as u64;
    (b_start < a_end && b_start >= a_start) || (b_end < a_end && b_end >= a_start)
}

pub struct SingletonInit<'p, 'm, 'x, 'pr> {
    env: Environment,
    core: Core<'p, 'm, 'x>,
    heap_start_adjusted: u64,
    image: &'pr Program,
}

const ENTER_ADDR: u64 = 100;

impl VirtualMachine for SingletonInit<'_, '_, '_, '_> {
    type Error = CrateError;

    fn enter(&mut self, target: u32) -> Result<(), Self::Error> {
        let main_offset = self.env.main_offset();
        self.set_reg(30, ENTER_ADDR)?;
        self.core.cpu.set_pc((target + main_offset) as u64);
        Ok(())
    }

    fn set_reg(&mut self, reg: u8, value: u64) -> Result<(), Self::Error> {
        if reg <= 31 {
            self.core.cpu.write_gen_reg(
                &crate::processor::instruction_registry::RegisterType::XReg(reg as RegIndex),
                value as i64,
            )?;
        } else {
            self.core.cpu.write_gen_reg(
                &crate::processor::instruction_registry::RegisterType::SReg((reg - 32) as RegIndex),
                value as i64,
            )?;
        }
        Ok(())
    }

    fn copy_reg(&mut self, from: u8, to: u8) -> Result<(), Self::Error> {
        let v = if from <= 31 {
            self.core.cpu.read_gen_reg(
                &crate::processor::instruction_registry::RegisterType::XReg(from as RegIndex),
            )? as u64
        } else {
            self.core.cpu.read_gen_reg(
                &crate::processor::instruction_registry::RegisterType::SReg(
                    (from - 32) as RegIndex,
                ),
            )? as u64
        };
        if to <= 31 {
            self.core.cpu.write_gen_reg(
                &crate::processor::instruction_registry::RegisterType::XReg(to as RegIndex),
                v as i64,
            )?;
        } else {
            self.core.cpu.write_gen_reg(
                &crate::processor::instruction_registry::RegisterType::SReg((to - 32) as RegIndex),
                v as i64,
            )?;
        }
        Ok(())
    }

    fn execute_until(&mut self, target: u32) -> Result<(), Self::Error> {
        while self.core.cpu.pc != Into::<u64>::into(target + self.env.main_offset()) {
            self.core.execute_at_pc()?;
        }
        Ok(())
    }

    fn jump(&mut self, pc: u32) -> Result<(), Self::Error> {
        let main_offset = self.env.main_offset();
        // physical address of the instruction we need to set PC to
        // let address = self.program_start + main_offset as u64 + pc as u64;
        self.core.cpu.pc = (main_offset + pc) as u64;
        Ok(())
    }

    fn allocate_memory(&mut self, bytes: u32) -> Result<(), Self::Error> {
        let pointer = self.core.mem.heap_mut().alloc(bytes)?;
        self.set_reg(0, pointer)
    }

    fn allocate_proxy(&mut self, proxy: ProxyType) -> Result<(), Self::Error> {
        let proxy_start = self.core.allocate_proxy(proxy)?;
        self.set_reg(0, proxy_start)
    }

    fn allocate_data(&mut self, data: DataType) -> Result<(), Self::Error> {
        let prog_data = self.image.get_data(data);
        let start = if let Some(prog) = prog_data {
            self.core.allocate_data(prog.bytes().to_vec())
        } else {
            return Err(crate::Error::Boot(Error::NoData));
        }?;
        self.set_reg(0, start)
    }

    fn get_singleton(&mut self, reg: u8, rel_start: u32) -> Result<(), Self::Error> {
        let singleton_address = self.heap_start_adjusted + rel_start as u64;
        self.set_reg(reg, singleton_address)
    }

    fn execute_to_complete(&mut self) -> Result<(), Self::Error> {
        while self.core.cpu.pc != ENTER_ADDR {
            self.core.execute_at_pc()?;
        }
        Ok(())
    }
}
