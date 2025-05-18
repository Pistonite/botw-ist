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
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type RegIndex = u32;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Unhandled extra-op: {0}")]
    UnhandledExtraOp(String),
    #[error("Unrecognized conditional code: {0}")]
    UnhandledConditionCode(String),
    #[error("Instruction could not be read at address {0:#0x}")]
    InstructionCouldNotBeRead(u64),
    #[error("Cannot read {0} value from register {1:?}")]
    InvalidRegisterRead(&'static str, RegisterType),
    #[error("Cannot write {0} value to register {1:?}")]
    InvalidRegisterWrite(&'static str, RegisterType),

    #[error("Memory error: {0}")]
    Mem(crate::memory::Error),
    #[error("Instruction emitted an error: {0}")]
    InstructionError(String),
    #[error("Unexpected: {0}")]
    Unexpected(String),
}

impl From<crate::memory::Error> for Error {
    fn from(err: crate::memory::Error) -> Self {
        Error::Mem(err)
    }
}

#[derive(Debug)]
pub struct Flags {
    // Condition flags
    pub n: bool, // Negative
    pub z: bool, // Zero
    pub c: bool, // Carry
    pub v: bool, // Overflow
}
type StubCondFunction = dyn Fn(&Processor) -> Result<bool>;
type StubFunction = dyn FnMut(&mut Core) -> Result<()>;
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

    pub fn run_and_ret(mut func: Box<StubFunction>) -> Self {
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

// https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Registers-in-AArch64-state
// #[allow(dead_code)]
pub struct Processor {
    // 31 general-purpose registers (called X0-X30 in code)
    // X30 is used as the link register (LR)
    pub x: [i64; 31],
    // 32 floating-point registers (called S0-S31 in code) - 128 bits each
    pub s: [f64; 32],
    // 4 stack pointer registers
    // https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Stack-Pointer-register?lang=en
    // sp_el0 is the classic "SP" register
    // The other 3 are stack pointers for each of the 3 exception levels
    // This is why the exception link/program status registers go from 1-3 rather than 0-2
    pub sp_el0: u64,
    _sp_el1: u64,
    _sp_el2: u64,
    _sp_el3: u64,
    // 3 exception link registers
    _elr_el1: i64,
    _elr_el2: i64,
    _elr_el3: i64,
    // 3 saved program status registers (these are 32 bits)
    _spsr_el1: i32,
    _spsr_el2: i32,
    _spsr_el3: i32,
    // Floating point status control register
    _fpscr: i32,
    // PC is not directly accessible to all instructions
    pub pc: u64,

    pub flags: Flags,

    // pub memory: Memory,
    pub stub_functions: HashMap<u64, Rc<RefCell<Stub>>>,

    // stores addresses of all functions visited
    // when function branched to, address is added (first entry is where the call was made from, second is where the branch was to)
    // when function returns, address at the back is removed
    pub stack_trace: Vec<(u64, u64)>,

    pub inst_cache: HashMap<u64, InstructionBlock>,
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

impl Default for Processor {
    fn default() -> Self {
        //Needs to not use init_memory, would cause infinite recursion
        Self::new()
    }
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

    fn init(&mut self) {
        Proxies::init_trigger_param_stubs(self);

        // memset_0 bl to memset
        self.register_stub_function(0x180026c, Stub::simple(Box::new(Self::memset)));
        // Some locking function
        self.register_stub_function(0x18001e0, Stub::ret());
        // Buffered safe string format
        self.register_stub_function(0xB0CE94, Stub::ret());
        // std::strcmp
        self.register_stub_function(0x1800760, Stub::simple(Box::new(Self::strcmp)));
        // InfoData::logFailure
        self.register_stub_function(0xD2E950, Stub::ret());
        // memcpy
        self.register_stub_function(0x494DB1C, Stub::simple(Box::new(Self::memcpy)));
        // memcpy_0
        self.register_stub_function(0x18001D0, Stub::simple(Box::new(Self::memcpy)));
        // Mutex fn
        self.register_stub_function(0x1800A1C, Stub::ret());
        // Mutex fn
        self.register_stub_function(0x1800A20, Stub::ret());
        // System Tick
        self.register_stub_function(0x1800270, Stub::ret());
        // Dummy Vec2f flag intializer
        self.register_stub_function(0xDF0D08, Stub::ret());

        // initForOpenWorldDemo
        self.register_stub_function(0x11F3364, Stub::simple(Box::new(Self::get_debug_heap)));
        self.register_stub_function(0x85456C, Stub::simple(Box::new(Self::get_actor)));

        // skip CreatePlayerActorEquipManager check in createPlayerEquipment
        self.register_stub_function(0x971540, Stub::skip());
        self.register_stub_function(0xAA81EC, Stub::skip());
        // skip check for actor
        self.register_stub_function(0x97166C, Stub::skip());

        // stub out doRequestCreateArmor
        self.register_stub_function(0x666CF8, Stub::ret());

        // stub out doRequestCreateWeapon
        self.register_stub_function(
            0x6669F8,
            Stub::simple(Box::new(Self::do_request_create_weapon)),
        );

        // skip Player::equipmentStuff
        self.register_stub_function(0x849580, Stub::ret());
    }

    fn do_request_create_weapon(core: &mut Core<'_, '_, '_>) -> Result<()> {
        let slot_idx = core.mem.mem_read_i32(core.cpu.read_arg(21) as u64 + 0x18)?;
        let value = core.cpu.read_arg(3) as i32;
        core.cpu.write_arg(0, core.mem.get_pmdm_addr());
        core.cpu.write_arg(1, value as u64);
        core.cpu.write_arg(2, slot_idx as u64);
        core.cpu
            .set_pc((0x971438 - 4 + core.mem.get_main_offset()).into());
        Ok(())
    }

    fn get_actor(core: &mut Core<'_, '_, '_>) -> Result<()> {
        core.cpu.write_arg(0, 0);
        core.ret();
        Ok(())
    }

    fn get_debug_heap(core: &mut Core<'_, '_, '_>) -> Result<()> {
        core.cpu.write_arg(0, 1);
        core.ret();
        Ok(())
    }

    /// Simulates memset
    fn memset(core: &mut Core<'_, '_, '_>) -> Result<()> {
        let s = core.cpu.read_arg(0) as u64;
        let c = core.cpu.read_arg(1) as u8;
        let n = core.cpu.read_arg(2) as u64;
        let mut writer = core.mem.write(s, None)?;
        for _ in 0..n {
            writer.write_u8(c)?;
        }
        core.ret();
        Ok(())
    }

    fn strcmp(core: &mut Core<'_, '_, '_>) -> Result<()> {
        let mut string_a_ptr = core.cpu.read_arg(0) as u64;
        let mut string_b_ptr = core.cpu.read_arg(1) as u64;
        let mut ret_val: i8;
        loop {
            let string_a_val = core.mem.mem_read_byte(string_a_ptr)?;
            let string_b_val = core.mem.mem_read_byte(string_b_ptr)?;
            ret_val = string_a_val as i8 - string_b_val as i8;
            if string_a_val != string_b_val || string_a_val == 0 {
                break;
            }
            string_a_ptr += 1;
            string_b_ptr += 1;
        }
        core.cpu
            .write_gen_reg(&RegisterType::XReg(0), ret_val as i64)?;
        core.ret();
        Ok(())
    }

    fn memcpy(core: &mut Core<'_, '_, '_>) -> Result<()> {
        let dest = core.cpu.read_arg(0) as u64;
        let src = core.cpu.read_arg(1) as u64;
        let num_bytes = core.cpu.read_arg(2) as usize;

        core.mem.memcpy(dest, src, num_bytes)?;

        core.cpu
            .write_gen_reg(&RegisterType::XReg(0), dest as i64)?;
        core.ret();
        Ok(())
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
                    &RegisterValue::SReg(f32::from_le_bytes((val as i32).to_le_bytes())),
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

    pub fn check_pc(&self, main_offset: u32) -> Option<&Rc<RefCell<Stub>>> {
        self.stub_functions.get(&(self.pc - (main_offset as u64)))
    }

    pub fn register_stub_function(&mut self, addr: u64, stub: Stub) {
        self.stub_functions
            .insert(addr, Rc::new(RefCell::new(stub)));
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

impl Processor {
    /// Attach the processor to a memory instance
    pub fn attach<'p, 'm, 'x>(
        &'p mut self,
        mem: &'m mut Memory,
        proxies: &'x mut Proxies,
    ) -> Core<'p, 'm, 'x> {
        self.write_gen_reg(
            &RegisterType::SP,
            mem.get_region(crate::memory::RegionType::Stack).get_end() as i64,
        )
        .unwrap(); // We know this will not result in an error
        Core {
            cpu: self,
            mem,
            proxies,
        }
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            n: false,
            z: false,
            c: false,
            v: false,
        }
    }

    pub fn from_nzcv(nzcv: u8) -> Self {
        Flags {
            n: (nzcv & 0b1000) != 0,
            z: (nzcv & 0b0100) != 0,
            c: (nzcv & 0b0010) != 0,
            v: (nzcv & 0b0001) != 0,
        }
    }
}
