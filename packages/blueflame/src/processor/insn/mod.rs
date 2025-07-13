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

#[allow(unused)] // used only in tests
macro_rules! decode {
    ($($x:tt)*) => {
        disarm64::decoder::decode(
            $crate::processor::insn::paste_insn!($($x)*)
        ).expect(concat!("failed to decode ", stringify!($($x)*)))
    };
}
#[allow(unused)] // used only in tests
pub(crate) use decode;

// TODO --cleanup: remove this
#[derive(derive_more::derive::Constructor)]
pub struct Core<'a, 'b> {
    cpu: &'a mut crate::processor::Cpu0,
    proc: &'b mut crate::processor::Process,
}
