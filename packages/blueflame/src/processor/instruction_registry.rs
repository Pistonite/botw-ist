use anyhow::Result;
use disarm64::decoder::{self};
use std::str::FromStr;
use thiserror::Error;

use crate::Core;

use crate::processor::Error;

use super::RegIndex;

#[derive(Clone)]
pub struct AuxiliaryOperation {
    pub operation: String,
    pub shift_val: i64,
}

#[derive(Clone, Copy, Debug)]
pub enum RegisterType {
    XReg(RegIndex),
    WReg(RegIndex),
    BReg(RegIndex),
    HReg(RegIndex),
    SReg(RegIndex),
    DReg(RegIndex),
    QReg(RegIndex),
    XZR,
    WZR,
    SP,
    LR,
}

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
            _ => Err(anyhow::anyhow!("Unknown register type: {}", reg)),
        }
    }
}

#[derive(Debug, Error)]
pub enum InstructionParseError {
    #[error("Invalid input: {0}")]
    ParseError(String),
}

pub enum InstructionType {
    Return,
    Branch,
}

pub trait ExecutableInstruction: ExecutableInstructionClone {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error>;
    fn instruction_type(&self) -> Option<InstructionType> {
        None
    }
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

#[derive(Clone)]
pub struct LegacyInstruction {
    pub inst: Box<dyn ExecutableInstruction>,
    pub condition: Option<String>,
}

#[derive(Clone)]
pub struct AddInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub rm: RegisterType,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl Clone for Box<dyn ExecutableInstruction> {
    fn clone(&self) -> Box<dyn ExecutableInstruction> {
        self.clone_box()
    }
}

impl ExecutableInstruction for AddInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.add(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AddImmInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub imm_val: i64,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AddImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.add_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AddsInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub rm: RegisterType,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AddsInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.adds(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AddsImmInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub imm_val: i64,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AddsImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.adds_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AndInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AndInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.and(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AndImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AndImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.and_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AndsInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AndsInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ands(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AndsImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AndsImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ands_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct EorInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for EorInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.eor(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct EorImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for EorImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.eor_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct MaddInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    xa: RegisterType,
}

impl ExecutableInstruction for MaddInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.madd(self.rd, self.rn, self.rm, self.xa)
    }
}

#[derive(Clone)]
pub struct MovInstruction {
    rd: RegisterType,
    rn: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mov(self.rd, self.rn, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct MovImmInstruction {
    rd: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mov_imm(self.rd, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct MovkInstruction {
    rd: RegisterType,
    imm_val: u16,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovkInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.movk(self.rd, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct MsubInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    xa: RegisterType,
}

impl ExecutableInstruction for MsubInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.msub(self.rd, self.rn, self.rm, self.xa)
    }
}

#[derive(Clone)]
pub struct MulInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MulInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mul(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct MulImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MulImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mul_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct MvnInstruction {
    rd: RegisterType,
    rn: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MvnInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mvn(self.rd, self.rn, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct NegInstruction {
    rd: RegisterType,
    rn: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for NegInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.neg(self.rd, self.rn, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct OrnInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for OrnInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.orn(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct OrnImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for OrnImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.orn_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct OrrInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for OrrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.orr(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct OrrImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for OrrImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.orr_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SubInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sub(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SubImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sub_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SubsInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubsInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.subs(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SubsImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubsImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.subs_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct TstInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for TstInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.tst(self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct TstImmInstruction {
    rn: RegisterType,
    imm_val: i64,
}

impl ExecutableInstruction for TstImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.tst_imm(self.rn, self.imm_val)
    }
}

#[derive(Clone)]
pub struct AsrInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AsrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.asr(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AsrImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AsrImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.asr_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct BfxilInstruction {
    rd: RegisterType,
    rn: RegisterType,
    lsb: i64,
    width: i64,
}

impl ExecutableInstruction for BfxilInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.bfxil(self.rd, self.rn, self.lsb, self.width)
    }
}

#[derive(Clone)]
pub struct LslInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LslInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.lsl(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LslImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LslImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.lsl_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LsrInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LsrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.lsr(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LsrImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LsrImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.lsr_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SbfizInstruction {
    rd: RegisterType,
    rn: RegisterType,
    lsb: i64,
    width: i64,
}

impl ExecutableInstruction for SbfizInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sbfiz(self.rd, self.rn, self.lsb, self.width)
    }
}

#[derive(Clone)]
pub struct CcmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
    nzcv_val: u8,
    cond: String,
}

impl ExecutableInstruction for CcmpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ccmp(self.rn, self.rm, self.nzcv_val, &self.cond)
    }
}

#[derive(Clone)]
pub struct CcmpImmInstruction {
    rn: RegisterType,
    imm_val: u8,
    nzcv_val: u8,
    cond: String,
}

impl ExecutableInstruction for CcmpImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ccmp_imm(self.rn, self.imm_val, self.nzcv_val, &self.cond)
    }
}

#[derive(Clone)]
pub struct CincInstruction {
    rd: RegisterType,
    rn: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CincInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cinc(self.rd, self.rn, &self.cond)
    }
}

#[derive(Clone)]
pub struct CmnInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for CmnInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cmn(self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct CmnImmInstruction {
    rn: RegisterType,
    imm_val: u8,
}

impl ExecutableInstruction for CmnImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cmn_imm(self.rn, self.imm_val)
    }
}

#[derive(Clone)]
pub struct CmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for CmpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cmp(self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct CmpImmInstruction {
    rn: RegisterType,
    imm_val: u8,
}

impl ExecutableInstruction for CmpImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cmp_imm(self.rn, self.imm_val)
    }
}

#[derive(Clone)]
pub struct CselInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CselInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.csel(self.rd, self.rn, self.rm, &self.cond)
    }
}

#[derive(Clone)]
pub struct CsetInstruction {
    rd: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CsetInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cset(self.rd, &self.cond)
    }
}

#[derive(Clone)]
pub struct CsincInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CsincInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.csinc(self.rd, self.rn, self.rm, &self.cond)
    }
}

