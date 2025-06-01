use crate::processor::{self as self_};

use crate::memory::Ptr;
use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args: Vec<String> = parse::split_args(args, 2);
    let split_second: Vec<String> = parse::split_bracket_args(&collected_args[1]);
    let rt = glue::parse_reg_or_panic(&collected_args[0]);
    let rn_sp = glue::parse_reg_or_panic(&split_second[0]);
    let extra_op = parse::parse_auxiliary(split_second.get(2))?;
    let imm_val = if let Some(val) = split_second.get(1) {
        if val.starts_with('#') {
            parse::get_imm_val(val)?
        } else {
            let rm = glue::parse_reg_or_panic(val);
            return Some(Box::new(StrhInstruction {
                rt,
                rn_sp,
                rm,
                extra_op,
            }));
        }
    } else {
        0
    };
    if parse::ends_with_exclam(&collected_args[1]) {
        Some(Box::new(StrhPreInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else if collected_args[1].contains("], ") {
        Some(Box::new(StrhPostInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else {
        Some(Box::new(StrhImmInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    }
}

fn strh_core(core: &mut Core, xt: RegisterType, address: u64) -> Result<(), Error> {
    let halfword_val = glue::read_gen_reg(core.cpu, &xt) as u16;
    Ptr!(<u16>(address)).store(&halfword_val, core.proc.memory_mut())?;
    Ok(())
}

#[derive(Clone)]
pub struct StrhInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (xm_val, _) = glue::handle_extra_op(
            core.cpu,
            glue::read_gen_reg(core.cpu, &self.rm),
            self.rm,
            self.rm.get_bitwidth(),
            self.extra_op.as_ref(),
        )?;

        let memory_to_read = xn_sp_val + xm_val;
        strh_core(core, self.rt, memory_to_read as u64)
    }
}

#[derive(Clone)]
pub struct StrhPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhPreInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read = xn_sp_val + imm_val;
        strh_core(core, self.rt, memory_to_read as u64)?;
        glue::write_gen_reg(core.cpu, &self.rn_sp, memory_to_read);
        Ok(())
    }
}

#[derive(Clone)]
pub struct StrhPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhPostInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        strh_core(core, self.rt, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        glue::write_gen_reg(core.cpu, &self.rn_sp, new_reg_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct StrhImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        strh_core(core, self.rt, memory_to_read as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process};

    #[test]
    pub fn simple_strh_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x0, #0x44332211")?;
        core.handle_string_command("mov x1, #32")?;
        core.handle_string_command("strh x0, [x1]")?;
        //Test that 0x2211 gets stored in memory here
        assert_eq!(Ptr!(<i32>(32)).load(proc.memory())?, 8721);
        Ok(())
    }
}
