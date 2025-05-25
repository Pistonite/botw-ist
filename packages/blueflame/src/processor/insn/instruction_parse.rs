use crate::processor::{self as self_, crate_};

use disarm64::decoder::Opcode;
use disarm64::arm64::InsnOpcode;

use self_::Error;
use self_::insn::Core;

use super::instructions as xxx;

type ParseFn = fn(&Opcode) -> Result<Option<Box<dyn ExecutableInstruction>>, Error>;
static PARSE_LIST: &[ParseFn] = &[
    xxx::movz::parse,
    xxx::movn::parse,
    xxx::sbfm::parse,
    xxx::bfm::parse,
    xxx::lslv::parse,
    xxx::ldarb::parse,
    xxx::fsub::parse,
    xxx::fadd::parse,
    xxx::lsrv::parse,
];

pub const fn get_bit_range(bits: u32, start_idx: u8, end_idx: u8) -> u32 {
    // remove the bits that are before the start idx by moving them to the left, this uses the bitfield
    // convention where the MSB has index 31
    let truncate_start = bits << (31 - start_idx);
    truncate_start >> (31 - (start_idx - end_idx))
}

pub const fn get_bit_range_big(bits: u64, start_idx: u8, end_idx: u8) -> u64 {
    // remove the bits that are before the start idx by moving them to the left, this uses the bitfield
    // convention where the MSB has index 63
    let truncate_start = bits << (63 - start_idx);
    truncate_start >> (63 - (start_idx - end_idx))
}

pub fn opcode_to_inst(opcode: Opcode) -> Option<Box<dyn ExecutableInstruction>> {
    // bits based parsing
    for parsefn in PARSE_LIST {
        match parsefn(&opcode) {
            Ok(None) => continue,
            Ok(Some(inst)) => return Some(inst),
            Err(e) => {
                return None;
            }
        }
    }
    // legacy parsing (string-based)
    match LegacyInstruction::from_opcode(opcode) {
        Some(inst) => Some(Box::new(inst)), // must explicily coerce
        None => None,
    }
}

impl LegacyInstruction {
    pub fn from_opcode(opcode: Opcode) -> Option<Self> {
        let insn_string = opcode.to_string();
        let insn = Self::do_parse(
            &insn_string
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(" ")) ;

        if insn.is_none() {
            log::error!("failed to parse instruction as string: {insn_string}");
        }

        insn
    }
    fn do_parse(command: &str) -> Option<Self> {
        let command = command.trim();
        let mut data = split_insn(command);
        let mut condition: Option<String> = None;
        if let Some(split) = data.0.split_once('.') {
            data.0 = split.0;
            condition = Some(String::from(split.1));
        }
        let inst = parse_instruction(data.0, data.1)?;
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
}

/// Splits instruction into instruction and args
/// Returns tuple containing (instruction str, args str)
fn split_insn(command: &str) -> (&str, &str) {
    if let Some(split) = command.split_once(char::is_whitespace) {
        split
    } else {
        (command, "")
    }
}

fn parse_instruction(opcode: &str, args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    match opcode.to_lowercase().as_str() {
        "add" => xxx::add::parse(args),
        "adds" => xxx::adds::parse(args),
        "adrp" => xxx::adrp::parse(args),
        "and" => xxx::and::parse(args),
        "ands" => xxx::ands::parse(args),
        "asr" => xxx::asr::parse(args),
        "b" => xxx::b::parse(args),
        "bfxil" => xxx::bfxil::parse(args),
        "bic" => xxx::bic::parse(args),
        "bl" => xxx::bl::parse(args),
        "blr" => xxx::blr::parse(args),
        "br" => xxx::br::parse(args),
        "cbnz" => xxx::cbnz::parse(args),
        "cbz" => xxx::cbz::parse(args),
        "ccmp" => xxx::ccmp::parse(args),
        "cinc" => xxx::cinc::parse(args),
        "cmn" => xxx::cmn::parse(args),
        "cmp" => xxx::cmp::parse(args),
        "csel" => xxx::csel::parse(args),
        "cset" => xxx::cset::parse(args),
        "csinc" => xxx::csinc::parse(args),
        "csinv" => xxx::csinv::parse(args),
        "csneg" => xxx::csneg::parse(args),
        "eor" => xxx::eor::parse(args),
        "fcmp" => xxx::fcmp::parse(args),
        "fcvtzs" => xxx::fcvtzs::parse(args),
        "fdiv" => xxx::fdiv::parse(args),
        "fmov" => xxx::fmov::parse(args),
        "fmul" => xxx::fmul::parse(args),
        "ldp" => xxx::ldp::parse(args),
        "ldr" => xxx::ldr::parse(args),
        "ldrb" => xxx::ldrb::parse(args),
        "ldrh" => xxx::ldrh::parse(args),
        // "ldrsw" => xxx::ldrsw::parse(args),
        // "ldur" => xxx::ldur::parse(args),
        // "ldurb" => xxx::ldurb::parse(args),
        // "madd" => xxx::madd::parse(args),
        "mov" => xxx::mov::parse(args),
        "movk" => xxx::movk::parse(args),
        // "msub" => xxx::msub::parse(args),
        // "mul" => xxx::mul::parse(args),
        // "mvn" => xxx::mvn::parse(args),
        // "neg" => xxx::neg::parse(args),
        // "orn" => xxx::orn::parse(args),
        // "orr" => xxx::orr::parse(args),
        // "sub" => xxx::sub::parse(args),
        // "subs" => xxx::subs::parse(args),
        // "tst" => xxx::tst::parse(args),
        // "lsl" => xxx::lsl::parse(args),
        // "lsr" => xxx::lsr::parse(args),
        // "sbfiz" => xxx::sbfiz::parse(args),
        // "scvtf" => xxx::scvtf::parse(args),
        // "smaddl" => xxx::smaddl::parse(args),
        // "smull" => xxx::smull::parse(args),
        // "ret" => xxx::ret::parse(args),
        // "tbnz" => xxx::tbnz::parse(args),
        // "tbz" => xxx::tbz::parse(args),
        // "stp" => xxx::stp::parse(args),
        // "str" => xxx::str::parse(args),
        // "strb" => xxx::strb::parse(args),
        // "strh" => xxx::strh::parse(args),
        // "stur" => xxx::stur::parse(args),
        // "sturb" => xxx::sturb::parse(args),
        // "sturh" => xxx::sturh::parse(args),
        // "sxtw" => xxx::sxtw::parse(args),
        // "ubfm" => xxx::ubfm::parse(args),
        _ => {
            log::error!("unknown opcode when parsing instruction as string: {opcode}");
            None
        }
    }
}

pub fn split_args(args: &str, n: usize) -> Vec<String> {
    let split_args: std::str::SplitN<'_, &str> = args.splitn(n, ", ");
    split_args.map(|s| s.to_string()).collect()
}