#[derive(Clone)]
pub struct CsinvInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CsinvInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.csinv(self.rd, self.rn, self.rm, &self.cond)
    }
}

#[derive(Clone)]
pub struct CsnegInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CsnegInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.csneg(self.rd, self.rn, self.rm, &self.cond)
    }
}

#[derive(Clone)]
pub struct FcmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for FcmpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fcmp(self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct FcmpZeroInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for FcmpZeroInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fcmp_zero(self.rn)
    }
}

#[derive(Clone)]
pub struct FcvtzsInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for FcvtzsInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fcvtzs(self.rd, self.rn)
    }
}

#[derive(Clone)]
pub struct FdivInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for FdivInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fdiv(self.rd, self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct FmovInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for FmovInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fmov(self.rd, self.rn)
    }
}

#[derive(Clone)]
pub struct FmovImmInstruction {
    rd: RegisterType,
    float_val: f64,
}

impl ExecutableInstruction for FmovImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fmov_imm(self.rd, self.float_val)
    }
}

#[derive(Clone)]
pub struct FmulInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for FmulInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fmul(self.rd, self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct ScvtfInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for ScvtfInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.scvtf(self.rd, self.rn)
    }
}

#[derive(Clone)]
pub struct SmaddlInstruction {
    rd: RegisterType,
    wn: RegisterType,
    wm: RegisterType,
    ra: RegisterType,
}

impl ExecutableInstruction for SmaddlInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.smaddl(self.rd, self.wn, self.wm, self.ra)
    }
}

#[derive(Clone)]
pub struct SmullInstruction {
    rd: RegisterType,
    wn: RegisterType,
    wm: RegisterType,
}

impl ExecutableInstruction for SmullInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.smull(self.rd, self.wn, self.wm)
    }
}

#[derive(Clone)]
pub struct BInstruction {
    label_offset: u64,
}

impl ExecutableInstruction for BInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.b(self.label_offset)
    }
}

#[derive(Clone)]
pub struct BlInstruction {
    label_offset: u64,
}

impl ExecutableInstruction for BlInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.bl(self.label_offset)
    }

    fn instruction_type(&self) -> Option<InstructionType> {
        Some(InstructionType::Branch)
    }
}

