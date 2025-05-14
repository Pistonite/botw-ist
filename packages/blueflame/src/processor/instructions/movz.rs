use super::super::instruction_parse::InsnParser;
use crate::processor::instruction_registry::{ExecutableInstruction, RegisterType};

use crate::processor::Error;
use crate::Core;

use disarm64_defn::defn::InsnOpcode;

use super::super::instruction_parse::get_bit_range;
use anyhow::bail;

#[derive(Clone)]
pub struct InsnMovz {
    rd: RegisterType,
    imm: u16,
    shift: u16,
}

impl ExecutableInstruction for InsnMovz {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        let res_val = (self.imm as i64) << self.shift;
        proc.cpu.write_gen_reg(&self.rd, res_val)?;
        Ok(())
    }
}
impl InsnParser for InsnMovz {
    fn parse_from_decode(
        d: &disarm64::Opcode,
    ) -> std::result::Result<Option<Box<(dyn ExecutableInstruction)>>, anyhow::Error> {
        if d.mnemonic != disarm64::decoder_full::Mnemonic::movz {
            return Ok(None);
        }
        let bits = d.operation.bits();
        let sf = get_bit_range(bits, 31, 31);
        let rd_idx = get_bit_range(bits, 4, 0);
        let hw = get_bit_range(bits, 22, 21);
        let rd = match sf {
            0 => RegisterType::WReg(rd_idx),
            1 => RegisterType::XReg(rd_idx),
            _ => bail!("Invalid decode value for sf in movz inst"),
        };
        let imm = get_bit_range(bits, 20, 5);
        let shift = hw * 16;
        Ok(Some(Box::new(InsnMovz {
            rd,
            imm: imm as u16,
            shift: shift as u16,
        })))
    }
}

#[cfg(test)]
use anyhow::Result;
#[test]
pub fn test_movz_parse() -> Result<()> {
    let movz_test =
        InsnMovz::parse_from_decode(&disarm64::decoder::decode(0x52800088).unwrap())?.unwrap();
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    movz_test.exec_on(&mut core)?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(8))?, 4);
    Ok(())
}
