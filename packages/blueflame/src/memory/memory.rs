use std::sync::Arc;

use crate::env::{Environment, enabled};
use crate::memory::{
    AccessFlag, AccessFlags, Error, PAGE_SIZE, Page, Ptr, REGION_ALIGN, Reader, Section,
    SimpleHeap, Writer, align_up, perm, region,
};
use crate::program::ArchivedModule;

/// Memory of the simulated process
#[derive(Clone)]
pub struct Memory {
    env: Environment,
    sections: Vec<Arc<Section>>,
    heap: SimpleHeap,
    program_start: u64,
    stack_end: u64,
}

impl Memory {
    /// Create a new memory without the program, only heap and stack,
    pub fn new(env: Environment, heap_size: u32, heap_alloc: u64, stack_size: u32) -> Self {
        let heap = SimpleHeap::new(0, heap_size, heap_alloc);
        let heap_section = heap.create_section();
        let stack_section = Section::new_region(
            "stack",
            align_up!(0x1000 + heap_section.len_bytes() as u64, REGION_ALIGN),
            stack_size,
            perm!(rw) | region!(stack),
        );
        let stack_end = stack_section.start() + stack_section.len_bytes() as u64;
        Self {
            env,
            sections: vec![Arc::new(heap_section), Arc::new(stack_section)],
            heap,
            program_start: 0,
            stack_end,
        }
    }

    pub fn new_program_zc(
        env: Environment,
        program_start: u64,
        program_size: u32,
        modules: &[ArchivedModule],
        heap: SimpleHeap,
        stack_start: u64,
        stack_size: u32,
    ) -> Result<Self, Error> {
        let mut sections = Vec::new();

        for module in modules {
            let module_start = module.rel_start.to_native() as u64 + program_start;
            let module_name = module.name.as_str();
            for i in 0..module.sections.len() {
                let section_rel_start = module.sections[i].rel_start.to_native();
                let section_size = match module.sections.get(i + 1) {
                    Some(next) => next.rel_start.to_native() - section_rel_start,
                    None => program_size - section_rel_start,
                };
                let section = Section::new_program_zc(
                    module_name,
                    program_start,
                    module_start,
                    section_size,
                    &module.sections[i],
                )?;
                sections.push(Arc::new(section));
            }
        }

        let heap_section = heap.create_section();
        sections.push(Arc::new(heap_section));
        let stack_section =
            Section::new_region("stack", stack_start, stack_size, perm!(rw) | region!(stack));
        let stack_end = stack_section.start() + stack_section.len_bytes() as u64;
        sections.push(Arc::new(stack_section));

        sections.sort_by_key(|s| s.start());

        Ok(Self {
            env,
            sections,
            heap,
            program_start,
            stack_end,
        })
    }

    /// Get the emulated environment constants
    pub fn env(&self) -> Environment {
        self.env
    }

    /// Get the physical starting address of the program region
    pub fn program_start(&self) -> u64 {
        self.program_start
    }

    /// Get the physical starting address of the main module
    pub fn main_start(&self) -> u64 {
        self.program_start + self.env.main_offset() as u64
    }

    pub fn stack_end(&self) -> u64 {
        self.stack_end
    }

    /// Format the address as section+offset
    pub fn format_addr(&self, addr: u64) -> String {
        let i = match self.sections.binary_search_by_key(&addr, |s| s.start()) {
            Ok(i) => i,
            Err(0) => {
                // address is before the first section, format as absolute address
                // (probably offseting off a nullptr or something)
                return format!("0x{addr:016x}");
            }
            Err(i) => i - 1,
        };
        let section = &self.sections[i];
        let in_section = section.end() > addr;
        let section_off = addr - section.module_start;
        let name = &section.module_name;
        format!(
            "{name}+0x{section_off:08x}{}",
            if in_section { ' ' } else { '~' }
        )
    }

    /// Create a reader to start reading at address
    ///
    /// # Flags
    /// If any region bit is specified, then only those regions are allowed to
    /// be accessed. If the execute permission bit is set, the region being accessed
    /// also needs to have execute permission. Region permissions are still checked,
    /// of course.
    pub fn read(&self, addr: u64, flags: AccessFlags) -> Result<Reader, Error> {
        let flags = perm!(r) | convert_region_flags(flags);
        let (section_idx, page_idx, page_off, max_page_off) = self.calculate(addr, flags)?;
        let page = self.page_by_indices_unchecked(section_idx, page_idx);

        Ok(Reader::new(self, page, page_off, max_page_off, addr, flags))
    }

