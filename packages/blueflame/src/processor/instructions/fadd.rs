use super::super::instruction_parse::InsnParser;
use crate::processor::instruction_registry::{ExecutableInstruction, RegisterType};

use crate::processor::{Error, RegisterValue};
use crate::Core;

use disarm64_defn::defn::InsnOpcode;

use super::super::instruction_parse::get_bit_range;
use anyhow::bail;

#[derive(Clone)]
pub struct InsnFadd {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for InsnFadd {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        let rn_val = proc.cpu.read_reg(&self.rn);
        let rm_val = proc.cpu.read_reg(&self.rm);
        match (rn_val, rm_val) {
            (RegisterValue::SReg(rn), RegisterValue::SReg(rm)) => {
                proc.cpu.write_reg(&self.rd, &RegisterValue::SReg(rn + rm))
            }
            (RegisterValue::DReg(rn), RegisterValue::DReg(rm)) => {
                proc.cpu.write_reg(&self.rd, &RegisterValue::DReg(rn + rm))
            }
            _ => Ok(()),
        }
    }
}

impl InsnParser for InsnFadd {
    fn parse_from_decode(
        d: &disarm64::Opcode,
    ) -> std::result::Result<Option<Box<(dyn ExecutableInstruction)>>, anyhow::Error> {
        if d.mnemonic != disarm64::decoder_full::Mnemonic::fadd {
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
            _ => bail!("Invalid sf value for fadd"),
        };
        Ok(Some(Box::new(InsnFadd {
            rd: reg_type(rd_idx),
            rn: reg_type(rn_idx),
            rm: reg_type(rm_idx),
        })))
    }
}

#[cfg(test)]
use anyhow::Result;
#[test]
pub fn test_fsub_parse() -> Result<()> {
    // `fadd d0, d1, d2`: 0x1E622820
    let fadd_test =
        InsnFadd::parse_from_decode(&disarm64::decoder::decode(0x1E622820).unwrap())?.unwrap();

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();

    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };

    // Set D1 = 5.5, D2 = 2.0, so result in D0 should be 7.5
    core.cpu
        .write_reg(&RegisterType::DReg(1), &RegisterValue::DReg(5.5))?;
    core.cpu
        .write_reg(&RegisterType::DReg(2), &RegisterValue::DReg(2.0))?;

    fadd_test.exec_on(&mut core)?;

    let result = core.cpu.read_float_reg(&RegisterType::DReg(0))?;
    assert!((result - 7.5).abs() < 1e-6, "Expected 7.5, got {}", result);

    Ok(())
}
