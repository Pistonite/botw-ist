use std::sync::Arc;

use crate::memory::{
    AccessFlags, Error, PAGE_SIZE, Page, REGION_ALIGN, align_down, align_up, perm, region,
};
use crate::program::Section as ProgramSection;

/// Memory section implementation
///
/// Cloning a section will clone the page table,
/// the page contents are clone-on-write
#[derive(Clone)]
pub struct Section {
    /// Name of the module this section belongs to
    pub module_name: String,
    /// Physical address of the start of the module this section belongs to
    pub module_start: u64,
    /// The flags for this section, which indicates its type and permissions
    pub flags: AccessFlags,
    /// Physical address of the start of the section. Must be aligned to 0x10000
    start: u64,
    /// Clone-on-write page table. All pages are eagerly allocated
    /// so we don't have to worry about mutation during reads
    pages: Vec<Arc<Page>>,
}

impl Section {
    /// Create a new RW region as a section (i.e. stack/heap)
    pub fn new_region(name: &str, start: u64, size: u32, flags: AccessFlags) -> Self {
        let start = align_down!(start, REGION_ALIGN);
        let num_pages = align_up!(size, PAGE_SIZE) / PAGE_SIZE;
        let mut pages = Vec::with_capacity(num_pages as usize);
        for _ in 0..num_pages {
            pages.push(Arc::new(Page::zeroed()));
        }
        Self {
            module_name: name.to_string(),
            module_start: start,
            flags,
            start,
            pages,
        }
    }
    /// Create a new program section using zero-copy archive
    pub fn new_program_zc(
        module_name: &str,
        program_start: u64,
        module_start: u64,
        byte_size: u32,
        section: &ProgramSection,
    ) -> Result<Self, Error> {
        let section_rel_start = section.rel_start;
        log::debug!(
            "constructing section for module `{module_name}`, at rel_start=0x{section_rel_start:08x}, size=0x{byte_size:08x}"
        );

        let section_abs_start = program_start + section_rel_start as u64;
        let num_pages = align_up!(byte_size, PAGE_SIZE) / PAGE_SIZE;
        let mut pages = Vec::with_capacity(num_pages as usize);

        // construct pages with data from the image
        let mut current_seg_rel_start = section_rel_start;
        for segment in section.segments.iter() {
            let seg_rel_start = align_down!(segment.rel_start, PAGE_SIZE);
            if current_seg_rel_start > seg_rel_start {
                return Err(Error::SectionConstruction(format!(
                    "program image has overlapping sections! current: 0x{current_seg_rel_start:08x}, segment: 0x{seg_rel_start:08x}"
                )));
            }
            while current_seg_rel_start < seg_rel_start {
                // fill the gap with zeroed pages
                pages.push(Arc::new(Page::zeroed()));
                current_seg_rel_start += PAGE_SIZE;
            }
            let seg_size = segment.data.len() as u32;
            let num_pages_for_seg = align_up!(seg_size, PAGE_SIZE) / PAGE_SIZE;
            for i in 0..num_pages_for_seg {
                let start_byte = (i * PAGE_SIZE) as usize;
                let end_byte = ((i + 1) * PAGE_SIZE).min(seg_size) as usize;
                pages.push(Arc::new(Page::from_slice(
                    &segment.data[start_byte..end_byte],
                )));
                current_seg_rel_start += PAGE_SIZE;
            }
        }

        // compute flags
        let flags = AccessFlags::from(section.permissions);
        let section_flags = if flags.all(perm!(x)) {
            // if it has execute, assume it's a text section
            region!(text)
        } else if flags.all(perm!(w)) {
            // if it has write, assume it's a data section
            region!(data)
        } else {
            // if it has read, assume it's a rodata section
            region!(rodata)
        };
        let flags = flags | section_flags;

        log::debug!(
            "section for module `{module_name}` constructed at 0x{section_abs_start:08x}, size=0x{byte_size:08x}, flags: {flags}"
        );

        Ok(Self {
            module_name: module_name.to_string(),
            module_start,
            flags,
            start: section_abs_start,
            pages,
        })
    }

    /// Get the start physical address of the section
    pub const fn start(&self) -> u64 {
        self.start
    }

    /// Get the end physical address of the section
    pub fn end(&self) -> u64 {
        self.start + self.len_bytes() as u64
    }

    /// Get number of total pages in this section
    pub fn len_pages(&self) -> u32 {
        self.pages.len() as u32
    }

    /// Get the total size in bytes of this section
    pub fn len_bytes(&self) -> u32 {
        self.pages.len() as u32 * PAGE_SIZE
    }

    /// Get a page by page index without checking bounds.
    ///
    /// Unsafe: will panic if the index is out of bounds, so you
    /// put this in an unsafe block to indicate you checked the index yourself.
    pub fn get_unchecked(&self, page_idx: u32) -> &Page {
        self.pages[page_idx as usize].as_ref()
    }

    /// Get a mutable page by page index. The page will be cloned if it is currently shared
    /// (clone-on-write).
    ///
    /// Will panic if the index is out of bounds
    pub fn get_mut_unchecked(&mut self, page_idx: u32) -> &mut Page {
        Arc::make_mut(&mut self.pages[page_idx as usize])
    }
}
