use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_sbfiz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let lsb = Self::get_imm_val(&collected_args[2])?;
        let width = Self::get_imm_val(&collected_args[3])?;

        Ok(Box::new(SbfizInstruction { rd, rn, lsb, width }))
    }

#[derive(Clone)]
pub struct SbfizInstruction {
    rd: RegisterType,
    rn: RegisterType,
    lsb: i64,
    width: i64,
}

impl ExecutableInstruction for SbfizInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sbfiz(self.rd, self.rn, self.lsb, self.width)
    }
}

impl Core<'_, '_, '_> {
    pub fn sbfiz(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        lsb: i64,
        width: i64,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)? as u64;
        let lsb_val = lsb as u64;
        let width_val = width as u64;

        let mask = (1u64 << width_val) - 1;

        let extracted = (xn_val & mask) << lsb_val;
        let shift = 64 - (lsb_val + width_val);

        self.cpu
            .write_gen_reg(&xd, ((extracted << shift) >> shift) as i64)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_sbfiz_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov x1, #0x000000000000007F"))?;
    core.handle_string_command(&String::from("sbfiz x2, x1, #16, #8"))?;
    //Makes this into 0x00000000007F0000
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(2))?, 8323072);
    Ok(())
}
