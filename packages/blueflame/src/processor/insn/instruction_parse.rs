use crate::processor::{self as self_, crate_};

use disarm64::decoder::*;

use self_::Error;

use super::instruction_registry::{ExecutableInstruction, LegacyInstruction};
use super::instructions::*;

type ParseFn = fn(&Opcode) -> Result<Option<Box<dyn ExecutableInstruction>>, Error>;
pub static PARSE_LIST: &[ParseFn] = &[
    movz::InsnMovz::parse_from_decode,
    // sbfm::InsnSbfm::parse_from_decode,
    // movn::InsnMovn::parse_from_decode,
    // bfm::InsnBfm::parse_from_decode,
    // lslv::InsnLslv::parse_from_decode,
    // ldarb::InsnLdarb::parse_from_decode,
    // fsub::InsnFsub::parse_from_decode,
    // fadd::InsnFadd::parse_from_decode,
    // lsrv::InsnLsrv::parse_from_decode,
];

pub trait InsnParser {
    fn parse_from_decode(
        d: &Opcode,
    ) -> Result<Option<Box<dyn ExecutableInstruction>>, Error>;
}

pub const fn get_bit_range(bits: u32, start_idx: u8, end_idx: u8) -> u32 {
    // remove the bits that are before the start idx by moving them to the left, this uses the bitfield
    // convention where the MSB has index 31
    let truncate_start = bits << (31 - start_idx);
    truncate_start >> (31 - (start_idx - end_idx))
}

pub const fn get_bit_range_big(bits: u64, start_idx: u8, end_idx: u8) -> u64 {
    // remove the bits that are before the start idx by moving them to the left, this uses the bitfield
    // convention where the MSB has index 63
    let truncate_start = bits << (63 - start_idx);
    truncate_start >> (63 - (start_idx - end_idx))
}

pub fn byte_to_inst(raw: u32) -> Result<
Box<dyn ExecutableInstruction>, Error> {
    let Some(opcode) = disarm64::decoder::decode(raw) else {
        log::warn!("failed to decode instruction 0x{raw:08x}");
        return Err(Error::BadInstruction(raw));
    };
    for parsefn in PARSE_LIST {
        let res = parsefn(&opcode)?;
        match res {
            Some(inst) => return Ok(inst),
            None => continue,
        }
    }
    let legacy_inst = LegacyInstruction::from_u32(raw);
    let inst = match legacy_inst {
        Ok(inst) => Box::new(inst),
        Err(e) => {
            for parsefn in PARSE_LIST {
                let res = parsefn(&opcode)?;
                match res {
                    Some(inst) => return Ok(inst),
                    None => continue,
                }
            }
            // println!("parsed as {:?}", opcode);
            // println!("parsed as str {:?}", opcode.to_string());
            // println!("parsed as def {:?}", opcode.definition());
            // println!("bytes {0:#0x}", bytecode);
            return Err(e);
        }
    };
    Ok(inst)
}

#[cfg(test)]
#[test]
pub fn test_bit_range() {
    let temp: u32 = 0x80000000;
    assert_eq!(get_bit_range(temp, 31, 31), 1)
}
#[test]
pub fn test_bit_range_alt() {
    let temp: u32 = 0xf0000000;
    assert_eq!(get_bit_range(temp, 31, 30), 3)
}
