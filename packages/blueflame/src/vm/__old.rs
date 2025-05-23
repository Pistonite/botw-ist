// TODO --cleanup
use serde::{Deserialize, Serialize};
use deku::{DekuRead, DekuWrite};

use super::ids::{DataId, ProxyId};

/// Bytecode for creating our own program by using gadgets in the existing program,
/// for example, to run a partially stubbed out function
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Bytecode {
    /// Enter the bytecode execution.
    ///
    /// The executor should simulate a branch-and-link to the target address
    /// (relative to start of the main module).
    /// This is always the first instruction in the bytecode.
    #[deku(id = 0x01)]
    Enter(u32),

    /// Set the high 32 bits of a register, and clear the low 32 bits.
    ///
    /// The first argument is the register number, and the second is the value.
    /// The register number is 0-30 for X0 to X30
    #[deku(id = 0x11)]
    SetRegHi(u8, u32),

    /// Set the low 32 bits of a register and clear the high 32 bits.
    ///
    /// The first argument is the register number, and the second is the value.
    /// The register number is 0-30 for X0 to X30
    #[deku(id = 0x12)]
    SetRegLo(u8, u32),

    /// The low 32 bits of a register, and the next instruction will set the register
    /// together with the high 32 bits.
    ///
    /// The first argument is the register number, and the second is the value.
    /// The register number is 0-30 for X0 to X30
    #[deku(id = 0x13)]
    RegLoNextHi(u32),

    /// Copy first register to second register.
    ///
    /// Either argument could be 0-30 for X0 to X30, or 32-63 for floating
    /// point registers S0 to S31. When moving from X to S or vice versa,
    /// the bits should be copied exactly.
    #[deku(id = 0x14)]
    CopyReg(u8, u8),

    /// Execute the program, and return when the next instruction
    /// is at the target (address relative to the start of the main module).
    #[deku(id = 0x21)]
    ExecuteUntil(u32),

    /// Set the PC to relative to the start of the main module, without
    /// doing anything else
    #[deku(id = 0x22)]
    Jump(u32),

    /// Equivalent to `ExecuteUntil(X); Jump(X + 4)`
    #[deku(id = 0x23)]
    ExecuteUntilThenSkipOne(u32),

    // TODO --cleanup: too big bytecode
    // /// Equivalent to `ExecuteUntil(X); AllocateSingleton; Jump(X + 4)`
    // #[deku(id = 0x24)]
    // ExecuteUntilThenAllocSingletonSkipOne(u32),

    /// Equivalent to `Jump(X); ExecuteUntil(X + 4)`
    #[deku(id = 0x25)]
    JumpExecute(u32),

    /// Allocate X bytes of memory, and put the address in X0
    #[deku(id = 0x31)]
    Allocate(u32),

    /// Allocate a new proxy object of the type, and put the address in X0
    #[deku(id = 0x32)]
    AllocateProxy(ProxyId),

    /// Put raw data in memory, and put the address in X0
    #[deku(id = 0x33)]
    AllocateData(DataId),

    /// Allocate the singleton, put the address in X0
    ///
    /// The first arg is the heap_rel_start, the second is the size of the singleton
    #[deku(id = 0x34)]
    AllocateSingleton(u32, u16),

    /// Put the address of the singleton in the register.
    ///
    /// The reguster is 0-30 for X0 to X30, the second arg
    /// is the heap_rel_start of the singleton
    #[deku(id = 0x35)]
    GetSingleton(u8, u32),

    /// Execute the program until jumping out of the function
    /// initially jumpped into with Enter
    #[deku(id = 0x41)]
    ExecuteToComplete,
}
// make sure the binary size doesn't explode
static_assertions::assert_eq_size!(Bytecode, u64);
