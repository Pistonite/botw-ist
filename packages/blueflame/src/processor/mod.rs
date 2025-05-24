mod cpu;
pub use cpu::*;
mod error;
pub use error::*;
mod process;
pub use process::*;
mod register;
pub use register::*;

pub mod insn;

mod execute;

pub use execute::*;

mod stack_trace;
pub use stack_trace::*;

pub const BLOCK_COUNT_LIMIT: usize = 0x1000;
pub const BLOCK_ITERATION_LIMIT: usize = 0x400000;


// TODO --cleanup: remove this
pub mod glue {
    use std::str::FromStr;

    use super::*;

impl RegisterType {
    pub(crate) fn get_bitwidth(&self) -> u8 {
        match self {
            RegisterType::XReg(_) => 64,
            RegisterType::WReg(_) => 32,
            RegisterType::BReg(_) => 64,
            RegisterType::HReg(_) => 64,
            RegisterType::SReg(_) => 32,
            RegisterType::DReg(_) => 64,
            RegisterType::QReg(_) => 128,
            RegisterType::XZR => 64,
            RegisterType::WZR => 32,
            RegisterType::SP => 64,
            RegisterType::LR => 64,
        }
    }
}
impl FromStr for RegisterType {
    type Err = anyhow::Error;
    fn from_str(reg: &str) -> Result<Self, Self::Err> {
        match reg.to_lowercase().as_str() {
            "xzr" => return Ok(RegisterType::XZR),
            "wzr" => return Ok(RegisterType::WZR),
            "sp" => return Ok(RegisterType::SP),
            "lr" => return Ok(RegisterType::LR),
            _ => {}
        };
        let (reg_type, idx) = reg.split_at(1);
        let reg_num = idx.parse::<RegIndex>()?;
        match reg_type.to_lowercase().as_str() {
            "w" => Ok(RegisterType::WReg(reg_num)),
            "x" => Ok(RegisterType::XReg(reg_num)),
            "b" => Ok(RegisterType::BReg(reg_num)),
            "h" => Ok(RegisterType::HReg(reg_num)),
            "s" => Ok(RegisterType::SReg(reg_num)),
            "d" => Ok(RegisterType::DReg(reg_num)),
            "q" => Ok(RegisterType::QReg(reg_num)),
            _ => Err(anyhow::anyhow!("Unknown register type: {reg}")),
        }
    }
}
    #[derive(PartialEq, Debug)]
    pub enum RegisterValue {
        XReg(i64),
        WReg(i32),
        BReg(f64),
        HReg(f64),
        SReg(f32),
        DReg(f64),
        QReg(Vec<f64>),
        LR,
    }
    pub fn read_gen_reg(cpu: &Cpu0, reg: &RegisterType) -> i64 {
        let reg_val: RegisterValue = glue::read_reg(cpu, reg);
        match reg_val {
            RegisterValue::XReg(v) => v,
            RegisterValue::WReg(v) => v as i64,
            RegisterValue::SReg(v) => i32::from_le_bytes(v.to_le_bytes()) as i64,
            RegisterValue::DReg(v) => i64::from_le_bytes(v.to_le_bytes()),
            _ => {
                panic!("Invalid register read for general register: {reg:?}");
            }
        }
    }

    #[allow(arithmetic_overflow)]
    pub fn read_reg(cpu: &Cpu0, reg: &RegisterType) -> RegisterValue {
        match reg {
            RegisterType::XReg(idx) => RegisterValue::XReg(cpu.read(reg!(x[*idx]))),
            RegisterType::WReg(idx) => RegisterValue::WReg(cpu.read(reg!(w[*idx]))),
            RegisterType::BReg(idx) => RegisterValue::BReg(cpu.read(reg!(d[*idx]))),
            RegisterType::HReg(idx) => RegisterValue::HReg(cpu.read(reg!(d[*idx]))),
            RegisterType::SReg(idx) => RegisterValue::SReg(cpu.read(reg!(s[*idx]))),
            RegisterType::DReg(idx) => RegisterValue::DReg(cpu.read(reg!(d[*idx]))),
            RegisterType::QReg(idx) => {
                log::error!("QReg not implemented now, aborting");
                panic!("QReg not implemented now, aborting");
            }
            RegisterType::XZR => RegisterValue::XReg(0),
            RegisterType::WZR => RegisterValue::WReg(0),
            RegisterType::SP => RegisterValue::XReg(cpu.read(reg!(sp))),
            RegisterType::LR => RegisterValue::LR,
        }
    }

    pub fn write_gen_reg(cpu: &mut Cpu0, reg: &RegisterType, val: i64) {
        log::debug!("write_gen_reg: {reg:?} with value {val}");
        match reg {
            RegisterType::XReg(_) => {
                glue::write_reg(cpu, reg, &RegisterValue::XReg(val));
            }
            RegisterType::WReg(_) => {
                glue::write_reg(cpu, reg, &RegisterValue::WReg(val as i32));
            }
            RegisterType::SP => {
                glue::write_reg(cpu, reg, &RegisterValue::XReg(val));
            }
            RegisterType::SReg(_) => {
                glue::write_reg(
                    cpu,
                    reg,
                    &RegisterValue::SReg(f32::from_bits(val as u32)),
                );
            }
            RegisterType::WZR => {},
            RegisterType::XZR => {},
            _ => {
                panic!("Invalid register write for general register: {reg:?} with value {val}");
            }
        }
    }

    pub fn write_reg(cpu: &mut Cpu0, reg: &RegisterType, val: &RegisterValue) {
        log::debug!("write_reg: {reg:?} with value {val:?}");
        match (reg, val) {
            (RegisterType::XReg(idx), RegisterValue::XReg(v)) => {
                cpu.write(reg!(x[*idx]), *v);
            }
            (RegisterType::WReg(idx), RegisterValue::WReg(v)) => {
                cpu.write(reg!(w[*idx]), *v);
            }
            (RegisterType::BReg(idx), RegisterValue::BReg(v)) => {
                cpu.write(reg!(d[*idx]), *v);
            }
            (RegisterType::HReg(idx), RegisterValue::HReg(v)) => {
                cpu.write(reg!(d[*idx]), *v);
            }
            (RegisterType::SReg(idx), RegisterValue::SReg(v)) => {
                cpu.write(reg!(s[*idx]), *v);
            }
            (RegisterType::DReg(idx), RegisterValue::DReg(v)) => {
                cpu.write(reg!(d[*idx]), *v);
            }
            // (RegisterType::QReg(idx), RegisterValue::QReg(v)) => {
            //      self.s[*idx] = (((v[0] as u128) << 64) | (v[1] as u128)) as f32;
            // },
            (RegisterType::XZR, _) => {}
            (RegisterType::WZR, _) => {},
            (RegisterType::SP, RegisterValue::XReg(v)) => {
                cpu.write(reg!(sp), *v);
            }
            (RegisterType::LR, RegisterValue::XReg(v)) => {
                cpu.write(reg!(lr), *v);
            }
            _ =>  {
                panic!("Invalid register write: {reg:?} with value {val:?}");
            }
        }
    }
}
