use crate::processor::{self as self_};

use anyhow::Result;
use disarm64::decoder::{self};
use std::panic::UnwindSafe;
use std::str::FromStr;

use self_::{Error, RegisterType};
use self_::insn::Core;

#[derive(Clone)]
pub struct AuxiliaryOperation {
    pub operation: String,
    pub shift_val: i64,
}

pub trait ExecutableInstruction: ExecutableInstructionClone + Send + Sync + UnwindSafe + 'static{
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error>;
    fn is_jump(&self) -> bool;
}

pub trait ExecutableInstructionClone {
    fn clone_box(&self) -> Box<dyn ExecutableInstruction>;
}

impl<T> ExecutableInstructionClone for T
where
    T: 'static + ExecutableInstruction + Clone,
{
    fn clone_box(&self) -> Box<dyn ExecutableInstruction> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ExecutableInstruction> {
    fn clone(&self) -> Box<dyn ExecutableInstruction> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct LegacyInstruction {
    pub inst: Box<dyn ExecutableInstruction>,
    pub condition: Option<String>,
}

impl LegacyInstruction {
    /// Splits instruction into instruction and args
    /// Returns tuple containing (instruction str, args str)
    fn split_insn(command: &str) -> (&str, &str) {
        if let Some(split) = command.split_once(char::is_whitespace) {
            split
        } else {
            (command, "")
        }
    }

    // fn get_inst_type<'a>(inst: &'a str) -> String {
    //     let data = Self::split_insn(inst);
    //     if let Some(split) = data.0.split_once('.') {
    //         String::from(split.0)
    //     } else {
    //         String::from(data.0)
    //     }
    // }

    fn split_args(args: &str, n: usize) -> Vec<String> {
        let split_args: std::str::SplitN<'_, &str> = args.splitn(n, ", ");
        split_args.map(|s| s.to_string()).collect()
    }

    fn split_bracket_args(bracketed: &str) -> Vec<String> {
        let binding = bracketed.replace(['[', ']'], "");
        let split_string = binding.splitn(3, ", ");
        split_string.map(|s| s.to_string()).collect()
    }

    fn ends_with_exclam(arg: &str) -> bool {
        arg.ends_with('!')
    }

    fn _convert_to_f32(input: &str) -> Result<f32> {
        let trimmed_input = input.trim_start_matches('#');
        Ok(trimmed_input.parse::<f32>()?)
    }
    fn convert_to_f64(input: &str) -> Result<f64> {
        let trimmed_input = input.trim_start_matches('#');
        Ok(trimmed_input.parse::<f64>()?)
    }

    fn get_imm_val(imm: &String) -> Result<i64> {
        let no_hash = if let Some(stripped) = imm.strip_prefix('#') {
            stripped
        } else {
            imm
        };

        let mult = if no_hash.starts_with('-') { -1 } else { 1 };
        let no_dash = no_hash.replace(['-', '!', ' ', '\t'], "");
        if let Some(hex) = no_dash.strip_prefix("0x") {
            Ok((u64::from_str_radix(hex, 16)? as i64) * mult)
        } else {
            Ok(no_dash.parse::<i64>()? * mult)
        }
    }

    fn get_label_val(imm: &str) -> Result<u64> {
        if let Some(stripped) = imm.strip_prefix("0x") {
            Ok(u64::from_str_radix(stripped, 16)?)
        } else {
            Ok(u64::from_str_radix(imm, 16)?)
        }
    }

    fn is_imm(str: &str) -> bool {
        str.starts_with('#') || str.starts_with("0x")
    }

    fn parse_auxiliary(fourth_arg: Option<&String>) -> Result<Option<AuxiliaryOperation>> {
        if let Some(arg) = fourth_arg {
            let split_arg: Vec<String> = arg
                .split(char::is_whitespace)
                .map(|s| s.to_string())
                .collect();
            let shift_val = if split_arg.len() > 1 {
                Self::get_imm_val(&split_arg[1])?
            } else {
                0
            };
            Ok(Some(AuxiliaryOperation {
                operation: split_arg.first().unwrap().to_string(),
                shift_val,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_instruction(opcode: &str, args: &str) -> Option<Box<dyn ExecutableInstruction>> {
        match opcode.to_lowercase().as_str() {
            "add" => panic!("todo"),
            // "adds" => Self::parse_adds(args),
            // "and" => Self::parse_and(args),
            // "ands" => Self::parse_ands(args),
            // "eor" => Self::parse_eor(args),
            // "madd" => Self::parse_madd(args),
            // "mov" => Self::parse_mov(args),
            // "movk" => Self::parse_movk(args),
            // "msub" => Self::parse_msub(args),
            // "mul" => Self::parse_mul(args),
            // "mvn" => Self::parse_mvn(args),
            // "neg" => Self::parse_neg(args),
            // "orn" => Self::parse_orn(args),
            // "orr" => Self::parse_orr(args),
            // "sub" => Self::parse_sub(args),
            // "subs" => Self::parse_subs(args),
            // "tst" => Self::parse_tst(args),
            // "asr" => Self::parse_asr(args),
            // "bfxil" => Self::parse_bfxil(args),
            // "lsl" => Self::parse_lsl(args),
            // "lsr" => Self::parse_lsr(args),
            // "sbfiz" => Self::parse_sbfiz(args),
            // "ccmp" => Self::parse_ccmp(args),
            // "cinc" => Self::parse_cinc(args),
            // "cmn" => Self::parse_cmn(args),
            // "cmp" => Self::parse_cmp(args),
            // "csel" => Self::parse_csel(args),
            // "cset" => Self::parse_cset(args),
            // "csinc" => Self::parse_csinc(args),
            // "csinv" => Self::parse_csinv(args),
            // "csneg" => Self::parse_csneg(args),
            // "fcmp" => Self::parse_fcmp(args),
            // "fcvtzs" => Self::parse_fcvtzs(args),
            // "fdiv" => Self::parse_fdiv(args),
            // "fmov" => Self::parse_fmov(args),
            // "fmul" => Self::parse_fmul(args),
            // "scvtf" => Self::parse_scvtf(args),
            // "smaddl" => Self::parse_smaddl(args),
            // "smull" => Self::parse_smull(args),
            // "b" => Self::parse_b(args),
            // "bic" => Self::parse_bic(args),
            // "bl" => Self::parse_bl(args),
            // "blr" => Self::parse_blr(args),
            // "br" => Self::parse_br(args),
            // "cbnz" => Self::parse_cbnz(args),
            // "cbz" => Self::parse_cbz(args),
            // "ret" => Self::parse_ret(args),
            // "tbnz" => Self::parse_tbnz(args),
            // "tbz" => Self::parse_tbz(args),
            // "ldp" => Self::parse_ldp(args),
            // "ldr" => Self::parse_ldr(args),
            // "ldrb" => Self::parse_ldrb(args),
            // "ldrsw" => Self::parse_ldrsw(args),
            // "ldur" => Self::parse_ldur(args),
            // "ldurb" => Self::parse_ldurb(args),
            // "ldrh" => Self::parse_ldrh(args),
            // "stp" => Self::parse_stp(args),
            // "str" => Self::parse_str(args),
            // "strb" => Self::parse_strb(args),
            // "strh" => Self::parse_strh(args),
            // "stur" => Self::parse_stur(args),
            // "sturb" => Self::parse_sturb(args),
            // "sturh" => Self::parse_sturh(args),
            // "sxtw" => Self::parse_sxtw(args),
            // "adrp" => Self::parse_adrp(args),
            // "ubfm" => Self::parse_ubfm(args),
            _ => {
                log::error!("unknown opcode when parsing instruction as string: {opcode}");
                None
            }
        }
    }
}

impl LegacyInstruction {
    pub fn from_u32(bits: u32) -> Result<Self, Error> {
        let Some(insn) = decoder::decode(bits) else {
            return Err(Error::BadInstruction(bits))
        };
        let insn_string = insn.to_string();
        let Some(insn) = Self::do_parse(
            &insn_string
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(" ")) else {
            log::error!("failed to parse instruction from as string: 0x{bits:08x}");
            return Err(Error::BadInstruction(bits));
        };

        Ok(insn)
    }
    fn do_parse(command: &str) -> Option<Self> {
        let command = command.trim();
        let mut data = Self::split_insn(command);
        let mut condition: Option<String> = None;
        if let Some(split) = data.0.split_once('.') {
            data.0 = split.0;
            condition = Some(String::from(split.1));
        }
        let inst = Self::parse_instruction(data.0, data.1)?;
        Some(LegacyInstruction { inst, condition })
    }
}

impl ExecutableInstruction for LegacyInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let should_run = match &self.condition {
            Some(cond) => core.cpu.check_condition(cond),
            None => true
        };
        if should_run {
            return self.inst.exec_on(core);
        }
        Ok(())
    }
    fn is_jump(&self) -> bool {
        self.inst.is_jump()
    }
}

pub fn core_get_inst_type(inst: &str) -> &str {
    let data = core_split_insn(inst);
    if let Some(split) = data.0.split_once('.') {
        split.0
    } else {
        data.0
    }
}
pub fn core_split_insn(command: &str) -> (&str, &str) {
    if let Some(split) = command.split_once(char::is_whitespace) {
        split
    } else {
        (command, "")
    }
}

impl Core<'_, '_> {


    pub fn handle_string_command_no_inc(&mut self, command: &str) -> Result<(), Error> {
        // 0 is placeholder, as we don't have the raw instruction here
        let inst = LegacyInstruction::do_parse(command)
            .ok_or( Error::BadInstruction(0)
            )?;
        inst.exec_on(self)?;
        Ok(())
    }

    pub fn handle_string_command(&mut self, command: &str) -> Result<(), Error> {
        // 0 is placeholder, as we don't have the raw instruction here
        let inst = LegacyInstruction::do_parse(command)
            .ok_or( Error::BadInstruction(0)
            )?;

        inst.exec_on(self)?;
        self.cpu.pc += 4;
        Ok(())
    }
}
