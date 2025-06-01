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
            return Some(Box::new(StrbInstruction {
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
        Some(Box::new(StrbPreInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else if collected_args[1].contains("], ") {
        Some(Box::new(StrbPostInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else {
        Some(Box::new(StrbImmInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    }
}

fn strb_core(core: &mut Core, xt: RegisterType, address: u64) -> Result<(), Error> {
    // Implements core of strb, interfacing w/ memory
    let byte_val = glue::read_gen_reg(core.cpu, &xt) as u8;
    Ptr!(<u8>(address)).store(&byte_val, core.proc.memory_mut())?;
    Ok(())
}

#[derive(Clone)]
pub struct StrbInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbInstruction {
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
        strb_core(core, self.rt, memory_to_read as u64)
    }
}

#[derive(Clone)]
pub struct StrbPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbPreInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read = xn_sp_val + imm_val;
        strb_core(core, self.rt, memory_to_read as u64)?;
        glue::write_gen_reg(core.cpu, &self.rn_sp, memory_to_read);
        Ok(())
    }
}

#[derive(Clone)]
pub struct StrbPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbPostInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        strb_core(core, self.rt, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        glue::write_gen_reg(core.cpu, &self.rn_sp, new_reg_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct StrbImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        strb_core(core, self.rt, memory_to_read as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process};

    #[test]
    pub fn simple_strb_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x0, #0x44332211")?;
        core.handle_string_command("mov x1, #32")?;
        core.handle_string_command("strb x0, [x1]")?;
        //Test that 0x11 gets stored in memory here
        assert_eq!(Ptr!(<i32>(32)).load(proc.memory())?, 17);
        Ok(())
    }
}
