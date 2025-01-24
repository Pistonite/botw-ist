#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
pub enum Bytecode {
    /// Enter the bytecode execution.
    ///
    /// The executor should simulate a branch-and-link to the target address
    /// (relative to start of the main module).
    /// This is always the first instruction in the bytecode.
    #[cfg_attr(feature = "deku", deku(id = 0x01))]
    Enter(u32),

    /// Set the high 32 bits of a register, and clear the low 32 bits.
    ///
    /// The first argument is the register number, and the second is the value.
    /// The register number is 0-30 for X0 to X30
    #[cfg_attr(feature = "deku", deku(id = 0x11))]
    SetRegHi(u8, u32),

    /// Set the low 32 bits of a register and clear the high 32 bits.
    ///
    /// The first argument is the register number, and the second is the value.
    /// The register number is 0-30 for X0 to X30
    #[cfg_attr(feature = "deku", deku(id = 0x12))]
    SetRegLo(u8, u32),

    /// The low 32 bits of a register, and the next instruction will set the register
    /// together with the high 32 bits.
    ///
    /// The first argument is the register number, and the second is the value.
    /// The register number is 0-30 for X0 to X30
    #[cfg_attr(feature = "deku", deku(id = 0x13))]
    RegLoNextHi(u32),

    /// Copy first register to second register.
    ///
    /// Either argument could be 0-30 for X0 to X30, or 32-63 for floating
    /// point registers S0 to S31. When moving from X to S or vice versa,
    /// the bits should be copied exactly.
    #[cfg_attr(feature = "deku", deku(id = 0x14))]
    CopyReg(u8, u8),

    /// Execute the program, and return when the next instruction
    /// is at the target (address relative to the start of the main module).
    #[cfg_attr(feature = "deku", deku(id = 0x21))]
    ExecuteUntil(u32),

    /// Set the PC to relative to the start of the main module, without
    /// doing anything else
    #[cfg_attr(feature = "deku", deku(id = 0x22))]
    Jump(u32),

    /// Equivalent to `ExecuteUntil(X); Jump(X + 4)`
    #[cfg_attr(feature = "deku", deku(id = 0x23))]
    ExecuteUntilThenSkipOne(u32),

    /// Equivalent to `ExecuteUntil(X); AllocateSingleton; Jump(X + 4)`
    #[cfg_attr(feature = "deku", deku(id = 0x24))]
    ExecuteUntilThenAllocSingletonSkipOne(u32),

    /// Equivalent to `Jump(X); ExecuteUntil(X + 4)`
    #[cfg_attr(feature = "deku", deku(id = 0x25))]
    JumpExecute(u32),

    /// Allocate X bytes of memory, and put the address in X0
    #[cfg_attr(feature = "deku", deku(id = 0x31))]
    Allocate(u32),

    /// Allocate a new proxy object of the type, and put the address in X0
    #[cfg_attr(feature = "deku", deku(id = 0x32))]
    AllocateProxy(ProxyType),

    /// Put raw data in memory, and put the address in X0
    #[cfg_attr(feature = "deku", deku(id = 0x33))]
    AllocateData(DataType),

    /// Allocate the singleton, put the address in X0
    #[cfg_attr(feature = "deku", deku(id = 0x34))]
    AllocateSingleton,

    /// Put the address of the singleton in the register.
    ///
    /// The reguster is 0-30 for X0 to X30
    #[cfg_attr(feature = "deku", deku(id = 0x35))]
    GetSingleton(u8),

    /// Execute the program until jumping out of the function
    /// initially jumpped into with Enter
    #[cfg_attr(feature = "deku", deku(id = 0x41))]
    ExecuteToComplete,
}
// make sure the binary size doesn't explode
static_assertions::assert_eq_size!(Bytecode, u64);

/// Proxy type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
#[repr(u8)]
pub enum ProxyType {
    /// ksys::gdt::TriggerParam, the storage for game data flags
    #[cfg_attr(feature = "deku", deku(id = 0x01))]
    TriggerParam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deku", derive(deku::DekuRead, deku::DekuWrite))]
#[cfg_attr(feature = "deku", deku(id_type = "u8"))]
#[repr(u8)]
pub enum DataType {
    /// Actor/ActorInfo.product.byml (decompressed version of the sbyml)
    #[cfg_attr(feature = "deku", deku(id = 0x01))]
    ActorInfoByml,
}
