use super::super::instruction_parse::InsnParser;
use crate::processor::instruction_parse::get_bit_range_big;
use crate::processor::instruction_registry::{ExecutableInstruction, RegisterType};

use crate::processor::Error;
use crate::Core;

use disarm64_defn::defn::InsnOpcode;

use super::super::instruction_parse::get_bit_range;
use anyhow::bail;
#[derive(Clone)]
pub struct InsnBfm {
    rd: RegisterType,
    rn: RegisterType,
    immr: u8,
    imms: u8,
}

impl Core<'_, '_, '_> {
    pub fn bfm(
        &mut self,
        rd: RegisterType,
        rn: RegisterType,
        immr: u8,
        imms: u8,
    ) -> Result<(), Error> {
        let mut rd_val = self.cpu.read_gen_reg(&rd)? as u64;
        let rn_val = self.cpu.read_gen_reg(&rn)? as u64;

        if imms >= immr {
            let start_idx = immr;
            let copy_size = 1 + imms - immr;
            let end_idx = start_idx + copy_size - 1;

            let src_bits = get_bit_range_big(rn_val, end_idx, start_idx);

            let mask = ((1u64 << copy_size) - 1) << start_idx;
            rd_val = (rd_val & !mask) | (src_bits << start_idx);
        } else {
            let copy_size = imms + 1;
            let src_bits = get_bit_range_big(rn_val, copy_size - 1, 0);

            let mask = ((1u64 << copy_size) - 1) << immr;
            rd_val = (rd_val & !mask) | (src_bits << immr);
        }

        self.cpu.write_gen_reg(&rd, rd_val as i64)?;
        Ok(())
    }
}

impl ExecutableInstruction for InsnBfm {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.bfm(self.rd, self.rn, self.immr, self.imms)?;
        Ok(())
    }
}

impl InsnParser for InsnBfm {
    fn parse_from_decode(
        d: &disarm64::Opcode,
    ) -> std::result::Result<Option<Box<(dyn ExecutableInstruction)>>, anyhow::Error> {
        if d.mnemonic != disarm64::decoder_full::Mnemonic::bfm {
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
            _ => bail!("Invalid decode value for sf in bfm inst"),
        };
        let rn = match sf {
            0 => RegisterType::WReg(rn_idx),
            1 => RegisterType::XReg(rn_idx),
            _ => bail!("Invalid decode value for sf in bfm inst"),
        };
        Ok(Some(Box::new(InsnBfm {
            rd,
            rn,
            immr: immr as u8,
            imms: imms as u8,
        })))
    }
}

#[cfg(test)]
use anyhow::Result;
#[test]
// Simple test to make sure that movz instructions don't parse as bfms
pub fn test_bfm_parse_neg() -> Result<()> {
    let bfm_test = InsnBfm::parse_from_decode(&disarm64::decoder::decode(0x52800088).unwrap())?;
    if let Some(_) = bfm_test {
        panic!("movz parsed as bfm")
    }
    Ok(())
}
