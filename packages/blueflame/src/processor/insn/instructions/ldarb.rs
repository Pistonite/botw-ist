use crate::processor::{self as self_, crate_};

use disarm64::arm64::InsnOpcode;
use disarm64::decoder::{Mnemonic, Opcode};

use crate_::memory::Ptr;

use self_::insn::Core;
use self_::insn::instruction_parse::{ExecutableInstruction, get_bit_range};
use self_::{Error, RegisterType, glue};

#[derive(Clone)]
pub struct InsnLdarb {
    rt: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for InsnLdarb {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let addr = glue::read_gen_reg(core.cpu, &self.rn) as u64;
        let ptr = Ptr!(<u8>(addr));
        let res_val = ptr.load(&core.proc.memory())?;
        glue::write_gen_reg(core.cpu, &self.rt, res_val as i64);
        Ok(())
    }
}

pub fn parse(d: &Opcode) -> Result<Option<Box<(dyn ExecutableInstruction)>>, Error> {
    if d.mnemonic != Mnemonic::ldarb {
        return Ok(None);
    }
    let bits = d.operation.bits();
    let rt_idx = get_bit_range(bits, 4, 0);
    let rn_idx = get_bit_range(bits, 9, 5);
    let rt = RegisterType::WReg(rt_idx);
    let rn = RegisterType::XReg(rn_idx);
    Ok(Some(Box::new(InsnLdarb { rt, rn })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use disarm64::decoder::decode;

    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn test_ldarb_parse() -> anyhow::Result<()> {
        let opcode = decode(0x08DFFC20).expect("failed to decode instruction");
        let insn = parse(&opcode)?.expect("failed to parse ldar instruction");
        let mut cpu = Cpu0::default();
        cpu.write(reg!(w[1]), 0x1000);
        let mut proc = Process::new_for_test();
        Ptr!(<u8>(0x1000)).store(&42, &mut proc.memory_mut())?;
        let mut core = Core::new(&mut cpu, &mut proc);
        insn.exec_on(&mut core)?;
        assert_eq!(cpu.read::<u32>(reg!(w[0])), 42);
        Ok(())
    }
}
