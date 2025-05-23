pub mod arithmetic_utils;
pub mod conditional_checker;
pub mod instruction_registry;
pub mod instructions;

pub mod instruction_parse;

use crate::memory::Memory;
use crate::memory::Proxies;
use crate::Core;
use anyhow::Result;
use instruction_registry::ExecutableInstruction;
use instruction_registry::{AuxiliaryOperation, RegisterType};
use std::collections::HashMap;
use std::panic::UnwindSafe;
use std::sync::Arc;
use std::sync::Mutex;

pub type RegIndex = u32;

type StubCondFunction = dyn Fn(&Processor) -> Result<bool> + Send +UnwindSafe+ 'static;
type StubFunction = dyn Fn(&mut Core) -> Result<()> + Send + UnwindSafe + 'static;
pub struct Stub {
    pub condition: Option<Box<StubCondFunction>>,
    pub func: Box<StubFunction>,
}
impl Stub {
    pub fn simple(func: Box<StubFunction>) -> Self {
        Stub {
            condition: None,
            func,
        }
    }

    pub fn skip() -> Self {
        Stub {
            condition: None,
            func: Box::new(|_| Ok(())),
        }
    }

    pub fn run_and_ret(func: Box<StubFunction>) -> Self {
        Stub {
            condition: None,
            func: Box::new(move |core| {
                let result = func(core);
                core.ret();
                result
            }),
        }
    }

    pub fn ret() -> Self {
        Stub {
            condition: None,
            func: Box::new(move |core| {
                core.ret();
                Ok(())
            }),
        }
    }
}

type InstructionBlock = Vec<Option<Box<dyn ExecutableInstruction>>>;


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


impl Processor {
    pub fn new() -> Self {
        let mut p = Processor {
            x: [0; 31],
            s: [0.0; 32],
            sp_el0: 0,
            _sp_el1: 0,
            _sp_el2: 0,
            _sp_el3: 0,
            _elr_el1: 0,
            _elr_el2: 0,
            _elr_el3: 0,
            _spsr_el1: 0,
            _spsr_el2: 0,
            _spsr_el3: 0,
            _fpscr: 0,
            pc: 0,
            flags: Flags::new(),
            stub_functions: HashMap::new(),
            stack_trace: Vec::new(),
            inst_cache: HashMap::new(),
        };
        p.init();
        p
    }


    pub fn read_arg(&self, i: RegIndex) -> i64 {
        let v = self.read_reg(&RegisterType::XReg(i));
        match v {
            RegisterValue::XReg(val) => val,
            _ => 0, // Should never get here
        }
    }

    pub fn write_arg(&mut self, i: RegIndex, val: u64) {
        // We know this will not result in an error
        self.write_gen_reg(&RegisterType::XReg(i), val as i64)
            .unwrap();
    }

    #[allow(arithmetic_overflow)]
    pub fn read_reg(&self, reg: &RegisterType) -> RegisterValue {
        match reg {
            RegisterType::XReg(idx) => RegisterValue::XReg(self.x[*idx as usize]),
            RegisterType::WReg(idx) => RegisterValue::WReg(self.x[*idx as usize] as i32),
            RegisterType::BReg(idx) => RegisterValue::BReg(self.s[*idx as usize]),
            RegisterType::HReg(idx) => RegisterValue::HReg(self.s[*idx as usize]),
            RegisterType::SReg(idx) => RegisterValue::SReg(self.s[*idx as usize] as f32),
            RegisterType::DReg(idx) => RegisterValue::DReg(self.s[*idx as usize]),
            //TODO: Fix QRegisters writing/shifting wrong
            RegisterType::QReg(idx) => RegisterValue::QReg(vec![
                self.s[*idx as usize],
                ((self.s[*idx as usize] as u32) << 8) as f64,
            ]),
            RegisterType::XZR => RegisterValue::XReg(0),
            RegisterType::WZR => RegisterValue::WReg(0),
            RegisterType::SP => RegisterValue::XReg(self.sp_el0 as i64),
            RegisterType::LR => RegisterValue::LR, //Return value
        }
    }

    pub fn write_reg(&mut self, reg: &RegisterType, val: &RegisterValue) -> Result<(), Error> {
        match (reg, val) {
            (RegisterType::XReg(idx), RegisterValue::XReg(v)) => {
                self.x[*idx as usize] = *v;
                Ok(())
            }
            (RegisterType::WReg(idx), RegisterValue::WReg(v)) => {
                self.x[*idx as usize] = *v as u32 as i64;
                Ok(())
            }
            (RegisterType::BReg(idx), RegisterValue::BReg(v)) => {
                self.s[*idx as usize] = *v;
                Ok(())
            }
            (RegisterType::HReg(idx), RegisterValue::HReg(v)) => {
                self.s[*idx as usize] = *v;
                Ok(())
            }
            (RegisterType::SReg(idx), RegisterValue::SReg(v)) => {
                self.s[*idx as usize] = *v as f64;
                Ok(())
            }
            (RegisterType::DReg(idx), RegisterValue::DReg(v)) => {
                self.s[*idx as usize] = *v;
                Ok(())
            }
            // (RegisterType::QReg(idx), RegisterValue::QReg(v)) => {
            //      self.s[*idx] = (((v[0] as u128) << 64) | (v[1] as u128)) as f32;
            // },
            (RegisterType::XZR, _) => Ok(()),
            (RegisterType::WZR, _) => Ok(()),
            (RegisterType::SP, RegisterValue::XReg(v)) => {
                self.sp_el0 = *v as u64;
                Ok(())
            }
            (RegisterType::LR, RegisterValue::XReg(v)) => {
                self.x[30] = *v;
                Ok(())
            }
            _ => Err(Error::InvalidRegisterWrite("any", *reg)),
        }
    }

