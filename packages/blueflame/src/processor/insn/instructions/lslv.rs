use crate::processor::{self as self_};

use disarm64::arm64::InsnOpcode;
use disarm64::decoder::{Mnemonic, Opcode};

use self_::insn::instruction_parse::{get_bit_range, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

#[derive(Clone)]
pub struct InsnLslv {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for InsnLslv {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let rn_val = glue::read_gen_reg(core.cpu, &self.rn) as u64;
        let shift_val = (glue::read_gen_reg(core.cpu, &self.rm) as u64) & 0x3F;
        let res_val = rn_val << shift_val;
        glue::write_gen_reg(core.cpu, &self.rd, res_val as i64);
        Ok(())
    }
}

pub fn parse(d: &Opcode) -> Result<Option<Box<(dyn ExecutableInstruction)>>, Error> {
    if d.mnemonic != Mnemonic::lslv {
        return Ok(None);
    }
    let bits = d.operation.bits();
    let sf = get_bit_range(bits, 31, 31);
    let rd_idx = get_bit_range(bits, 4, 0);
    let rn_idx = get_bit_range(bits, 9, 5);
    let rm_idx = get_bit_range(bits, 20, 16);
    let rd = match sf {
        0 => RegisterType::WReg(rd_idx),
        1 => RegisterType::XReg(rd_idx),
        _ => {
            log::error!("Invalid sf value in lslv instruction: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    let rn = match sf {
        0 => RegisterType::WReg(rn_idx),
        1 => RegisterType::XReg(rn_idx),
        _ => {
            log::error!("Invalid sf value in lslv instruction: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    let rm = match sf {
        0 => RegisterType::WReg(rm_idx),
        1 => RegisterType::XReg(rm_idx),
        _ => {
            log::error!("Invalid sf value in lslv instruction: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    Ok(Some(Box::new(InsnLslv { rd, rn, rm })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use disarm64::decoder::decode;

    use self_::{reg, Cpu0, Process};

    #[test]
    fn test_lslv_parse() -> anyhow::Result<()> {
        let opcode = decode(0x9ac32041).expect("failed to decode");
        let insn = parse(&opcode)?.expect("failed to parse lslv instruction");
        let mut cpu = Cpu0::default();
        cpu.write(reg!(w[2]), 1);
        cpu.write(reg!(w[3]), 3);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        insn.exec_on(&mut core)?;
        assert_eq!(cpu.read::<u32>(reg!(w[1])), 8);
        Ok(())
    }
}
