use crate::program as __self;

use std::io::{Read, Write};

use deku::{DekuContainerRead, DekuContainerWrite};
use flate2::write::GzEncoder;
use flate2::Compression;


use __self::Program;

/// Errors packing or unpacking programs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error serializing the program
    #[error("fail to serialize the program: {0}")]
    Serialize(String),
    #[error("fail to compress the program: {0}")]
    Compress(String),
    #[error("fail to decompress the program: {0}")]
    Decompress(String),
    #[error("fail to deserialize the program: {0}")]
    Deserialize(String),
}

/// Pack the program into a Blueflame image
pub fn pack(program: &Program) -> Result<Vec<u8>, Error> {
    let data = program
        .to_bytes()
        .map_err(|e| Error::Serialize(e.to_string()))?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&data)
        .map_err(|e| Error::Compress(e.to_string()))?;
    let data = encoder
        .finish()
        .map_err(|e| Error::Compress(e.to_string()))?;
    Ok(data)
}

/// Unpack a Blueflame image into a program
pub fn unpack(data: &[u8]) -> Result<Program, Error> {
    let mut decoder = flate2::read::GzDecoder::new(data);
    let mut data = Vec::new();
    decoder
        .read_to_end(&mut data)
        .map_err(|e| Error::Decompress(e.to_string()))?;
    let (_, program) =
        Program::from_bytes((&data, 0)).map_err(|e| Error::Deserialize(e.to_string()))?;
    Ok(program)
}