   pub  fn parse_auxiliary(fourth_arg: Option<&String>) -> Option<Option<AuxiliaryOperation>> {
        if let Some(arg) = fourth_arg {
            let split_arg: Vec<String> = arg
                .split(char::is_whitespace)
                .map(|s| s.to_string())
                .collect();
            let shift_val = if split_arg.len() > 1 {
                match get_imm_val(&split_arg[1]) {
                    Some(val) => val,
                    None => {
                        log::error!("Failed to parse shift value from argument: {}", split_arg[1]);
                        return None
                    }
                }
            } else {
                0
            };
            Some(Some(AuxiliaryOperation {
                operation: split_arg.first().unwrap().to_string(),
                shift_val,
            }))
        } else {
            Some(None)
        }
    }
pub     fn get_imm_val(imm: &String) -> Option<i64> {
        let no_hash = imm.strip_prefix('#').unwrap_or(imm);
        let mult = if no_hash.starts_with('-') { -1 } else { 1 };
        let no_dash = no_hash.replace(['-', '!', ' ', '\t'], "");

        if let Some(hex) = no_dash.strip_prefix("0x") {
            match u64::from_str_radix(hex, 16) {
                Ok(val) => Some((val as i64) * mult),
                Err(_) => {
                    log::error!("get_imm_val failed to parse hex value: {hex}");
                    None
                }
            }
        } else {
            match no_dash.parse::<i64>() {
                Ok(val) => Some(val * mult),
                Err(_) => {
                    log::error!("get_imm_val failed to parse value (decimal): {no_dash}");
                    None
                }
            }
        }
    }
    pub fn get_label_val(imm: &str) -> Option<u64> {
        let stripped = imm.strip_prefix("0x").unwrap_or(imm);
        match u64::from_str_radix(stripped, 16) {
            Ok(val) => Some(val),
            Err(_) => {
                log::error!("get_label_val failed to parse label value: {stripped}");
                None
            }
        }
    }
    pub fn is_imm(str: &str) -> bool {
        str.starts_with('#') || str.starts_with("0x")
    }
    pub fn convert_to_f64(input: &str) -> Option<f64> {
        let trimmed_input = input.trim_start_matches('#');
        match trimmed_input.parse::<f64>() {
            Ok(val) => Some(val),
            Err(_) => {
                log::error!("convert_to_f64 failed to parse value: {trimmed_input}");
                None
            }
        }
    }
    pub fn ends_with_exclam(arg: &str) -> bool {
        arg.ends_with('!')
    }
    pub fn split_bracket_args(bracketed: &str) -> Vec<String> {
        let binding = bracketed.replace(['[', ']'], "");
        let split_string = binding.splitn(3, ", ");
        split_string.map(|s| s.to_string()).collect()
    }

#[derive(Clone)]
pub struct AuxiliaryOperation {
    pub operation: String,
    pub shift_val: i64,
}

pub enum AuxParseResult {
    Error,
    None,
    Some(AuxiliaryOperation),
}

pub trait ExecutableInstruction: ExecutableInstructionClone + Send + Sync + std::panic::UnwindSafe + 'static{
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error>;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bit_range() {
        let temp: u32 = 0x80000000;
        assert_eq!(get_bit_range(temp, 31, 31), 1)
    }
    #[test]
    fn test_bit_range_alt() {
        let temp: u32 = 0xf0000000;
        assert_eq!(get_bit_range(temp, 31, 30), 3)
    }
}