    pub fn read_gen_reg(&self, reg: &RegisterType) -> Result<i64, Error> {
        let reg_val: RegisterValue = self.read_reg(reg);
        match reg_val {
            RegisterValue::XReg(v) => Ok(v),
            RegisterValue::WReg(v) => Ok(v as i64),
            RegisterValue::SReg(v) => Ok(i32::from_le_bytes(v.to_le_bytes()) as i64),
            RegisterValue::DReg(v) => Ok(i64::from_le_bytes(v.to_le_bytes())),
            _ => Err(Error::InvalidRegisterRead("general", *reg)),
        }
    }

    pub fn write_gen_reg(&mut self, reg: &RegisterType, val: i64) -> Result<(), Error> {
        match reg {
            RegisterType::XReg(_) => {
                self.write_reg(reg, &RegisterValue::XReg(val))?;
                Ok(())
            }
            RegisterType::WReg(_) => {
                self.write_reg(reg, &RegisterValue::WReg(val as i32))?;
                Ok(())
            }
            RegisterType::SP => {
                self.sp_el0 = val as u64;
                Ok(())
            }
            RegisterType::SReg(_) => {
                self.write_reg(
                    reg,
                    &RegisterValue::SReg(f32::from_bits(val as u32)),
                )?;
                Ok(())
            }
            RegisterType::WZR => Ok(()),
            RegisterType::XZR => Ok(()),
            _ => Err(Error::InvalidRegisterWrite("general", *reg)),
        }
    }

    pub fn write_float_reg(&mut self, reg: &RegisterType, val: f64) -> Result<(), Error> {
        match reg {
            RegisterType::SReg(_) => self.write_reg(reg, &RegisterValue::SReg(val as f32)),
            RegisterType::DReg(_) => self.write_reg(reg, &RegisterValue::DReg(val)),
            _ => Err(Error::InvalidRegisterWrite("float", *reg)),
        }
    }

    pub fn read_float_reg(&mut self, reg: &RegisterType) -> Result<f64, Error> {
        let reg_val: RegisterValue = self.read_reg(reg);
        match reg_val {
            RegisterValue::SReg(v) => Ok(v as f64),
            RegisterValue::DReg(v) => Ok(v),
            _ => Err(Error::InvalidRegisterRead("float", *reg)),
        }
    }

    // Returns the value and the carry bit
    pub fn handle_extra_op(
        &mut self,
        val: i64,
        val_size: RegisterType,
        bitwidth: u8,
        op: Option<AuxiliaryOperation>,
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
                            let result = ((val as u64) << extra_op.shift_val) as i64; // we essentially implicitly uxtw during conversion to u32.
                            Ok((result, false))
                        }
                        //the distinct signed/unsigned extend behavior between uxtw and lsl (technically sxtw)
                        _ => Err(Error::UnhandledExtraOp(extra_op.operation)),
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
                            let result = ((val as u64) << extra_op.shift_val) as i64; // we essentially implicitly uxtw during conversion to u64.
                            Ok((result, false))
                        }
                        //the distinct signed/unsigned extend behavior between uxtw and lsl (technically sxtw)
                        _ => Err(Error::UnhandledExtraOp(extra_op.operation)),
                    }
                } else {
                    Ok((val, false))
                }
            }
        }
    }

    pub fn handle_extra_op_unsigned(
        &mut self,
        val: u64,
        op: Option<AuxiliaryOperation>,
    ) -> Result<u64, Error> {
        if let Some(extra_op) = op {
            match extra_op.operation.as_str() {
                "sxtw" => {
                    if extra_op.shift_val == 0 {
                        Ok(val)
                    } else {
                        Ok(val << extra_op.shift_val)
                    }
                }
                "lsl" => Ok(val << extra_op.shift_val),
                "lsr" => Ok(val >> extra_op.shift_val),
                _ => Err(Error::UnhandledExtraOp(extra_op.operation)),
            }
        } else {
            Ok(val)
        }
    }

    /// Change the processor pc, currently unchecked
    pub fn set_pc(&mut self, new_pc: u64) {
        self.pc = new_pc;
    }

    pub fn check_pc(&self, main_offset: u32) -> Option<&Arc<Mutex<Stub>>> {
        self.stub_functions.get(&(self.pc - (main_offset as u64)))
    }

    pub fn register_stub_function(&mut self, addr: u64, stub: Stub) {
        // TODO: hardcode the program start now... need architecture change
        self.stub_functions
            .insert(addr + 0x1234500000, Arc::new(Mutex::new(stub)));
    }

    // pub fn handle_stub(&mut self, stub: &mut Stub, core: &mut Core<'_, '_, '_>) -> Result<()> {
    //     let condition_met = {
    //         let condition = &stub.condition; // Immutable borrow ends here
    //         if let Some(cond) = condition {
    //             (cond)(self)?
    //         } else {
    //             true
    //         }
    //     };
    //     if condition_met {
    //         let func = &mut stub.func;
    //         let _ = (*func)(core);
    //     }
    //     Ok(())
    // }
}
