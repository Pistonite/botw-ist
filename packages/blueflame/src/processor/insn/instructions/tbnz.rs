use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;

    fn parse_tbnz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let split = Self::split_args(args, 3);
        let rn = RegisterType::from_str(&split[0])?;
        let imm_val = Self::get_imm_val(&split[1])? as u64;
        let label_offset = Self::get_label_val(&split[2])?;
        Ok(Box::new(TbnzInstruction {
            rn,
            imm_val,
            label_offset,
        }))
    }

    fn parse_tbz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let split = Self::split_args(args, 3);
        let rn = RegisterType::from_str(&split[0])?;
        let imm_val = Self::get_imm_val(&split[1])? as u64;
        let label_offset = Self::get_label_val(&split[2])?;
        Ok(Box::new(TbzInstruction {
            rn,
            imm_val,
            label_offset,
        }))
    }

#[derive(Clone)]
pub struct TbnzInstruction {
    rn: RegisterType,
    imm_val: u64,
    label_offset: u64,
}

impl ExecutableInstruction for TbnzInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.tbnz(self.rn, self.imm_val, self.label_offset)
    }
}


impl Core<'_, '_, '_> {
    // Note: imm is the bit number. Should be between 0 and 63 (or 0 and 31 for W registers)
    pub fn tbnz(&mut self, xn: RegisterType, imm: u64, label_offset: u64) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;

        let bit_value = xn_val & (0b1 << imm);

        if bit_value != 0 {
            let new_pc = self.cpu.pc.wrapping_add_signed((label_offset - 4) as i64);
            self.cpu.pc = new_pc;
            // self.cpu.pc = label_offset - 4;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn tbnz_does_branch() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(30), 4)?;
    core.handle_string_command(&String::from("tbnz x30, 2, 50"))?;
    assert_eq!(core.cpu.pc, 0x1050);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn tbnz_does_not_branch() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(30), 4)?;
    core.handle_string_command(&String::from("tbnz x30, 0, 50"))?;
    assert_eq!(core.cpu.pc, 0x1004);
    Ok(())
}
