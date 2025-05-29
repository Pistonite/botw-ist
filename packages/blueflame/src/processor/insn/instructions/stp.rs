use crate::processor::{self as self_, crate_};

use crate_::memory::Ptr;
use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::reg;
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args: Vec<String> = parse::split_args(args, 3);
    let split_third: Vec<String> = parse::split_bracket_args(&collected_args[2]);
    let rt1 = glue::parse_reg_or_panic(&collected_args[0]);
    let rt2 = glue::parse_reg_or_panic(&collected_args[1]);
    let rn_sp = glue::parse_reg_or_panic(&split_third[0]);
    let extra_op = parse::parse_auxiliary(split_third.get(2))?;
    let imm_val = if let Some(val) = split_third.get(1) {
        if val.starts_with('#') {
            parse::get_imm_val(val)?
        } else {
            let rm = glue::parse_reg_or_panic(val);
            return Some(Box::new(StpInstruction {
                rt1,
                rt2,
                rn_sp,
                rm,
                extra_op,
            }));
        }
    } else {
        0
    };
    if parse::ends_with_exclam(&collected_args[2]) {
        Some(Box::new(StpPreInstruction {
            rt1,
            rt2,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else if collected_args[2].contains("], ") {
        Some(Box::new(StpPostInstruction {
            rt1,
            rt2,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else {
        Some(Box::new(StpImmInstruction {
            rt1,
            rt2,
            rn_sp,
            imm_val,
            extra_op,
        }))
    }
}

fn stp_core(
    core: &mut Core,
    xt1: RegisterType,
    xt2: RegisterType,
    address: u64,
) -> Result<(), Error> {
    // Implements core of stp, interfacing w/ memory
    match xt1 {
        RegisterType::XReg(_) => {
            let xt_val = glue::read_gen_reg(core.cpu, &xt1);
            Ptr!(<i64>(address)).store(&xt_val, core.proc.memory_mut())?;
        }
        RegisterType::WReg(_) => {
            let xt_val = glue::read_gen_reg(core.cpu, &xt1);
            Ptr!(<i32>(address)).store(&(xt_val as i32), core.proc.memory_mut())?;
        }
        RegisterType::XZR => Ptr!(<i64>(address)).store(&0, core.proc.memory_mut())?,
        RegisterType::WZR => Ptr!(<i32>(address)).store(&0, core.proc.memory_mut())?,
        RegisterType::SReg(_) => {
            let xt_val = glue::read_float_reg(core.cpu, &xt1);
            Ptr!(<f32>(address)).store(&(xt_val as f32), core.proc.memory_mut())?;
        }
        RegisterType::QReg(i) => {
            let (lo, hi) = core.cpu.read::<(u64, u64)>(reg!(q[i]));
            Ptr!(<u64>(address)).store(&lo, core.proc.memory_mut())?;
            Ptr!(<u64>(address + 8)).store(&hi, core.proc.memory_mut())?;
        }
        _ => {
            log::error!("Invalid register write xt1: {:?}", xt1);
            return Err(Error::BadInstruction(0));
        }
    };
    match xt2 {
        RegisterType::XReg(_) => {
            let xt_val = glue::read_gen_reg(core.cpu, &xt2);
            Ptr!(<i64>(address + 8)).store(&xt_val, core.proc.memory_mut())?;
        }
        RegisterType::WReg(_) => {
            let xt_val = glue::read_gen_reg(core.cpu, &xt2);
            Ptr!(<i32>(address + 4)).store(&(xt_val as i32), core.proc.memory_mut())?;
        }
        RegisterType::XZR => Ptr!(<i64>(address + 8)).store(&0, core.proc.memory_mut())?,
        RegisterType::WZR => Ptr!(<i32>(address + 4)).store(&0, core.proc.memory_mut())?,
        RegisterType::SReg(_) => {
            let xt_val = glue::read_float_reg(core.cpu, &xt2);
            Ptr!(<f32>(address + 4)).store(&(xt_val as f32), core.proc.memory_mut())?;
        }
        RegisterType::QReg(i) => {
            let (lo, hi) = core.cpu.read::<(u64, u64)>(reg!(q[i]));
            Ptr!(<u64>(address + 16)).store(&lo, core.proc.memory_mut())?;
            Ptr!(<u64>(address + 24)).store(&hi, core.proc.memory_mut())?;
        }
        _ => {
            log::error!("Invalid register write xt2: {:?}", xt2);
            return Err(Error::BadInstruction(0));
        }
    };
    Ok(())
}

#[derive(Clone)]
pub struct StpInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpInstruction {
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
        stp_core(core, self.rt1, self.rt2, memory_to_read as u64)
    }
}

#[derive(Clone)]
pub struct StpPreInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpPreInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        stp_core(core, self.rt1, self.rt2, memory_to_read as u64)?;
        glue::write_gen_reg(core.cpu, &self.rn_sp, memory_to_read);
        Ok(())
    }
}

#[derive(Clone)]
pub struct StpPostInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpPostInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        stp_core(core, self.rt1, self.rt2, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        glue::write_gen_reg(core.cpu, &self.rn_sp, new_reg_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct StpImmInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        stp_core(core, self.rt1, self.rt2, memory_to_read as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_stp_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w0, #32")?;
        core.handle_string_command("mov w1, #64")?;
        core.handle_string_command("stp w0, w1, [w0]")?;
        assert_eq!(Ptr!(<i32>(32)).load(proc.memory())?, 32);
        assert_eq!(Ptr!(<i32>(36)).load(proc.memory())?, 64);
        Ok(())
    }
}
