use crate::processor::{self as self_};

use crate::memory::Ptr;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

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
            return Some(Box::new(LdpInstruction {
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
        Some(Box::new(LdpPreInstruction {
            rt1,
            rt2,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else if collected_args[2].contains("], ") {
        Some(Box::new(LdpPostInstruction {
            rt1,
            rt2,
            rn_sp,
            imm_val,
            extra_op,
        }))
    } else {
        Some(Box::new(LdpImmInstruction {
            rt1,
            rt2,
            rn_sp,
            imm_val,
            extra_op,
        }))
    }
}

#[derive(Clone)]
pub struct LdpPostInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

fn ldp_core(
    core: &mut Core,
    rt1: RegisterType,
    rt2: RegisterType,
    address: u64,
) -> Result<(), Error> {
    let loaded_val1: i64 = match rt1 {
        RegisterType::XReg(_) => Ptr!(<i64>(address)).load(core.proc.memory())?,
        RegisterType::WReg(_) => Ptr!(<i32>(address)).load(core.proc.memory())? as i64,
        RegisterType::SReg(_) => Ptr!(<i32>(address)).load(core.proc.memory())? as i64,
        RegisterType::DReg(_) => Ptr!(<i64>(address)).load(core.proc.memory())?,
        _ => {
            log::error!("Loading into non-general register type: {:?}", rt1);
            return Err(Error::BadInstruction(0));
        }
    };
    let loaded_val2: i64 = match rt2 {
        RegisterType::XReg(_) => Ptr!(<i64>(address + 8)).load(core.proc.memory())?,
        RegisterType::WReg(_) => Ptr!(<i32>(address + 4)).load(core.proc.memory())? as i64,
        RegisterType::SReg(_) => Ptr!(<i32>(address + 4)).load(core.proc.memory())? as i64,
        RegisterType::DReg(_) => Ptr!(<i64>(address + 8)).load(core.proc.memory())?,
        _ => {
            log::error!("Loading into non-general register type: {:?}", rt2);
            return Err(Error::BadInstruction(0));
        }
    };
    glue::write_gen_reg(core.cpu, &rt1, loaded_val1);
    glue::write_gen_reg(core.cpu, &rt2, loaded_val2);
    Ok(())
}

impl ExecutableInstruction for LdpPostInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        ldp_core(core, self.rt1, self.rt2, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        glue::write_gen_reg(core.cpu, &self.rn_sp, new_reg_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct LdpImmInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdpImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let mem_to_read: i64 = xn_sp_val + imm_val;
        ldp_core(core, self.rt1, self.rt2, mem_to_read as u64)
    }
}

#[derive(Clone)]
pub struct LdpPreInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdpPreInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn_sp,
            self.extra_op.as_ref(),
        )?;
        let mem_to_read: i64 = xn_sp_val + imm_val;
        ldp_core(core, self.rt1, self.rt2, mem_to_read as u64)?;
        glue::write_gen_reg(core.cpu, &self.rn_sp, mem_to_read);
        Ok(())
    }
}

#[derive(Clone)]
pub struct LdpInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdpInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_sp_val: i64 = glue::read_gen_reg(core.cpu, &self.rn_sp);
        let (xm_val, _) = glue::handle_extra_op(
            core.cpu,
            glue::read_gen_reg(core.cpu, &self.rm),
            self.rm,
            self.rm.get_bitwidth(),
            self.extra_op.as_ref(),
        )?;

        let mem_to_read = xn_sp_val + xm_val;
        ldp_core(core, self.rt1, self.rt2, mem_to_read as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{reg, Cpu0, Process};

    #[test]
    pub fn simple_ldp_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        Ptr!(<i32>(32)).store(&1234, &mut proc.memory_mut())?;
        Ptr!(<i32>(36)).store(&5678, &mut proc.memory_mut())?;
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("add w0, wzr, #32")?;
        core.handle_string_command("ldp w1, w2, [w0]")?;
        assert_eq!(cpu.read::<i32>(reg!(w[1])), 1234);
        assert_eq!(cpu.read::<i32>(reg!(w[2])), 5678);
        Ok(())
    }
}