#[derive(Clone)]
pub struct BicInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for BicInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.bic(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct BicImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for BicImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.bic_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct BlrInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for BlrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.blr(self.rn)
    }

    fn instruction_type(&self) -> Option<InstructionType> {
        Some(InstructionType::Branch)
    }
}

#[derive(Clone)]
pub struct BrInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for BrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.br(self.rn)
    }
}

#[derive(Clone)]
pub struct CbnzInstruction {
    rn: RegisterType,
    label_offset: u64,
}

impl ExecutableInstruction for CbnzInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cbnz(self.rn, self.label_offset)
    }
}

#[derive(Clone)]
pub struct CbzInstruction {
    rn: RegisterType,
    label_offset: u64,
}

impl ExecutableInstruction for CbzInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cbz(self.rn, self.label_offset)
    }
}

#[derive(Clone)]
pub struct RetInstruction;

impl ExecutableInstruction for RetInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ret();
        Ok(())
    }

    fn instruction_type(&self) -> Option<InstructionType> {
        Some(InstructionType::Return)
    }
}

#[derive(Clone)]
pub struct RetArgsInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for RetArgsInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ret_with_arg(self.rn)
    }

    fn instruction_type(&self) -> Option<InstructionType> {
        Some(InstructionType::Return)
    }
}

#[derive(Clone)]
pub struct TbnzInstruction {
    rn: RegisterType,
    imm_val: u64,
    label_offset: u64,
}

impl ExecutableInstruction for TbnzInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.tbnz(self.rn, self.imm_val, self.label_offset)
    }
}

#[derive(Clone)]
pub struct TbzInstruction {
    rn: RegisterType,
    imm_val: u64,
    label_offset: u64,
}

impl ExecutableInstruction for TbzInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.tbz(self.rn, self.imm_val, self.label_offset)
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.rm,
            self.extra_op.clone(),
        )
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp_pre_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
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

impl ExecutableInstruction for LdpPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp_post_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp_imm(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}

