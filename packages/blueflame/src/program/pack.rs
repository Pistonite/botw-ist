use std::io::{Read, Write};

use flate2::Compression;
use flate2::write::GzEncoder;
use rkyv::rancor;

use crate::program::Program;

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
    let config = bincode::config::standard().with_little_endian().with_fixed_int_encoding();
    let data = bincode::encode_to_vec(program, config)
        .map_err(|e| Error::Serialize(e.to_string()))?;

    // let data =
    //     rkyv::to_bytes::<rancor::Error>(program).map_err(|e| Error::Serialize(e.to_string()))?;

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
    let mut decoded = Vec::new();
    let mut decoder = flate2::read::GzDecoder::new(data);
    decoder
        .read_to_end(&mut decoded)
        .map_err(|e| Error::Decompress(e.to_string()))?;

    let config = bincode::config::standard().with_little_endian().with_fixed_int_encoding();

    let (program, _) = bincode::decode_from_slice(&decoded, config)
    // rkyv::deserialize::<Program, rancor::Error>(program)
        .map_err(|e| Error::Deserialize(e.to_string()))?;

    Ok(program)
}

// /// Unpack a Blueflame image into a program with zero-copy deserialization
// pub fn unpack_zc<'a>(data: &[u8], out: &'a mut Vec<u8>) -> Result<&'a ArchivedProgram, Error> {
//     let mut decoder = flate2::read::GzDecoder::new(data);
//     decoder
//         .read_to_end(out)
//         .map_err(|e| Error::Decompress(e.to_string()))?;
//
//     let program = rkyv::access::<ArchivedProgram, rancor::Error>(&out[..])
//         .map_err(|e| Error::Deserialize(e.to_string()))?;
//
//     Ok(program)
// }
