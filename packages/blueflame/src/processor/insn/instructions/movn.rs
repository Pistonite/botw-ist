use crate::processor::{self as self_, crate_};

use disarm64::arm64::InsnOpcode;
use disarm64::decoder::{Mnemonic, Opcode};

use self_::insn::Core;
use self_::insn::instruction_parse::{ExecutableInstruction, get_bit_range};
use self_::{Error, RegisterType, glue};

#[derive(Clone)]
struct InsnMovn {
    rd: RegisterType,
    imm: u16,
    shift: u16,
}

impl ExecutableInstruction for InsnMovn {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let res_val = !(self.imm as i64) << self.shift;
        glue::write_gen_reg(core.cpu, &self.rd, res_val);
        Ok(())
    }
}
pub fn parse(d: &Opcode) -> Result<Option<Box<(dyn ExecutableInstruction)>>, Error> {
    if d.mnemonic != Mnemonic::movn {
        return Ok(None);
    }
    let bits = d.operation.bits();
    let sf = get_bit_range(bits, 31, 31);
    let rd_idx = get_bit_range(bits, 4, 0);
    let hw = get_bit_range(bits, 22, 21);
    let rd = match sf {
        0 => RegisterType::WReg(rd_idx),
        1 => RegisterType::XReg(rd_idx),
        _ => {
            log::error!("Invalid sf value in movz instruction: {sf}");
            return Err(Error::BadInstruction(bits));
        }
    };
    let imm = get_bit_range(bits, 20, 5);
    let shift = hw * 16;
    Ok(Some(Box::new(InsnMovn {
        rd,
        imm: imm as u16,
        shift: shift as u16,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};
    #[test]
    pub fn test_movn_parse() -> anyhow::Result<()> {
        let opcode = disarm64::decoder::decode(0x12800016).expect("failed to decode");
        let insn = parse(&opcode)?.unwrap();
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        insn.exec_on(&mut core)?;
        assert_eq!(cpu.read::<i32>(reg!(w[22])), -1);
        Ok(())
    }
}
