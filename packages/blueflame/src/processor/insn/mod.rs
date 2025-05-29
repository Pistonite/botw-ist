mod insn_vec;
pub use insn_vec::*;

mod op;
pub use op::*;

mod arithmetic_utils;
mod instruction_parse;
pub use instruction_parse::AuxiliaryOperation;
mod instruction_registry;
mod instructions;

pub use blueflame_proc_macros::paste_insn;

// TODO --cleanup: remove this
#[derive(derive_more::derive::Constructor)]
pub struct Core<'a, 'b> {
    cpu: &'a mut crate::processor::Cpu0,
    proc: &'b mut crate::processor::Process,
}