    /// Create a writer to start writing at address
    ///
    /// # Flags
    /// If any region bit is specified, then only those regions are allowed to
    /// be accessed. Otherwise all regions are allowed (permissions are still checked, of course)
    pub fn write(&mut self, addr: u64, flags: AccessFlags) -> Result<Writer, Error> {
        let flags = perm!(w) | convert_region_flags(flags);
        let (section_idx, page_idx, page_off, max_page_off) = self.calculate(addr, flags)?;

        Ok(Writer::new(
            self,
            section_idx,
            page_idx,
            page_off,
            max_page_off,
            addr,
            flags,
        ))
    }

    /// Get a page by section index and page index without checking bounds.
    /// This is safe if the bounds are get through `calculate` method.
    pub fn page_by_indices_unchecked(&self, section_idx: u32, page_idx: u32) -> &Page {
        self.sections[section_idx as usize].get_unchecked(page_idx)
    }
    /// Get a mutable page by section index and page index without checking bounds.
    /// This is safe if the bounds are get through `calculate` method.
    pub fn page_by_indices_mut_unchecked(&mut self, section_idx: u32, page_idx: u32) -> &mut Page {
        Arc::make_mut(&mut self.sections[section_idx as usize]).get_mut_unchecked(page_idx)
    }

    /// Calculate the section index, page index, page offset, and max page offset
    /// for the given address. Also checks if access the address is allowed
    pub fn calculate(&self, addr: u64, flags: AccessFlags) -> Result<(u32, u32, u32, u32), Error> {
        // return section index, page index, page offset, max page offset
        if !self.heap.check_allocated(addr) {
            if enabled!("mem-strict-heap") {
                log::error!(
                    "accessing unallocated heap address: 0x{addr:016x} ({})",
                    self.format_addr(addr)
                );
                return Err(Error::HeapUnallocated(addr, flags));
            }
            log::warn!(
                "bypassed - accessing unallocated heap address: 0x{addr:016x} ({})",
                self.format_addr(addr)
            );
        }
        let Some(section_idx) = self.find_section_idx(addr) else {
            if enabled!("mem-strict-section") {
                log::error!(
                    "accessing invalid section: 0x{addr:016x} ({})",
                    self.format_addr(addr)
                );
                return Err(Error::InvalidSection(addr, flags));
            }
            log::warn!(
                "bypassed - accessing invalid section: 0x{addr:016x} ({})",
                self.format_addr(addr)
            );
            return Err(Error::Bypassed);
        };
        let section = &self.sections[section_idx];
        // permission check
        if !flags.all(AccessFlag::Force) && !section.flags.all(flags.perms()) {
            if enabled!("mem-permission") {
                log::error!(
                    "permission denied: 0x{addr:016x} ({})",
                    self.format_addr(addr)
                );
                return Err(Error::PermissionDenied(addr, flags));
            }
            log::warn!(
                "bypassed - accessing section without permission: 0x{addr:016x} ({})",
                self.format_addr(addr)
            );
        }
        let rel_addr = addr - section.start();
        let page_idx = (rel_addr / PAGE_SIZE as u64) as u32;
        let page_off = (rel_addr % PAGE_SIZE as u64) as u32;
        let max_page_off = self.heap.check_max_page_offset(addr).unwrap_or(PAGE_SIZE);
        Ok((section_idx as u32, page_idx, page_off, max_page_off))
    }

    fn find_section_idx(&self, address: u64) -> Option<usize> {
        let i = match self.sections.binary_search_by_key(&address, |s| s.start()) {
            Ok(i) => i,
            Err(0) => return None, // address is before the first section
            Err(i) => i - 1,
        };
        if self.sections[i].end() > address {
            Some(i)
        } else {
            None // address is after the last section
        }
    }

    /// Allocate `size` bytes on the heap.
    pub fn alloc(&mut self, size: u32) -> Result<u64, Error> {
        self.heap.alloc(size)
    }

    /// Allocate space on the heap for the given byte slice,
    /// and copy the slice to the allocated space.
    ///
    /// Return the pointer to the slice
    pub fn alloc_with(&mut self, data: &[u8]) -> Result<u64, Error> {
        let ptr = self.heap.alloc(data.len() as u32)?;
        Ptr!(<u8>(ptr)).store_slice(data, self)?;
        Ok(ptr)
    }
}

fn convert_region_flags(flags: AccessFlags) -> AccessFlags {
    if flags.any(region!(all)) && enabled!("mem-strict-section") {
        flags
    } else {
        flags | region!(all) // allow all regions
    }
}
