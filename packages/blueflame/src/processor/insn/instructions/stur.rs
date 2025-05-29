use crate::processor::{self as self_, crate_};

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error};
use crate_::memory::Ptr;

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
            return Some(Box::new(SturInstruction {
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
        Some(Box::new(SturPreInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else if collected_args[1].contains("], ") {
        Some(Box::new(SturPostInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else {
        Some(Box::new(SturImmInstruction {
            rt,
            rn_sp,
            imm_val,
            extra_op,
        }))
    }
}

fn stur_core(core: &mut Core, xt: RegisterType, address: u64) -> Result<(), Error> {
    // Implements core of str, interfacing w/ memory
    match xt {
        RegisterType::XReg(_) => {
            let xt_val = glue::read_gen_reg(core.cpu, &xt);
            Ptr!(<i64>(address)).store(&xt_val, core.proc.memory_mut())?;
            Ok(())
        }
        RegisterType::WReg(_) => {
            let xt_val = glue::read_gen_reg(core.cpu, &xt) as i32;
            Ptr!(<i32>(address)).store(&xt_val, core.proc.memory_mut())?;
            Ok(())
        }
        RegisterType::WZR => {
            Ptr!(<i32>(address)).store(&0, core.proc.memory_mut())?;
            Ok(())
        }
        RegisterType::XZR => {
            Ptr!(<i64>(address)).store(&0, core.proc.memory_mut())?;
            Ok(())
        }
        RegisterType::SReg(_) => {
            let xt_val = glue::read_float_reg(core.cpu, &xt) as f32;
            Ptr!(<f32>(address)).store(&xt_val, core.proc.memory_mut())?;
            Ok(())
        }
        RegisterType::DReg(_) => {
            let xt_val = glue::read_float_reg(core.cpu, &xt);
            Ptr!(<f64>(address)).store(&xt_val, core.proc.memory_mut())?;
            Ok(())
        }
        _ => {
            log::error!("Invalid register write: {xt:?} at address {address:#016x}");
            Err(Error::BadInstruction(0))
        }
    }
}

#[derive(Clone)]
pub struct SturInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturInstruction {
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
        stur_core(core, self.rt, memory_to_read as u64)
    }
}

#[derive(Clone)]
pub struct SturPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturPreInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read = xn_sp_val + imm_val;
        stur_core(core, self.rt, memory_to_read as u64)?;
        glue::write_gen_reg(core.cpu, &self.rn_sp, memory_to_read);
        Ok(())
    }
}

#[derive(Clone)]
pub struct SturPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturPostInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        stur_core(core, self.rt, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        glue::write_gen_reg(core.cpu, &self.rn_sp, new_reg_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct SturImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        stur_core(core, self.rt, memory_to_read as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_stur_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w0, #32")?;
        core.handle_string_command("str w0, [w0]")?;
        assert_eq!(Ptr!(<i32>(32)).load(proc.memory())?, 32);
        Ok(())
    }
}

