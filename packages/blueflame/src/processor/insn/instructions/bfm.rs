use crate::processor::{self as self_, crate_};

use disarm64::arm64::InsnOpcode;
use disarm64::decoder::{Mnemonic, Opcode};

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction, get_bit_range};
use self_::{Error, RegisterType, glue};

#[derive(Clone)]
pub struct InsnBfm {
    rd: RegisterType,
    rn: RegisterType,
    immr: u8,
    imms: u8,
}

pub fn parse(d: &Opcode) -> Result<Option<Box<(dyn ExecutableInstruction)>>, Error> {
    if d.mnemonic != Mnemonic::bfm {
        return Ok(None);
    }
    let bits = d.operation.bits();
    let sf = get_bit_range(bits, 31, 31);
    let rd_idx = get_bit_range(bits, 4, 0);
    let rn_idx = get_bit_range(bits, 9, 5);
    let imms = get_bit_range(bits, 15, 10);
    let immr = get_bit_range(bits, 21, 16);
    let rd = match sf {
        0 => RegisterType::WReg(rd_idx),
        1 => RegisterType::XReg(rd_idx),
        _ => {
            log::error!("Invalid decode value for sf in bfm inst: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    let rn = match sf {
        0 => RegisterType::WReg(rn_idx),
        1 => RegisterType::XReg(rn_idx),
        _ => {
            log::error!("Invalid decode value for sf in bfm inst: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    Ok(Some(Box::new(InsnBfm {
        rd,
        rn,
        immr: immr as u8,
        imms: imms as u8,
    })))
}

impl ExecutableInstruction for InsnBfm {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let mut rd_val = glue::read_gen_reg(core.cpu, &self.rd) as u64;
        let rn_val = glue::read_gen_reg(core.cpu, &self.rn) as u64;

        if self.imms >= self.immr {
            let start_idx = self.immr;
            let copy_size = 1 + self.imms - self.immr;
            let end_idx = start_idx + copy_size - 1;

            let src_bits = parse::get_bit_range_big(rn_val, end_idx, start_idx);

            let mask = 1u64.overflowing_shl(copy_size as u32).0 - 1;
            let mask = mask << start_idx;
            rd_val = ((rd_val & !mask) | (src_bits << start_idx)) >> self.immr;
        } else {
            let copy_size = self.imms + 1;
            let src_bits = parse::get_bit_range_big(rn_val, copy_size - 1, 0);

            let mask = 1u64.overflowing_shl(copy_size as u32).0 - 1;
            let mask = mask << self.immr;
            rd_val = ((rd_val & !mask) | (src_bits << self.immr)) >> self.immr;
        }

        glue::write_gen_reg(core.cpu, &self.rd, rd_val as i64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, insn::paste_insn, reg};

    fn test_bfm(bits: u32, input: u64, expected: u64) -> anyhow::Result<()> {
        let opcode = disarm64::decoder::decode(bits).expect("failed to decode");
        let insn = parse(&opcode)?.unwrap();
        let mut cpu = Cpu0::default();
        cpu.write(reg!(x[1]), input);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        insn.exec_on(&mut core)?;
        assert_eq!(cpu.read::<u64>(reg!(x[0])), expected);
        Ok(())
    }

    #[test]
    pub fn test_full_bitfield() -> anyhow::Result<()> {
        // bfm x0, x1, #0, #63
        test_bfm(
            paste_insn!(20 FC 40 B3),
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
        )
    }

    #[test]
    pub fn test_low_byte() -> anyhow::Result<()> {
        // bfm X0, X1, #0, #7
        test_bfm(
            paste_insn!(20 1C 40 B3),
            0x0123_4567_89ab_cdef,
            0x0000_0000_0000_00ef,
        )
    }

    #[test]
    pub fn test_bits_8_to_15() -> anyhow::Result<()> {
        // bfm X0, X1, #8, #15
        test_bfm(
            paste_insn!(20 3C 48 B3),
            0x0123_4567_89ab_cdef,
            0x0000_0000_0000_00cd,
        )
    }

    #[test]
    pub fn test_upper_half_wrapped() -> anyhow::Result<()> {
        // bfm X0, X1, #32, #47
        test_bfm(
            paste_insn!(20 BC 60 B3),
            0xffff_ffff_1234_5678,
            0x0000_0000_0000_ffff,
        )
    }

    #[test]
    pub fn test_middle_word() -> anyhow::Result<()> {
        // bfm X0, X1, #16, #31
        test_bfm(
            paste_insn!(20 7C 50 B3),
            0x0000_0000_ffff_ffff,
            0x0000_0000_0000_ffff,
        )
    }

    #[test]
    pub fn test_single_high_bit() -> anyhow::Result<()> {
        // bfm X0, X1, #63, #63
        test_bfm(
            paste_insn!(20 FC 7F B3),
            0x0000_0000_0000_0001,
            0x0000_0000_0000_0000,
        )
    }

    #[test]
    pub fn test_single_bit_wraparound() -> anyhow::Result<()> {
        // bfm X0, X1, #63, #63
        test_bfm(
            paste_insn!(20 FC 7F B3),
            0x8000_0000_0000_0000,
            0x0000_0000_0000_0001,
        )
    }
}
