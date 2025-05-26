use deku::{DekuRead, DekuWrite};

#[layered_crate::import]
use program::super_::env::{GameVer, DataId};

/// Image of a program at runtime
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct Program {
    /// Version of the game in the program
    pub ver: GameVer,
    /// Physical address of the start of the program region (where nnrtld is loaded), must be page aligned (4KB)
    pub program_start: u64,
    /// Size of the program region
    pub program_size: u32,

    program_regions_len: u32, // required for serialization
    /// Regions of the program. The image may not contain the full program,
    /// just range of addresses that are used.
    #[deku(count = "program_regions_len")]
    program_regions: Vec<ProgramRegion>,

    data_len: u32,
    /// Static data files used by the program
    #[deku(count = "data_len")]
    data: Vec<ProgramData>,
}


impl Program {
    /// Get the program regions stored in the image
    pub fn regions(&self) -> &[ProgramRegion] {
        &self.program_regions
    }

    /// Get the data stored in the image by id
    pub fn data(&self, id: DataId) -> Option<&ProgramData> {
        self.data.iter().find(|d| d.id == id)
    }
}


/// One contiguous region of memory in the program
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct ProgramRegion {
    /// Start of the region relative to the program_start, must be page aligned (4KB)
    pub rel_start: u32,
    /// Permission of the region
    ///  - 0x1: Execute
    ///  - 0x2: Write
    ///  - 0x4: Read
    pub permissions: u32,

    data_len: u32,
    /// Data of the region, must be page aligned (4KB)
    #[deku(count = "data_len")]
    data: Vec<u8>,
}

impl ProgramRegion {
    pub fn new(rel_start: u32, permissions: u32, data: Vec<u8>) -> Self {
        let data_len = data.len() as u32;
        Self {
            rel_start,
            permissions,
            data_len,
            data,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn into_data(self) -> Vec<u8> {
        self.data
    }
}

/// Data stored in the program image
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct ProgramData {
    /// Id (type) of the data
    pub id: DataId,

    bytes_len: u32,
    /// The raw bytes of the data
    #[deku(count = "bytes_len")]
    bytes: Vec<u8>,
}

impl ProgramData {
    pub fn new(id: DataId, bytes: Vec<u8>) -> Self {
        let bytes_len = bytes.len() as u32;
        Self {
            id,
            bytes_len,
            bytes,
        }
    }

    /// Get the raw bytes for this data
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Builder for a program
///
/// The binary serialization requires that the length
/// fields are set correctly for Vecs. This builder
/// is used to ensure that
pub struct ProgramBuilder {
    ver: GameVer,
    program_start: u64,
    program_size: u32,
    program_regions: Vec<ProgramRegion>,
    data: Vec<ProgramData>,
}

impl ProgramBuilder {
    /// Create a new builder and set the version
    pub fn new(ver: GameVer) -> Self {
        Self {
            ver,
            program_start: 0,
            program_size: 0,
            program_regions: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Set the program regions
    pub fn set_program_location(mut self, start: u64, size: u32) -> Self {
        self.program_start = start;
        self.program_size = size;
        self
    }

    pub fn add_regions<I: IntoIterator<Item=ProgramRegion>>(mut self, regions: I)-> Self {
        self.program_regions.extend(regions);
        self
    }

    pub fn add_data(mut self, data: ProgramData) -> Self {
        self.data.push(data);
        self
    }

    /// Build the program
    pub fn build(self) -> Program {
        Program {
            ver: self.ver,
            program_start: self.program_start,
            program_size: self.program_size,
            program_regions_len: self.program_regions.len() as u32,
            program_regions: self.program_regions,
            data_len: self.data.len() as u32,
            data: self.data,
        }
    }
}
