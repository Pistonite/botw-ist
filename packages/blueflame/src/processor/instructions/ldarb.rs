use super::super::instruction_parse::InsnParser;
use crate::processor::instruction_registry::{ExecutableInstruction, RegisterType};

use crate::processor::Error;
use crate::Core;

use disarm64_defn::defn::InsnOpcode;

use super::super::instruction_parse::get_bit_range;

#[derive(Clone)]
pub struct InsnLdarb {
    rt: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for InsnLdarb {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        let addr = proc.cpu.read_gen_reg(&self.rn)? as u64;
        let res_val = proc.mem.mem_read_byte(addr)? as u64;
        proc.cpu.write_gen_reg(&self.rt, res_val as i64)?;
        Ok(())
    }
}

impl InsnParser for InsnLdarb {
    fn parse_from_decode(
        d: &disarm64::Opcode,
    ) -> std::result::Result<Option<Box<(dyn ExecutableInstruction)>>, anyhow::Error> {
        if d.mnemonic != disarm64::decoder_full::Mnemonic::ldarb {
            return Ok(None);
        }
        let bits = d.operation.bits();
        let rt_idx = get_bit_range(bits, 4, 0);
        let rn_idx = get_bit_range(bits, 9, 5);
        let rt = RegisterType::WReg(rt_idx);
        let rn = RegisterType::XReg(rn_idx);
        Ok(Some(Box::new(InsnLdarb { rt, rn })))
    }
}

#[cfg(test)]
use anyhow::Result;
#[test]
pub fn test_ldarb_parse() -> Result<()> {
    let ldar_test =
        InsnLdarb::parse_from_decode(&disarm64::decoder::decode(0x08DFFC20).unwrap())?.unwrap();
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.mem.mem_write_byte(0x1000, 42)?;
    core.cpu.write_gen_reg(&RegisterType::XReg(1), 0x1000)?;
    ldar_test.exec_on(&mut core)?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(0))?, 42);
    Ok(())
}
