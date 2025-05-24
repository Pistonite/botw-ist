use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_smaddl(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let wn = RegisterType::from_str(&collected_args[1])?;
        let wm = RegisterType::from_str(&collected_args[2])?;
        let ra = RegisterType::from_str(&collected_args[3])?;
        Ok(Box::new(SmaddlInstruction { rd, wn, wm, ra }))
    }


#[derive(Clone)]
pub struct SmaddlInstruction {
    rd: RegisterType,
    wn: RegisterType,
    wm: RegisterType,
    ra: RegisterType,
}

impl ExecutableInstruction for SmaddlInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.smaddl(self.rd, self.wn, self.wm, self.ra)
    }
}


impl Core<'_, '_, '_> {
    pub fn smaddl(
        &mut self,
        xd: RegisterType,
        wn: RegisterType,
        wm: RegisterType,
        xa: RegisterType,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&wn)?;
        let xm_val = self.cpu.read_gen_reg(&wm)?;
        let xa_val = self.cpu.read_gen_reg(&xa)?;
        self.cpu.write_gen_reg(&xd, xn_val * xm_val + xa_val)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_smaddl_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w1, #2"))?;
    core.handle_string_command(&String::from("mov w2, #3"))?;
    core.handle_string_command(&String::from("mov x3, #4"))?;
    core.handle_string_command(&String::from("smaddl x4, w1, w2, x3"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(4))?, 10);
    Ok(())
}
