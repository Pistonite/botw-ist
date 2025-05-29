use crate::processor::{self as self_, crate_};

use disarm64::arm64::InsnOpcode;
use disarm64::decoder::{Mnemonic, Opcode};

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction, get_bit_range};
use self_::{Error, RegisterType, glue};

#[derive(Clone)]
pub struct InsnSbfm {
    rd: RegisterType,
    rn: RegisterType,
    immr: u8,
    imms: u8,
}

impl ExecutableInstruction for InsnSbfm {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        // proc.sbfm(self.rd, self.rn, self.immr, self.imms)
        let regsize: u8 = match self.rd {
            RegisterType::XReg(_) => 64,
            RegisterType::WReg(_) => 32,
            _ => {
                log::error!("sbfm: trying to execute with non general register destination");
                return Err(Error::BadInstruction(0));
            }
        };
        let mut reg_out_val: i64;
        let rn_val = glue::read_gen_reg(core.cpu, &self.rn) as u64;
        if self.imms >= self.immr {
            // I think this case is correct
            // copy to LSB of destination
            //TODO: more thourough testing of this.
            let start_idx = self.immr;
            let copy_size = 1 + self.imms - self.immr;

            let end_idx = start_idx + copy_size - 1;

            //ARM documentation has bitfields start in increasing order (right to left)
            //this is opposite of the convention used for the get_bit_range function,
            let src_bits = parse::get_bit_range_big(rn_val, end_idx, start_idx) as i64;
            //sign extend the copied in bits
            reg_out_val = src_bits << (64 - copy_size);
            reg_out_val >>= 64 - copy_size;
        } else {
            // copy from LSB of source
            let copy_size = self.imms + 1;
            let src_bits = parse::get_bit_range_big(rn_val, copy_size - 1, 0) as i64;
            // shift so MSB of src bits is MSB of u64, appropriately shifts in 0s.
            reg_out_val = src_bits << (64 - copy_size);
            // then sign right shift to appropriate position, right shift will appropriately
            // sign extend if needed.
            reg_out_val >>= self.immr - copy_size;
            if regsize == 32 {
                //account for larger size of i64
                reg_out_val >>= 32;
            }
        }
        glue::write_gen_reg(core.cpu, &self.rd, reg_out_val);
        Ok(())
    }
}

pub fn parse(d: &Opcode) -> Result<Option<Box<(dyn ExecutableInstruction)>>, Error> {
    if d.mnemonic != Mnemonic::sbfm {
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
            log::error!("Invalid decode value for sf in sbfm inst: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    let rn = match sf {
        0 => RegisterType::WReg(rn_idx),
        1 => RegisterType::XReg(rn_idx),
        _ => {
            log::error!("Invalid decode value for sf in sbfm inst: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    Ok(Some(Box::new(InsnSbfm {
        rd,
        rn,
        immr: immr as u8,
        imms: imms as u8,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //simple test to make sure that movz instructions don't parse as sbfms
    pub fn test_sbfm_parse_neg() -> anyhow::Result<()> {
        let opcode = disarm64::decoder::decode(0x52800088).expect("failed to decode instruction");
        let sbfm_test = parse(&opcode)?;
        assert!(sbfm_test.is_none(), "movz parsed as sbfm");
        Ok(())
    }
}
