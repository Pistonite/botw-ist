
mod insn_vec;
pub use insn_vec::*;



mod arithmetic_utils;
mod instruction_parse;
mod instruction_registry;
mod instructions;


// TODO --cleanup: remove this
#[derive(derive_more::derive::Constructor)]
pub struct Core<'a, 'b> {
    cpu: &'a mut crate::processor::Cpu0,
    proc: &'b mut crate::processor::Process,
}


