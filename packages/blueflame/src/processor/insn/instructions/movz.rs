use crate::processor::{self as self_};

use disarm64::arm64::InsnOpcode;
use disarm64::decoder::{Mnemonic, Opcode};

use self_::insn::Core;
use self_::insn::instruction_parse::{ExecutableInstruction, get_bit_range};
use self_::{Error, RegisterType, glue};

#[derive(Clone)]
struct InsnMovz {
    rd: RegisterType,
    imm: u16,
    shift: u16,
}

impl ExecutableInstruction for InsnMovz {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let res_val = (self.imm as i64) << self.shift;
        glue::write_gen_reg(core.cpu, &self.rd, res_val);
        Ok(())
    }
}
pub fn parse(d: &Opcode) -> Result<Option<Box<(dyn ExecutableInstruction)>>, Error> {
    if d.mnemonic != Mnemonic::movz {
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
    Ok(Some(Box::new(InsnMovz {
        rd,
        imm: imm as u16,
        shift: shift as u16,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    use disarm64::decoder::decode;
    use self_::{Cpu0, Process, insn::paste_insn, reg};

    #[test]
    pub fn test_movz_parse() -> anyhow::Result<()> {
        let opcode = decode(paste_insn!(88 00 80 52)).expect("failed to decode instruction");
        let insn = parse(&opcode)?.expect("failed to parse movz instruction");
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        insn.exec_on(&mut core)?;
        assert_eq!(cpu.read::<u32>(reg!(w[8])), 4);
        assert_eq!(cpu.pc, 0); // PC should not change
        Ok(())
    }
}
