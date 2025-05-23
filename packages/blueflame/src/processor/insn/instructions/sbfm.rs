use super::super::instruction_parse::InsnParser;
use crate::processor::instruction_parse::get_bit_range_big;
use crate::processor::instruction_registry::{ExecutableInstruction, RegisterType};

use crate::processor::Error;
use crate::Core;

use disarm64_defn::defn::InsnOpcode;

use super::super::instruction_parse::get_bit_range;
use anyhow::bail;
#[derive(Clone)]
pub struct InsnSbfm {
    rd: RegisterType,
    rn: RegisterType,
    immr: u8,
    imms: u8,
}

impl Core<'_, '_, '_> {
    pub fn sbfm(
        &mut self,
        rd: RegisterType,
        rn: RegisterType,
        immr: u8,
        imms: u8,
    ) -> Result<(), Error> {
        let regsize: u8 = match rd {
            RegisterType::XReg(_) => 64,
            RegisterType::WReg(_) => 32,
            _ => {
                return Err(Error::InstructionError(String::from(
                    "sbfm: trying to execute with non general register destination",
                )))
            }
        };
        let mut reg_out_val: i64;
        let rn_val = self.cpu.read_gen_reg(&rn)? as u64;
        if imms >= immr {
            // I think this case is correct
            // copy to LSB of destination
            //TODO: more thourough testing of this.
            let start_idx = immr;
            let copy_size = 1 + imms - immr;

            let end_idx = start_idx + copy_size - 1;

            //ARM documentation has bitfields start in increasing order (right to left)
            //this is opposite of the convention used for the get_bit_range function,
            let src_bits = get_bit_range_big(rn_val, end_idx, start_idx) as i64;
            //sign extend the copied in bits
            reg_out_val = src_bits << (64 - copy_size);
            reg_out_val >>= 64 - copy_size;
        } else {
            // copy from LSB of source
            let copy_size = imms + 1;
            let src_bits = get_bit_range_big(rn_val, copy_size - 1, 0) as i64;
            // shift so MSB of src bits is MSB of u64, appropriately shifts in 0s.
            reg_out_val = src_bits << (64 - copy_size);
            // then sign right shift to appropriate position, right shift will appropriately
            // sign extend if needed.
            reg_out_val >>= immr - copy_size;
            if regsize == 32 {
                //account for larger size of i64
                reg_out_val >>= 32;
            }
        }
        self.cpu.write_gen_reg(&rd, reg_out_val)?;
        Ok(())
    }
}

impl ExecutableInstruction for InsnSbfm {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sbfm(self.rd, self.rn, self.immr, self.imms)
    }
}
impl InsnParser for InsnSbfm {
    fn parse_from_decode(
        d: &disarm64::Opcode,
    ) -> std::result::Result<Option<Box<(dyn ExecutableInstruction)>>, anyhow::Error> {
        if d.mnemonic != disarm64::decoder_full::Mnemonic::sbfm {
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
            _ => bail!("Invalid decode value for sf in sbfm inst"),
        };
        let rn = match sf {
            0 => RegisterType::WReg(rn_idx),
            1 => RegisterType::XReg(rn_idx),
            _ => bail!("Invalid decode value for sf in sbfm inst"),
        };
        Ok(Some(Box::new(InsnSbfm {
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
//simple test to make sure that movz instructions don't parse as sbfms
pub fn test_sbfm_parse_neg() -> Result<()> {
    let sbfm_test = InsnSbfm::parse_from_decode(&disarm64::decoder::decode(0x52800088).unwrap())?;
    if let Some(_) = sbfm_test {
        panic!("movz parsed as sbfm")
    }
    Ok(())
}
