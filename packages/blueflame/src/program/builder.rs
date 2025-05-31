use crate::env::{DataId, GameVer};
use crate::program::{Data, Module, Program, Section, Segment};

/// Start building a program image
pub fn builder(ver: GameVer, program_start: u64, program_size: u32) -> BuilderPhase1 {
    let builder = Builder {
        ver,
        program_start,
        program_size,
        modules: Vec::new(),
    };
    BuilderPhase1(builder)
}

pub struct Builder {
    ver: GameVer,
    program_start: u64,
    program_size: u32,
    modules: Vec<Module>,
}

pub struct BuilderPhase1(Builder);

impl BuilderPhase1 {
    /// Add a new module with the rel_start to program start and name
    pub fn add_module(mut self, name: &str, rel_start: u32) -> Self {
        let module = Module {
            name: name.to_string(),
            rel_start,
            sections: Vec::new(),
        };
        self.0.modules.push(module);
        self
    }

    pub fn done_with_modules(mut self) -> BuilderPhase2 {
        // sort the modules by their relative start
        self.0.modules.sort_by_key(|m| m.rel_start);
        BuilderPhase2(self.0)
    }
}

/// Phase 2 of the builder, where you can add sections to modules
pub struct BuilderPhase2(Builder);
impl BuilderPhase2 {
    /// Add a new section with relative start to the program start.
    /// It will be automatically added to the right module
    pub fn add_section(mut self, rel_start: u32, permissions: u32) -> Self {
        log::debug!("adding section at rel_start: {rel_start:#08x}");
        let i = match self
            .0
            .modules
            .binary_search_by_key(&rel_start, |m| m.rel_start)
        {
            Ok(i) => i,
            Err(i) => {
                assert!(i >= 1, "New section starts before the first module");
                i - 1
            }
        };
        log::debug!("found module index: {i}");
        let module = &mut self.0.modules[i];
        module.sections.push(Section {
            rel_start,
            permissions,
            segments: Vec::new(),
        });
        self
    }

    pub fn done_with_sections(mut self) -> BuilderPhase3 {
        for module in &mut self.0.modules {
            // sort the sections by their relative start
            module.sections.sort_by_key(|s| s.rel_start);
        }
        BuilderPhase3(self.0, Vec::new())
    }
}

/// Phase 3 of the builder, where you can add segments to sections
pub struct BuilderPhase3(Builder, Vec<Data>);
impl BuilderPhase3 {
    /// Add a new segment with relative start to the program start,
    /// and data for the segment (must be page aligned, 4KB).
    pub fn add_segment(mut self, rel_start: u32, data: Vec<u8>) -> Self {
        log::debug!("adding segment at rel_start: {rel_start:#08x}");
        // find the module
        let i = match self
            .0
            .modules
            .binary_search_by_key(&rel_start, |m| m.rel_start)
        {
            Ok(i) => i,
            Err(i) => {
                assert!(i >= 1, "New segment starts before the first module");
                i - 1
            }
        };
        log::debug!("found module index: {i}");
        let module = &mut self.0.modules[i];
        // find the section
        let i = match module
            .sections
            .binary_search_by_key(&rel_start, |s| s.rel_start)
        {
            Ok(i) => i,
            Err(i) => {
                assert!(i >= 1, "New segment starts before the first section");
                i - 1
            }
        };
        log::debug!("found section index: {i}");
        let section = &mut module.sections[i];
        // add the segment to the section
        section.segments.push(Segment { rel_start, data });
        self
    }

    pub fn add_data(mut self, data: DataId, bytes: Vec<u8>) -> Self {
        log::debug!("adding data: {:?}", data);
        self.1.push(Data::new(data, bytes));
        self
    }

    pub fn done(mut self) -> Program {
        // sort the segments by their relative start
        for module in &mut self.0.modules {
            for section in &mut module.sections {
                section.segments.sort_by_key(|s| s.rel_start);
            }
        }
        Program {
            ver: self.0.ver,
            program_start: self.0.program_start,
            program_size: self.0.program_size,
            modules: self.0.modules,
            data: self.1,
        }
    }
}
