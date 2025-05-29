use crate::processor::{self as self_, crate_};

use crate_::memory::Ptr;
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
            return Some(Box::new(LdurInstruction {
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
        Some(Box::new(LdurPreInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else if collected_args[1].contains("], ") {
        Some(Box::new(LdurPostInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else {
        Some(Box::new(LdurImmInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    }
}

fn ldur_core(core: &mut Core, xd: RegisterType, address: u64) -> Result<(), Error> {
    // Implements core of ldur, interfacing w/ memory
    let loaded_val: i64 = match xd {
        RegisterType::XReg(_) => Ptr!(<i64>(address)).load(core.proc.memory())?,
        RegisterType::WReg(_) => Ptr!(<i32>(address)).load(core.proc.memory())? as i64,
        RegisterType::SReg(_) => {
            let val = Ptr!(<f32>(address)).load(core.proc.memory())?;
            glue::write_float_reg(core.cpu, &xd, val as f64);
            return Ok(());
        }
        RegisterType::DReg(_) => {
            let val = Ptr!(<f64>(address)).load(core.proc.memory())?;
            glue::write_float_reg(core.cpu, &xd, val);
            return Ok(());
        }
        _ => {
            log::error!("Invalid register type for ldur: {:?}", xd);
            return Err(Error::BadInstruction(0));
        }
    };
    glue::write_gen_reg(core.cpu, &xd, loaded_val);
    Ok(())
}

#[derive(Clone)]
pub struct LdurInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurInstruction {
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
        ldur_core(core, self.rt, memory_to_read as u64)
    }
}

#[derive(Clone)]
pub struct LdurPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurPreInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read = xn_sp_val + imm_val;
        ldur_core(core, self.rt, memory_to_read as u64)?;
        glue::write_gen_reg(core.cpu, &self.rn_sp, memory_to_read);
        Ok(())
    }
}

#[derive(Clone)]
pub struct LdurPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurPostInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        ldur_core(core, self.rt, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        glue::write_gen_reg(core.cpu, &self.rn_sp, new_reg_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct LdurImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        ldur_core(core, self.rt, memory_to_read as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_ldur_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        Ptr!(<i32>(32)).store(&1234, &mut proc.memory_mut())?;
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("add w0, wzr, #32")?;
        core.handle_string_command("ldur w1, [w0]")?;
        assert_eq!(cpu.read::<i32>(reg!(w[1])), 1234);
        Ok(())
    }
}
