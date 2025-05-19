use super::super::instruction_parse::InsnParser;
use crate::processor::instruction_registry::{ExecutableInstruction, RegisterType};

use crate::processor::Error;
use crate::Core;

use disarm64_defn::defn::InsnOpcode;

use super::super::instruction_parse::get_bit_range;
use anyhow::bail;

#[derive(Clone)]
pub struct InsnLsrv {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for InsnLsrv {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        let rn_val = proc.cpu.read_gen_reg(&self.rn)? as u64;
        let shift_val = (proc.cpu.read_gen_reg(&self.rm)? as u64) & 0x3F;
        let res_val = rn_val >> shift_val;
        proc.cpu.write_gen_reg(&self.rd, res_val as i64)?;
        Ok(())
    }
}

impl InsnParser for InsnLsrv {
    fn parse_from_decode(
        d: &disarm64::Opcode,
    ) -> std::result::Result<Option<Box<(dyn ExecutableInstruction)>>, anyhow::Error> {
        if d.mnemonic != disarm64::decoder_full::Mnemonic::lsrv {
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
            _ => bail!("Invalid decode value for sf in lsrv inst"),
        };
        let rn = match sf {
            0 => RegisterType::WReg(rn_idx),
            1 => RegisterType::XReg(rn_idx),
            _ => bail!("Invalid decode value for sf in lsrv inst"),
        };
        let rm = match sf {
            0 => RegisterType::WReg(rm_idx),
            1 => RegisterType::XReg(rm_idx),
            _ => bail!("Invalid decode value for sf in lsrv inst"),
        };
        Ok(Some(Box::new(InsnLsrv { rd, rn, rm })))
    }
}

#[cfg(test)]
use anyhow::Result;
#[test]
pub fn test_lsrv_parse() -> Result<()> {
    let lsrv_test =
        InsnLsrv::parse_from_decode(&disarm64::decoder::decode(0x1AC32441).unwrap())?.unwrap();
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.write_gen_reg(&RegisterType::WReg(2), 8)?;
    core.cpu.write_gen_reg(&RegisterType::WReg(3), 3)?;
    lsrv_test.exec_on(&mut core)?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(1))?, 1);
    Ok(())
}
