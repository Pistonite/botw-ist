mod cpu;
pub use cpu::*;
mod error;
pub use error::*;
mod process;
pub use process::*;
mod register;
pub use register::*;
mod execute;
pub use execute::*;
mod hook;
pub use hook::*;
mod stack_trace;
pub use stack_trace::*;

pub mod insn;

pub const STACK_RESERVATION: u64 = 0x100;
pub const BLOCK_COUNT_LIMIT: usize = 0x10000;
pub const BLOCK_ITERATION_LIMIT: usize = 0x400000;

// TODO --cleanup: remove this
pub mod glue {

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

        pub fn to_regname(self) -> RegName {
            match self {
                RegisterType::XReg(i) => reg!(x[i]),
                RegisterType::WReg(i) => reg!(w[i]),
                RegisterType::BReg(_) => panic!("cannot convert b reg to reg name"),
                RegisterType::HReg(_) => panic!("cannot convert h reg to reg name"),
                RegisterType::SReg(i) => reg!(s[i]),
                RegisterType::DReg(i) => reg!(d[i]),
                RegisterType::QReg(i) => reg!(q[i]),
                RegisterType::XZR => reg!(xzr),
                RegisterType::WZR => reg!(wzr),
                RegisterType::SP => reg!(sp),
                RegisterType::LR => reg!(lr),
            }
        }
    }

    pub fn parse_reg_or_panic(reg: &str) -> RegisterType {
        match reg.to_lowercase().as_str() {
            "xzr" => return RegisterType::XZR,
            "wzr" => return RegisterType::WZR,
            "sp" => return RegisterType::SP,
            "lr" => return RegisterType::LR,
            _ => {}
        };
        let (reg_type, idx) = reg.split_at(1);
        let Ok(reg_num) = idx.parse::<RegIndex>() else {
            log::error!("Failed to parse register: {reg}, failed to parse index: {idx}");
            panic!("Invalid register format: {reg}");
        };
        match reg_type.to_lowercase().as_str() {
            "w" => RegisterType::WReg(reg_num),
            "x" => RegisterType::XReg(reg_num),
            "b" => RegisterType::BReg(reg_num),
            "h" => RegisterType::HReg(reg_num),
            "s" => RegisterType::SReg(reg_num),
            "d" => RegisterType::DReg(reg_num),
            "q" => RegisterType::QReg(reg_num),
            _ => {
                log::error!("Failed to parse register: {reg}");
                panic!("Unknown register type: {reg}");
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
            RegisterType::QReg(_) => {
                log::error!(
                    "QReg not implemented now, aborting, pc=0x{:016x}, reading {:?}",
                    cpu.pc,
                    reg
                );
                panic!("QReg not implemented now, aborting");
            }
            RegisterType::XZR => RegisterValue::XReg(0),
            RegisterType::WZR => RegisterValue::WReg(0),
            RegisterType::SP => RegisterValue::XReg(cpu.read(reg!(sp))),
            RegisterType::LR => RegisterValue::LR,
        }
    }

    pub fn write_gen_reg(cpu: &mut Cpu0, reg: &RegisterType, val: i64) {
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
                glue::write_reg(cpu, reg, &RegisterValue::SReg(f32::from_bits(val as u32)));
            }
            RegisterType::WZR => {}
            RegisterType::XZR => {}
            _ => {
                panic!("Invalid register write for general register: {reg:?} with value {val}");
            }
        }
    }

    pub fn write_reg(cpu: &mut Cpu0, reg: &RegisterType, val: &RegisterValue) {
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
            (RegisterType::WZR, _) => {}
            (RegisterType::SP, RegisterValue::XReg(v)) => {
                cpu.write(reg!(sp), *v);
            }
            (RegisterType::LR, RegisterValue::XReg(v)) => {
                cpu.write(reg!(lr), *v);
            }
            _ => {
                panic!("Invalid register write: {reg:?} with value {val:?}");
            }
        }
    }
    pub fn write_float_reg(cpu: &mut Cpu0, reg: &RegisterType, val: f64) {
        match reg {
            RegisterType::SReg(_) => glue::write_reg(cpu, reg, &RegisterValue::SReg(val as f32)),
            RegisterType::DReg(_) => glue::write_reg(cpu, reg, &RegisterValue::DReg(val)),
            _ => panic!("Invalid register write for float register: {reg:?} with value {val}"),
        }
    }

    pub fn read_float_reg(cpu: &Cpu0, reg: &RegisterType) -> f64 {
        let reg_val: RegisterValue = glue::read_reg(cpu, reg);
        match reg_val {
            RegisterValue::SReg(v) => v as f64,
            RegisterValue::DReg(v) => v,
            _ => panic!("Invalid register read for float register: {reg:?}"),
        }
    }
    pub fn handle_extra_op_immbw(
        cpu: &mut Cpu0,
        val: i64,
        val_size: RegisterType,
        op: Option<&insn::AuxiliaryOperation>,
    ) -> Result<(i64, bool), Error> {
        handle_extra_op(cpu, val, val_size, 32, op)
    }

    // Returns the value and the carry bit
    pub fn handle_extra_op(
        _cpu: &mut Cpu0,
        val: i64,
        val_size: RegisterType,
        bitwidth: u8,
        op: Option<&insn::AuxiliaryOperation>,
    ) -> Result<(i64, bool), Error> {
        match val_size {
            RegisterType::WReg(_) => {
                if let Some(extra_op) = op {
                    match extra_op.operation.as_str() {
                        "sxtw" => {
                            if extra_op.shift_val == 0 {
                                Ok((val, false))
                            } else {
                                let result = ((val as u64) << extra_op.shift_val) as i64;
                                let carry = ((val as u64) << (extra_op.shift_val - 1))
                                    & (1u64 << (bitwidth - 1));
                                Ok((result, carry > 0))
                            }
                        }
                        "lsl" => {
                            let result = ((val as u32) << extra_op.shift_val) as i64;
                            let carry = ((val as u32) << (extra_op.shift_val - 1))
                                & (1u32 << (bitwidth - 1));
                            Ok((result, carry > 0))
                        }
                        "lsr" => {
                            let result = (val as u32) >> extra_op.shift_val;
                            Ok((
                                result as i64,
                                result & (1u32 << (extra_op.shift_val - 1)) > 0,
                            ))
                        }
                        "asr" => {
                            // signed integers perform arithmetic shifts
                            let result = (val as i32) >> extra_op.shift_val;
                            let unsigned_result = u32::from_le_bytes(result.to_le_bytes());
                            Ok((
                                result as i64,
                                unsigned_result & (1u32 << (extra_op.shift_val - 1)) > 0,
                            ))
                        }
                        "uxtw" => {
                            let result = ((val as u32 as u64) << extra_op.shift_val) as i64;
                            Ok((result, false))
                        }
                        "uxtb" => {
                            let result = ((val as u8 as u64) << extra_op.shift_val) as i64;
                            Ok((result, false))
                        }
                        //the distinct signed/unsigned extend behavior between uxtw and lsl (technically sxtw)
                        _ => {
                            panic!(
                                "unhandled extra op: {}, shift={}",
                                extra_op.operation, extra_op.shift_val
                            );
                        }
                    }
                } else {
                    Ok((val, false))
                }
            }
            _ => {
                if let Some(extra_op) = op {
                    match extra_op.operation.as_str() {
                        "sxtw" => {
                            if extra_op.shift_val == 0 {
                                Ok((val, false))
                            } else {
                                let result = ((val as u64) << extra_op.shift_val) as i64;
                                let carry = ((val as u64) << (extra_op.shift_val - 1))
                                    & (1u64 << (bitwidth - 1));
                                Ok((result, carry > 0))
                            }
                        }
                        "lsl" => {
                            let result = ((val as u64) << extra_op.shift_val) as i64;
                            let carry = ((val as u64) << (extra_op.shift_val - 1))
                                & (1u64 << (bitwidth - 1));
                            Ok((result, carry > 0))
                        }
                        "lsr" => {
                            let result = (val as u64) >> extra_op.shift_val;
                            Ok((
                                result as i64,
                                result & (1u64 << (extra_op.shift_val - 1)) > 0,
                            ))
                        }
                        "asr" => {
                            // signed integers perform arithmetic shifts
                            let result = val >> extra_op.shift_val;
                            let unsigned_result = u64::from_le_bytes(result.to_le_bytes());
                            Ok((
                                result,
                                unsigned_result & (1u64 << (extra_op.shift_val - 1)) > 0,
                            ))
                        }
                        "uxtw" => {
                            let result = ((val as u32 as u64) << extra_op.shift_val) as i64; // we essentially implicitly uxtw during conversion to u64.
                            Ok((result, false))
                        }
                        "uxtb" => {
                            let result = ((val as u8 as u64) << extra_op.shift_val) as i64; // we essentially implicitly uxtw during conversion to u64.
                            Ok((result, false))
                        }
                        //the distinct signed/unsigned extend behavior between uxtw and lsl (technically sxtw)
                        _ => {
                            panic!(
                                "unhandled extra op: {}, shift={}",
                                extra_op.operation, extra_op.shift_val
                            );
                        }
                    }
                } else {
                    Ok((val, false))
                }
            }
        }
    }

    pub fn handle_extra_op_unsigned(
        _cpu: &mut Cpu0,
        val: u64,
        op: Option<&insn::AuxiliaryOperation>,
    ) -> u64 {
        if let Some(extra_op) = op {
            match extra_op.operation.as_str() {
                "sxtw" => {
                    if extra_op.shift_val == 0 {
                        val
                    } else {
                        val << extra_op.shift_val
                    }
                }
                "lsl" => val << extra_op.shift_val,
                "lsr" => val >> extra_op.shift_val,
                _ => {
                    panic!("unhandled extra op unsigned: {}", extra_op.operation);
                }
            }
        } else {
            val
        }
    }
}
