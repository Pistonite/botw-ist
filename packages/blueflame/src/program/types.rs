// use deku::{DekuRead, DekuWrite};
use rkyv::{Archive, Serialize, Deserialize};

#[layered_crate::import]
use program::super_::env::{GameVer, DataId};

/// Image of a program at runtime
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Archive, Serialize, Deserialize)]
pub struct Program {
    /// Version of the game in the program
    pub ver: GameVer,
    /// Physical address of the start of the program region (where nnrtld is loaded), must be page aligned (4KB)
    pub program_start: u64,
    /// Size of the program region
    pub program_size: u32,
    /// Modules in the program
    pub modules: Vec<Module>,
    /// Static data files used by the program
    pub data: Vec<Data>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Archive, Serialize, Deserialize)]
pub struct Module {
    /// Name of this module
    pub name: String,
    /// Start of the module relative to start of the program
    pub rel_start: u32,
    /// Sections in this module
    pub sections: Vec<Section>,
}

/// A section, like .text or .data...
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Archive, Serialize, Deserialize)]
pub struct Section {
    /// Relative start of this section compared to the start of the program
    pub rel_start: u32,
    /// Permission of the section
    ///  - 0x1: Execute
    ///  - 0x2: Write
    ///  - 0x4: Read
    pub permissions: u32,
    /// Segments of data in this section
    pub segments: Vec<Segment>,
}

/// A segment is a contiguous block of raw data in the program,
/// with an offset relative to the start of the program.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Archive, Serialize, Deserialize)]
pub struct Segment {
    /// Relative start of the segment compared to start of the program
    pub rel_start: u32,
    /// Data of the segment, must be page aligned (4KB)
    pub data: Vec<u8>,
}

/// Data stored in the program image
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Archive, Serialize, Deserialize)]
pub struct Data {
    /// Id (type) of the data
    pub id: DataId,

    // bytes_len: u32,
    // #[deku(count = "bytes_len")]
    /// The raw bytes of the data
    pub bytes: Vec<u8>,
}

impl Data {
    pub fn new(id: DataId, bytes: Vec<u8>) -> Self {
        // let bytes_len = bytes.len() as u32;
        Self {
            id,
            // bytes_len,
            bytes,
        }
    }
}