#[derive(Clone)]
pub struct LdrInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldr(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldr_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldr_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldr_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrbInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrbInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrb(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrbPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrbPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrb_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrbPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrbPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrb_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrbImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrbImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrb_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrswInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrswInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrsw(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrswPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrswPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrsw_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrswPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrswPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrsw_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrswImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrswImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrsw_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdurInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldur(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldur_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldur_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldur_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdurbInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurbInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldurb(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdurbPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurbPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldurb_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdurbPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurbPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldurb_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdurbImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdurbImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldurb_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrhInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrhInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrh(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrhPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrhPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrh_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrhPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrhPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrh_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct LdrhImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdrhImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldrh_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.rm,
            self.extra_op.clone(),
        )
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp_pre_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp_post_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp_imm(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}

#[derive(Clone)]
pub struct StrInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrbInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strb(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrbPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strb_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrbPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strb_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrbImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrbImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strb_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrhInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strh(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrhPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strh_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrhPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strh_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrhImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrhImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.strh_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stur(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stur_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stur_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
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
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stur_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturbInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturbInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturb(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturbPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturbPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturb_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturbPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturbPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturb_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturbImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturbImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturb_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturhInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturhInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturh(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturhPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturhPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturh_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturhPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturhPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturh_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SturhImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SturhImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sturh_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SxtwInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for SxtwInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sxtw(self.rd, self.rn)
    }
}

#[derive(Clone)]
pub struct AdrpInstruction {
    rd: RegisterType,
    label_offsetess: u64,
}

impl ExecutableInstruction for AdrpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.adrp(self.rd, self.label_offsetess)
    }
}

#[derive(Clone)]
pub struct UbfmInstruction {
    rn: RegisterType,
    rd: RegisterType,
    immr: i64,
    imms: i64,
}

impl ExecutableInstruction for UbfmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ubfm(self.rd, self.rn, self.immr, self.imms)
    }
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

    fn parse_instruction(opcode: &str, args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        match opcode.to_lowercase().as_str() {
            "add" => Self::parse_add(args),
            "adds" => Self::parse_adds(args),
            "and" => Self::parse_and(args),
            "ands" => Self::parse_ands(args),
            "eor" => Self::parse_eor(args),
            "madd" => Self::parse_madd(args),
            "mov" => Self::parse_mov(args),
            "movk" => Self::parse_movk(args),
            "msub" => Self::parse_msub(args),
            "mul" => Self::parse_mul(args),
            "mvn" => Self::parse_mvn(args),
            "neg" => Self::parse_neg(args),
            "orn" => Self::parse_orn(args),
            "orr" => Self::parse_orr(args),
            "sub" => Self::parse_sub(args),
            "subs" => Self::parse_subs(args),
            "tst" => Self::parse_tst(args),
            "asr" => Self::parse_asr(args),
            "bfxil" => Self::parse_bfxil(args),
            "lsl" => Self::parse_lsl(args),
            "lsr" => Self::parse_lsr(args),
            "sbfiz" => Self::parse_sbfiz(args),
            "ccmp" => Self::parse_ccmp(args),
            "cinc" => Self::parse_cinc(args),
            "cmn" => Self::parse_cmn(args),
            "cmp" => Self::parse_cmp(args),
            "csel" => Self::parse_csel(args),
            "cset" => Self::parse_cset(args),
            "csinc" => Self::parse_csinc(args),
            "csinv" => Self::parse_csinv(args),
            "csneg" => Self::parse_csneg(args),
            "fcmp" => Self::parse_fcmp(args),
            "fcvtzs" => Self::parse_fcvtzs(args),
            "fdiv" => Self::parse_fdiv(args),
            "fmov" => Self::parse_fmov(args),
            "fmul" => Self::parse_fmul(args),
            "scvtf" => Self::parse_scvtf(args),
            "smaddl" => Self::parse_smaddl(args),
            "smull" => Self::parse_smull(args),
            "b" => Self::parse_b(args),
            "bic" => Self::parse_bic(args),
            "bl" => Self::parse_bl(args),
            "blr" => Self::parse_blr(args),
            "br" => Self::parse_br(args),
            "cbnz" => Self::parse_cbnz(args),
            "cbz" => Self::parse_cbz(args),
            "ret" => Self::parse_ret(args),
            "tbnz" => Self::parse_tbnz(args),
            "tbz" => Self::parse_tbz(args),
            "ldp" => Self::parse_ldp(args),
            "ldr" => Self::parse_ldr(args),
            "ldrb" => Self::parse_ldrb(args),
            "ldrsw" => Self::parse_ldrsw(args),
            "ldur" => Self::parse_ldur(args),
            "ldurb" => Self::parse_ldurb(args),
            "ldrh" => Self::parse_ldrh(args),
            "stp" => Self::parse_stp(args),
            "str" => Self::parse_str(args),
            "strb" => Self::parse_strb(args),
            "strh" => Self::parse_strh(args),
            "stur" => Self::parse_stur(args),
            "sturb" => Self::parse_sturb(args),
            "sturh" => Self::parse_sturh(args),
            "sxtw" => Self::parse_sxtw(args),
            "adrp" => Self::parse_adrp(args),
            "ubfm" => Self::parse_ubfm(args),
            _ => Err(
                InstructionParseError::ParseError(format!("Invalid opcode '{}'", opcode)).into(),
            ),
        }
    }

    fn parse_add(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        if collected_args[2].starts_with('#') {
            //Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(AddImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(AddInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_adds(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        if collected_args[2].starts_with('#') {
            //Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(AddsImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(AddsInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_and(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        if collected_args[2].starts_with('#') {
            //Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(AndImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(AndInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_ands(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        if collected_args[2].starts_with('#') {
            //Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(AndsImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(AndsInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_eor(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        if collected_args[2].starts_with('#') {
            //Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(EorImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(EorInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_madd(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let xa = RegisterType::from_str(&collected_args[3])?;
        Ok(Box::new(MaddInstruction { rd, rn, rm, xa }))
    }

    fn parse_mov(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(2))?;
        if collected_args[1].starts_with('#') {
            let imm_val = Self::get_imm_val(&collected_args[1])?;
            Ok(Box::new(MovImmInstruction {
                rd,
                imm_val,
                extra_op,
            }))
        } else {
            let rn = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(MovInstruction { rd, rn, extra_op }))
        }
    }

    fn parse_movk(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let imm_val = Self::get_imm_val(&collected_args[1])? as u16;
        let extra_op = Self::parse_auxiliary(collected_args.get(2))?;
        Ok(Box::new(MovkInstruction {
            rd,
            imm_val,
            extra_op,
        }))
    }

    fn parse_msub(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let xa = RegisterType::from_str(&collected_args[3])?;
        Ok(Box::new(MsubInstruction { rd, rn, rm, xa }))
    }

    fn parse_mul(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(MulImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(MulInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_mvn(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        Ok(Box::new(MvnInstruction { rd, rn, extra_op }))
    }

    fn parse_neg(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        Ok(Box::new(NegInstruction { rd, rn, extra_op }))
    }

    fn parse_orn(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(OrnImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(OrnInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_orr(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(OrrImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(OrrInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_sub(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(SubImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(SubInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_subs(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(SubsImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(SubsInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_tst(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&collected_args[0])?;

        if Self::is_imm(&collected_args[1]) {
            let imm_val = Self::get_imm_val(&collected_args[1])?;
            Ok(Box::new(TstImmInstruction { rn, imm_val }))
        } else {
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(TstInstruction { rn, rm }))
        }
    }

    fn parse_asr(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(AsrImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(AsrInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_bfxil(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let lsb = Self::get_imm_val(&collected_args[2])?;
        let width = Self::get_imm_val(&collected_args[3])?;

        Ok(Box::new(BfxilInstruction { rd, rn, lsb, width }))
    }

    fn parse_lsl(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(LslImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(LslInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_lsr(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(LsrImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(LsrInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_bic(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        if collected_args[2].starts_with('#') {
            //Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(BicImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(BicInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }

    fn parse_sbfiz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let lsb = Self::get_imm_val(&collected_args[2])?;
        let width = Self::get_imm_val(&collected_args[3])?;

        Ok(Box::new(SbfizInstruction { rd, rn, lsb, width }))
    }

    fn parse_ccmp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rn = RegisterType::from_str(&collected_args[0])?;
        let nzcv_val = Self::get_imm_val(&collected_args[2])? as u8;
        let cond = collected_args[3].clone();

        if collected_args[1].starts_with('#') {
            // Immediate value case
            let imm_val = Self::get_imm_val(&collected_args[1])? as u8;
            Ok(Box::new(CcmpImmInstruction {
                rn,
                imm_val,
                nzcv_val,
                cond,
            }))
        } else {
            // Register value case
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(CcmpInstruction {
                rn,
                rm,
                nzcv_val,
                cond,
            }))
        }
    }

    fn parse_cinc(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let cond = collected_args[2].clone();

        Ok(Box::new(CincInstruction { rd, rn, cond }))
    }

    fn parse_cmn(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&collected_args[0])?;

        if Self::is_imm(&collected_args[1]) {
            let imm_val = Self::get_imm_val(&collected_args[1])? as u8;
            Ok(Box::new(CmnImmInstruction { rn, imm_val }))
        } else {
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(CmnInstruction { rn, rm }))
        }
    }

    fn parse_cmp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&collected_args[0])?;

        if Self::is_imm(&collected_args[1]) {
            let imm_val = Self::get_imm_val(&collected_args[1])? as u8;
            Ok(Box::new(CmpImmInstruction { rn, imm_val }))
        } else {
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(CmpInstruction { rn, rm }))
        }
    }

    fn parse_csel(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let cond = collected_args[3].clone();

        Ok(Box::new(CselInstruction { rd, rn, rm, cond }))
    }

    fn parse_cset(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let cond = collected_args[1].clone();

        Ok(Box::new(CsetInstruction { rd, cond }))
    }

    fn parse_csinc(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let cond = collected_args[3].clone();

        Ok(Box::new(CsincInstruction { rd, rn, rm, cond }))
    }

    fn parse_csinv(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let cond = collected_args[3].clone();

        Ok(Box::new(CsinvInstruction { rd, rn, rm, cond }))
    }

    fn parse_csneg(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let cond = collected_args[3].clone();

        Ok(Box::new(CsnegInstruction { rd, rn, rm, cond }))
    }

    fn parse_fcmp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&collected_args[0])?;
        if collected_args[1].starts_with("#0.0") {
            // Variant where you don't compare register with anything
            Ok(Box::new(FcmpZeroInstruction { rn }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(FcmpInstruction { rn, rm }))
        }
    }

    fn parse_fcvtzs(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;

        Ok(Box::new(FcvtzsInstruction { rd, rn }))
    }

    fn parse_fdiv(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;

        Ok(Box::new(FdivInstruction { rd, rn, rm }))
    }

    fn parse_fmov(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rd = RegisterType::from_str(&collected_args[0])?;
        if collected_args[1].starts_with("#") {
            let float_val = Self::convert_to_f64(&collected_args[1])?;
            Ok(Box::new(FmovImmInstruction { rd, float_val }))
        } else {
            let rn = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(FmovInstruction { rd, rn }))
        }
    }

    fn parse_fmul(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;

        Ok(Box::new(FmulInstruction { rd, rn, rm }))
    }

    fn parse_scvtf(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        Ok(Box::new(ScvtfInstruction { rd, rn }))
    }

    fn parse_smaddl(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let wn = RegisterType::from_str(&collected_args[1])?;
        let wm = RegisterType::from_str(&collected_args[2])?;
        let ra = RegisterType::from_str(&collected_args[3])?;
        Ok(Box::new(SmaddlInstruction { rd, wn, wm, ra }))
    }

    fn parse_smull(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let wn = RegisterType::from_str(&collected_args[1])?;
        let wm = RegisterType::from_str(&collected_args[2])?;
        Ok(Box::new(SmullInstruction { rd, wn, wm }))
    }

    fn parse_b(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let label_offset = Self::get_label_val(args)?;
        Ok(Box::new(BInstruction { label_offset }))
    }

    fn parse_bl(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let label_offset = Self::get_label_val(args)?;
        Ok(Box::new(BlInstruction { label_offset }))
    }

    fn parse_blr(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let rn = RegisterType::from_str(args)?;
        Ok(Box::new(BlrInstruction { rn }))
    }

    fn parse_br(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let rn = RegisterType::from_str(args)?;
        Ok(Box::new(BrInstruction { rn }))
    }

    fn parse_cbnz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let split = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&split[0])?;
        let label_offset = Self::get_label_val(&split[1])?;
        Ok(Box::new(CbnzInstruction { rn, label_offset }))
    }

    fn parse_cbz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let split = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&split[0])?;
        let label_offset = Self::get_label_val(&split[1])?;
        Ok(Box::new(CbzInstruction { rn, label_offset }))
    }

    fn parse_ret(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        if args.is_empty() {
            Ok(Box::new(RetInstruction))
        } else {
            let rn = RegisterType::from_str(args)?;
            Ok(Box::new(RetArgsInstruction { rn }))
        }
    }

    fn parse_tbnz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let split = Self::split_args(args, 3);
        let rn = RegisterType::from_str(&split[0])?;
        let imm_val = Self::get_imm_val(&split[1])? as u64;
        let label_offset = Self::get_label_val(&split[2])?;
        Ok(Box::new(TbnzInstruction {
            rn,
            imm_val,
            label_offset,
        }))
    }

    fn parse_tbz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let split = Self::split_args(args, 3);
        let rn = RegisterType::from_str(&split[0])?;
        let imm_val = Self::get_imm_val(&split[1])? as u64;
        let label_offset = Self::get_label_val(&split[2])?;
        Ok(Box::new(TbzInstruction {
            rn,
            imm_val,
            label_offset,
        }))
    }

    fn parse_ldp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 3);
        let split_third: Vec<String> = Self::split_bracket_args(&collected_args[2]);
        let rt1 = RegisterType::from_str(&collected_args[0])?;
        let rt2 = RegisterType::from_str(&collected_args[1])?;
        let rn_sp = RegisterType::from_str(&split_third[0])?;
        let extra_op = Self::parse_auxiliary(split_third.get(2))?;
        let imm_val = if let Some(val) = split_third.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(LdpInstruction {
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
        if Self::ends_with_exclam(&collected_args[2]) {
            Ok(Box::new(LdpPreInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[2].contains("], ") {
            Ok(Box::new(LdpPostInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdpImmInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_ldr(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(LdrInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(LdrPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(LdrPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdrImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_ldrb(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(LdrbInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(LdrbPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(LdrbPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdrbImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_ldrsw(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(LdrswInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(LdrswPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(LdrswPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdrswImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_ldur(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(LdurInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(LdurPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(LdurPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdurImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_ldurb(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(LdurbInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(LdurbPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(LdurbPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdurbImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_ldrh(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(LdrhInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(LdrhPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(LdrhPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdrhImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_stp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 3);
        let split_third: Vec<String> = Self::split_bracket_args(&collected_args[2]);
        let rt1 = RegisterType::from_str(&collected_args[0])?;
        let rt2 = RegisterType::from_str(&collected_args[1])?;
        let rn_sp = RegisterType::from_str(&split_third[0])?;
        let extra_op = Self::parse_auxiliary(split_third.get(2))?;
        let imm_val = if let Some(val) = split_third.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(StpInstruction {
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
        if Self::ends_with_exclam(&collected_args[2]) {
            Ok(Box::new(StpPreInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[2].contains("], ") {
            Ok(Box::new(StpPostInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(StpImmInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_str(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(StrInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(StrPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(StrPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(StrImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_strb(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(StrbInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(StrbPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(StrbPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(StrbImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_strh(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(StrhInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(StrhPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(StrhPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(StrhImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_stur(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(SturInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(SturPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(SturPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(SturImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_sturb(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(SturbInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(SturbPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(SturbPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(SturbImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_sturh(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(SturhInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(SturhPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(SturhPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(SturhImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

    fn parse_sxtw(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        Ok(Box::new(SxtwInstruction { rd, rn }))
    }

    fn parse_adrp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let label_offsetess = Self::get_label_val(&collected_args[1])?;
        Ok(Box::new(AdrpInstruction {
            rd,
            label_offsetess,
        }))
    }

    fn parse_ubfm(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let immr = Self::get_imm_val(&collected_args[2])?;
        let imms = Self::get_imm_val(&collected_args[3])?;
        Ok(Box::new(UbfmInstruction { rn, rd, immr, imms }))
    }
}

impl LegacyInstruction {
    pub fn from_u32(instruction: u32) -> Result<Self, anyhow::Error> {
        let insn_option = decoder::decode(instruction);
        if let Some(insn) = insn_option {
            let insn_string = insn.to_string();
            // println!("INST: {}", insn_string);
            // println!("INSTOP: {:?}", insn);
            Self::from_str(
                &insn_string
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
            )
        } else {
            Err(InstructionParseError::ParseError(String::from("Byte parsing failed")).into())
        }
    }
}

impl FromStr for LegacyInstruction {
    type Err = anyhow::Error;
    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let command = command.trim();
        let mut data = Self::split_insn(command);
        let mut condition: Option<String> = None;
        if let Some(split) = data.0.split_once('.') {
            data.0 = split.0;
            condition = Some(String::from(split.1));
        }
        let inst = Self::parse_instruction(data.0, data.1)?;
        Ok(LegacyInstruction { inst, condition })
    }
}

impl ExecutableInstruction for LegacyInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        let mut should_run = true;
        if let Some(cond) = &self.condition {
            if !proc.cpu.flags.does_condition_succeed(cond)? {
                should_run = false;
            }
        }
        if should_run {
            return self.inst.exec_on(proc);
        }
        Ok(())
    }
}

impl Core<'_, '_, '_> {
    pub fn split_insn(command: &str) -> (&str, &str) {
        if let Some(split) = command.split_once(char::is_whitespace) {
            split
        } else {
            (command, "")
        }
    }

    pub fn get_inst_type(inst: &str) -> &str {
        let data = Self::split_insn(inst);
        if let Some(split) = data.0.split_once('.') {
            split.0
        } else {
            data.0
        }
    }

    pub fn handle_string_command_no_inc(&mut self, command: &str) -> Result<(), anyhow::Error> {
        let inst = LegacyInstruction::from_str(command)?;
        inst.exec_on(self)?;
        Ok(())
    }

    pub fn handle_string_command(&mut self, command: &str) -> Result<(), anyhow::Error> {
        let inst = LegacyInstruction::from_str(command)?;
        inst.exec_on(self)?;
        self.cpu.pc += 4;
        Ok(())
    }
}
