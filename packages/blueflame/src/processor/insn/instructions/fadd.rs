use crate::processor::{self as self_, crate_};

use disarm64::decoder::{Mnemonic, Opcode};
use disarm64::arm64::InsnOpcode;

use self_::insn::instruction_parse::{ExecutableInstruction, get_bit_range};
use self_::insn::Core;
use self_::{glue, Error, RegisterType, glue::RegisterValue};

#[derive(Clone)]
pub struct InsnFadd {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for InsnFadd {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let rn_val = glue::read_reg(core.cpu, &self.rn);
        let rm_val = glue::read_reg(core.cpu, &self.rm);
        match (rn_val, rm_val) {
            (RegisterValue::SReg(rn), RegisterValue::SReg(rm)) => {
                glue::write_reg(core.cpu, &self.rd, &RegisterValue::SReg(rn + rm))
            }
            (RegisterValue::DReg(rn), RegisterValue::DReg(rm)) => {
                glue::write_reg(core.cpu, &self.rd, &RegisterValue::DReg(rn + rm))
            }
            _ => {},
        }
        Ok(())
    }
}

pub fn parse(
    d: &Opcode,
) -> Result<Option<Box<(dyn ExecutableInstruction)>>, Error> {
    if d.mnemonic != Mnemonic::fadd {
        return Ok(None);
    }
    let bits = d.operation.bits();
    let sf = get_bit_range(bits, 22, 22);
    let rd_idx = get_bit_range(bits, 4, 0);
    let rn_idx = get_bit_range(bits, 9, 5);
    let rm_idx = get_bit_range(bits, 20, 16);
    let reg_type = match sf {
        0 => RegisterType::SReg,
        1 => RegisterType::DReg,
        _ => {
            log::error!("Invalid sf value for fadd: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    Ok(Some(Box::new(InsnFadd {
        rd: reg_type(rd_idx),
        rn: reg_type(rn_idx),
        rm: reg_type(rm_idx),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use disarm64::decoder::decode;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn test_fadd_parse() -> anyhow::Result<()> {
        let opcode = decode(0x1E622820).expect("failed to decode");
        let insn = parse(&opcode)?.expect("failed to parse");
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();

        // Set D1 = 5.5, D2 = 2.0, so result in D0 should be 7.5
        cpu.write(reg!(d[1]), 5.5f64);
        cpu.write(reg!(d[2]), 2.0f64);

        let mut core = Core::new(&mut cpu, &mut proc);
        insn.exec_on(&mut core)?;

        let result = cpu.read::<f64>(reg!(d[0]));
        assert_eq!(result, 7.5f64); // exact repr

        Ok(())
    }
}

