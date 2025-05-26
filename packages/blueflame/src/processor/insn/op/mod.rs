use disarm64::decoder::{Opcode, Operation};

#[layered_crate::import]
use processor::{Process, Cpu0, Error};

/// Check if the instruction *could* branch to another place (instead of
/// the next instruction)
pub fn is_branch(opcode: Opcode) -> bool {
    matches!(opcode.operation, 
        Operation::BRANCH_IMM(_) | 
        Operation::BRANCH_REG(_) | 
        Operation::COMPBRANCH(_) | 
        Operation::TESTBRANCH(_))
}

pub enum ExecResult {
    Handled,
    NotImplemented,
    Error(Error),
}

pub fn execute_opcode(cpu: &mut Cpu0, proc: &mut Process, opcode: Opcode) 
-> ExecResult {
    match opcode.operation {
        Operation::AARCH64_MISC(aarch64_misc) => {
            log::trace!("unimplemented AARCH64_MISC");
            return ExecResult::NotImplemented;
        },
        Operation::ADDSUB_CARRY(addsub_carry) => {
            log::trace!("unimplemented ADDSUB_CARRY");
            return ExecResult::NotImplemented;
        },
        Operation::ADDSUB_EXT(addsub_ext) => {
            log::trace!("unimplemented ADDSUB_EXT");
            return ExecResult::NotImplemented;
        },
        Operation::ADDSUB_IMM(addsub_imm) => {
            log::trace!("unimplemented ADDSUB_IMM");
            return ExecResult::NotImplemented;
        },
        Operation::ADDSUB_SHIFT(addsub_shift) => {
            log::trace!("unimplemented ADDSUB_SHIFT");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDALL(asimdall) => {
            log::trace!("unimplemented ASIMDALL");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDDIFF(asimddiff) => {
            log::trace!("unimplemented ASIMDDIFF");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDELEM(asimdelem) => {
            log::trace!("unimplemented ASIMDELEM");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDIMM(asimdimm) => {
            log::trace!("unimplemented ASIMDIMM");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDMISC(asimdmisc) => {
            log::trace!("unimplemented ASIMDMISC");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDPERM(asimdperm) => {
            log::trace!("unimplemented ASIMDPERM");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDSAME(asimdsame) => {
            log::trace!("unimplemented ASIMDSAME");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDSHF(asimdshf) => {
            log::trace!("unimplemented ASIMDSHF");
            return ExecResult::NotImplemented;
        },
        Operation::ASIMDTBL(asimdtbl) => {
            log::trace!("unimplemented ASIMDTBL");
            return ExecResult::NotImplemented;
        },

        Operation::ASISDELEM(asisdelem) => {
            log::trace!("unimplemented ASISDELEM");
            return ExecResult::NotImplemented;
        },
        Operation::ASISDMISC(asisdmisc) => {
            log::trace!("unimplemented ASISDMISC");
            return ExecResult::NotImplemented;
        },
        Operation::ASISDPAIR(asisdpair) => {
            log::trace!("unimplemented ASISDPAIR");
            return ExecResult::NotImplemented;
        },
        Operation::ASISDSAME(asisdsame) => {
            log::trace!("unimplemented ASISDSAME");
            return ExecResult::NotImplemented;
        },
        Operation::ASISDSHF(asisdshf) => {
            log::trace!("unimplemented ASISDSHF");
            return ExecResult::NotImplemented;
        },

        Operation::BFLOAT16(bfloat16) => {
            log::trace!("unimplemented BFLOAT16");
            return ExecResult::NotImplemented;
        },

        Operation::BITFIELD(bitfield) => {
            log::trace!("unimplemented BITFIELD");
            return ExecResult::NotImplemented;
        },

        Operation::BRANCH_IMM(branch_imm) => {
            log::trace!("unimplemented BRANCH_IMM");
            return ExecResult::NotImplemented;
        },
        Operation::BRANCH_REG(branch_reg) => {
            log::trace!("unimplemented BRANCH_REG");
            return ExecResult::NotImplemented;
        },

        Operation::COMPBRANCH(compbranch) => {
            log::trace!("unimplemented COMPBRANCH");
            return ExecResult::NotImplemented;
        },
        Operation::CONDCMP_IMM(condcmp_imm) => {
            log::trace!("unimplemented CONDCMP_IMM");
            return ExecResult::NotImplemented;
        },
        Operation::CONDCMP_REG(condcmp_reg) => {
            log::trace!("unimplemented CONDCMP_REG");
            return ExecResult::NotImplemented;
        },
        Operation::CONDSEL(condsel) => {
            log::trace!("unimplemented CONDSEL");
            return ExecResult::NotImplemented;
        },

        Operation::CRYPTOSHA3(cryptosha3) => {
            log::trace!("unimplemented CRYPTOSHA3");
            return ExecResult::NotImplemented;
        },

        Operation::CSSC(cssc) => {
            log::trace!("unimplemented CSSC");
            return ExecResult::NotImplemented;
        },

        Operation::DOTPRODUCT(dotproduct) => {
            log::trace!("unimplemented DOTPRODUCT");
            return ExecResult::NotImplemented;
        },

        Operation::DP_1SRC(dp_1_src) => {
            log::trace!("unimplemented DP_1SRC");
            return ExecResult::NotImplemented;
        },
        Operation::DP_2SRC(dp_2_src) => {
            log::trace!("unimplemented DP_2SRC");
            return ExecResult::NotImplemented;
        },
        Operation::DP_3SRC(dp_3_src) => {
            log::trace!("unimplemented DP_3SRC");
            return ExecResult::NotImplemented;
        },

        Operation::EXCEPTION(exception) => {
            log::trace!("unimplemented EXCEPTION");
            return ExecResult::NotImplemented;
        },

        Operation::FLOAT2FIX(float2_fix) => {
            log::trace!("unimplemented FLOAT2FIX");
            return ExecResult::NotImplemented;
        },
        Operation::FLOAT2INT(float2_int) => {
            log::trace!("unimplemented FLOAT2INT");
            return ExecResult::NotImplemented;
        },
        Operation::FLOATCCMP(floatccmp) => {
            log::trace!("unimplemented FLOATCCMP");
            return ExecResult::NotImplemented;
        },
        Operation::FLOATCMP(floatcmp) => {
            log::trace!("unimplemented FLOATCMP");
            return ExecResult::NotImplemented;
        },
        Operation::FLOATDP1(floatdp1) => {
            log::trace!("unimplemented FLOATDP1");
            return ExecResult::NotImplemented;
        },
        Operation::FLOATDP2(floatdp2) => {
            log::trace!("unimplemented FLOATDP2");
            return ExecResult::NotImplemented;
        },
        Operation::FLOATDP3(floatdp3) => {
            log::trace!("unimplemented FLOATDP3");
            return ExecResult::NotImplemented;
        },
        Operation::FLOATIMM(floatimm) => {
            log::trace!("unimplemented FLOATIMM");
            return ExecResult::NotImplemented;
        },
        Operation::FLOATSEL(floatsel) => {
            log::trace!("unimplemented FLOATSEL");
            return ExecResult::NotImplemented;
        },
        Operation::IC_SYSTEM(ic_system) => {
            log::trace!("unimplemented IC_SYSTEM");
            return ExecResult::NotImplemented;
        },
        Operation::LDSTEXCL(ldstexcl) => {
            log::trace!("unimplemented LDSTEXCL");
            return ExecResult::NotImplemented;
        },
        Operation::LDSTPAIR_INDEXED(ldstpair_indexed) => {
            log::trace!("unimplemented LDSTPAIR_INDEXED");
            return ExecResult::NotImplemented;
        },
        Operation::LDSTPAIR_OFF(ldstpair_off) => {
            log::trace!("unimplemented LDSTPAIR_OFF");
            return ExecResult::NotImplemented;
        },
        Operation::LDST_IMM10(ldst_imm10) => {
            log::trace!("unimplemented LDST_IMM10");
            return ExecResult::NotImplemented;
        },
        Operation::LDST_IMM9(ldst_imm9) => {
            log::trace!("unimplemented LDST_IMM9");
            return ExecResult::NotImplemented;
        },
        Operation::LDST_POS(ldst_pos) => {
            log::trace!("unimplemented LDST_POS");
            return ExecResult::NotImplemented;
        },
        Operation::LDST_REGOFF(ldst_regoff) => {
            log::trace!("unimplemented LDST_REGOFF");
            return ExecResult::NotImplemented;
        },
        Operation::LDST_UNPRIV(ldst_unpriv) => {
            log::trace!("unimplemented LDST_UNPRIV");
            return ExecResult::NotImplemented;
        },
        Operation::LDST_UNSCALED(ldst_unscaled) => {
            log::trace!("unimplemented LDST_UNSCALED");
            return ExecResult::NotImplemented;
        },
        Operation::LOADLIT(loadlit) => {
            log::trace!("unimplemented LOADLIT");
            return ExecResult::NotImplemented;
        },
        Operation::LOG_IMM(log_imm) => {
            log::trace!("unimplemented LOG_IMM");
            return ExecResult::NotImplemented;
        },
        Operation::LOG_SHIFT(log_shift) => {
            log::trace!("unimplemented LOG_SHIFT");
            return ExecResult::NotImplemented;
        },
        Operation::MOVEWIDE(movewide) => {
            log::trace!("unimplemented MOVEWIDE");
            return ExecResult::NotImplemented;
        },
        Operation::PCRELADDR(pcreladdr) => {
            log::trace!("unimplemented PCRELADDR");
            return ExecResult::NotImplemented;
        },
        Operation::SME2_MOV(sme2_mov) => {
            log::trace!("unimplemented SME2_MOV");
            return ExecResult::NotImplemented;
        },
        Operation::SME2_MOVAZ(sme2_movaz) => {
            log::trace!("unimplemented SME2_MOVAZ");
            return ExecResult::NotImplemented;
        },
        Operation::SME_FP_SD(sme_fp_sd) => {
            log::trace!("unimplemented SME_FP_SD");
            return ExecResult::NotImplemented;
        },
        Operation::SME_INT_SD(sme_int_sd) => {
            log::trace!("unimplemented SME_INT_SD");
            return ExecResult::NotImplemented;
        },
        Operation::SME_LDR(sme_ldr) => {
            log::trace!("unimplemented SME_LDR");
            return ExecResult::NotImplemented;
        },
        Operation::SME_MISC(sme_misc) => {
            log::trace!("unimplemented SME_MISC");
            return ExecResult::NotImplemented;
        },
        Operation::SME_MOV(sme_mov) => {
            log::trace!("unimplemented SME_MOV");
            return ExecResult::NotImplemented;
        },
        Operation::SME_SIZE_22(sme_size_22) => {
            log::trace!("unimplemented SME_SIZE_22");
            return ExecResult::NotImplemented;
        },
        Operation::SME_SIZE_22_HSD(sme_size_22_hsd) => {
            log::trace!("unimplemented SME_SIZE_22_HSD");
            return ExecResult::NotImplemented;
        },
        Operation::SME_STR(sme_str) => {
            log::trace!("unimplemented SME_STR");
            return ExecResult::NotImplemented;
        },
        Operation::SVE2_URQVS(sve2_urqvs) => {
            log::trace!("unimplemented SVE2_URQVS");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_LIMM(sve_limm) => {
            log::trace!("unimplemented SVE_LIMM");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_MISC(sve_misc) => {
            log::trace!("unimplemented SVE_MISC");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_MOVPRFX(sve_movprfx) => {
            log::trace!("unimplemented SVE_MOVPRFX");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_PRED_ZM(sve_pred_zm) => {
            log::trace!("unimplemented SVE_PRED_ZM");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_SHIFT_PRED(sve_shift_pred) => {
            log::trace!("unimplemented SVE_SHIFT_PRED");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_SHIFT_UNPRED(sve_shift_unpred) => {
            log::trace!("unimplemented SVE_SHIFT_UNPRED");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_SIZE_BHS(sve_size_bhs) => {
            log::trace!("unimplemented SVE_SIZE_BHS");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_SIZE_BHSD(sve_size_bhsd) => {
            log::trace!("unimplemented SVE_SIZE_BHSD");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_SIZE_HSD(sve_size_hsd) => {
            log::trace!("unimplemented SVE_SIZE_HSD");
            return ExecResult::NotImplemented;
        },
        Operation::SVE_SIZE_SD(sve_size_sd) => {
            log::trace!("unimplemented SVE_SIZE_SD");
            return ExecResult::NotImplemented;
        },
        Operation::TESTBRANCH(testbranch) => {
            log::trace!("unimplemented TESTBRANCH");
            return ExecResult::NotImplemented;
        },
    };
}
