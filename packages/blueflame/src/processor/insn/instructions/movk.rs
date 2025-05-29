use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error};

pub    fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 3);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let imm_val = parse::get_imm_val(&collected_args[1])? as u16;
    let extra_op = parse::parse_auxiliary(collected_args.get(2))?;
    Some(Box::new(MovkInstruction {
        rd,
        imm_val,
        extra_op,
    }))
}


#[derive(Clone)]
pub struct MovkInstruction {
    rd: RegisterType,
    imm_val: u16,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovkInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let imm_val = glue::handle_extra_op_unsigned(core.cpu, self.imm_val as u64, self.extra_op.as_ref());
        let xd_val = glue::read_gen_reg(core.cpu, &self.rd) as u64;
        let xd_top_bits = xd_val & 0xFFFF_FFFF_FFFF_0000;
        let imm_bottom_bits = imm_val & 0x0000_0000_0000_FFFF;

        glue::write_gen_reg(
            core.cpu,
            &self.rd,
            (xd_top_bits | imm_bottom_bits) as i64,
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};
    #[test]
    pub fn simple_movk_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        cpu.write(reg!(w[23]), 0x12345678);
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("movk w23, #0x4321")?;
        assert_eq!(cpu.read::<u32>(reg!(w[23])), 0x12344321);
        Ok(())
    }

    #[test]
    pub fn applied_movk_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w23, #0x40000")?;
        core.handle_string_command("movk w23, #0x4160")?;
        assert_eq!(cpu.read::<u32>(reg!(w[23])), 0x44160);
        Ok(())
    }
}
