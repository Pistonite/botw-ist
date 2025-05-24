use enumset::{enum_set, EnumSet, EnumSetType};

use super::Unsigned32;

#[doc(inline)]
pub use blueflame_macros::access;

/// Information for accessing memory for tracking and reporting
#[derive(Debug, Clone)]
pub struct MemAccess {
    /// The type of access
    pub flags: AccessFlags,
    /// The physical address being accessed
    pub addr: u64,
    /// The number of bytes being accessed
    pub bytes: u32,
}

impl std::fmt::Display for MemAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO --cleanup: fix debug and display
        write!(
            f,
            "{:?} access to 0x{:x} for {} bytes",
            self.flags, self.addr, self.bytes
        )
    }
}

#[derive(Debug, EnumSetType)]
pub enum AccessType {
    /// Reading data from the memory
    Read,
    /// Writing data to the memory
    Write,
    /// Reading instruction from memory
    Execute,
}


impl AccessType {
    /// Convert a permission bitmask to a permission set
    /// The mask is:
    /// - 0x4 for read
    /// - 0x2 for write
    /// - 0x1 for execute
    pub const fn from_perms(perm: u32) -> EnumSet<Self> {
        match perm {
            0x4 => enum_set!(AccessType::Read),
            0x2 => enum_set!(AccessType::Write),
            0x1 => enum_set!(AccessType::Execute),
            0x6 => enum_set!(AccessType::Read | AccessType::Write),
            0x5 => enum_set!(AccessType::Read | AccessType::Execute),
            0x3 => enum_set!(AccessType::Write | AccessType::Execute),
            0x7 => enum_set!(AccessType::Read | AccessType::Write | AccessType::Execute),
            _ => EnumSet::empty(),
        }
    }

    pub fn to_perm(&self) -> u32 {
        match self {
            AccessType::Read => 0x4,
            AccessType::Write => 0x2,
            AccessType::Execute => 0x1,
        }
    }

    pub fn to_perms(set: &EnumSet<Self>) -> u32 {
        let mut perms = 0;
        for perm in set.iter() {
            perms |= perm.to_perm();
        }
        perms
    }
}

/// Flags used for various memory access checks and parameters
///
/// Use the [`access`] macro to create an `AccessFlags` instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AccessFlag {
    /// Access for execute
    Execute = 0x1,
    /// Access for write
    Write = 0x2,
    /// Access for read
    Read = 0x4,


    /// Access the .text (RX) region of the program
    Text = 0x20,
    /// Access the .rodata (RO) region of the program
    Rodata = 0x40,
    /// Access the data or bss (RW) region of the program
    Data = 0x80,
    /// Access the stack
    Stack = 0x100,
    /// Access the heap
    Heap = 0x200,
}

// TODO --cleanup: fix debug and display
#[derive(Debug)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct AccessFlags(u32);

impl From<AccessFlag> for u32 {
    fn from(flags: AccessFlag) -> Self {
        flags as u32
    }
}

static_assertions::assert_eq_size!(AccessFlag, AccessFlags);

impl From<AccessFlag> for AccessFlags {
    fn from(flags: AccessFlag) -> Self {
        AccessFlags(flags as u32)
    }
}

impl<T: Unsigned32> From<T> for AccessFlags {
    fn from(flags: T) -> Self {
        AccessFlags(flags.to_u32())
    }
}

impl From<AccessFlags> for u32 {
    fn from(flags: AccessFlags) -> Self {
        flags.0
    }
}

impl std::ops::BitOr for AccessFlags {
    type Output = AccessFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        AccessFlags(self.0 | rhs.0)
    }
}

impl std::ops::BitOr<AccessFlag> for AccessFlags {
    type Output = AccessFlags;

    fn bitor(self, rhs: AccessFlag) -> Self::Output {
        AccessFlags(self.0 | rhs as u32)
    }
}

impl std::ops::BitOr for AccessFlag {
    type Output = AccessFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        AccessFlags(self as u32 | rhs as u32)
    }
}

impl std::ops::BitOr<AccessFlags> for AccessFlag {
    type Output = AccessFlags;

    fn bitor(self, rhs: AccessFlags) -> Self::Output {
        AccessFlags(self as u32 | rhs.0)
    }
}

impl AccessFlags {
    #[inline(always)]
    pub const fn default_const() -> Self {
        Self(0)
    }
    /// Create flags for access the memory for executing
    #[inline(always)]
    pub const fn execute() -> Self {
        Self(AccessFlag::Execute as u32 | AccessFlag::Read as u32 | Self::region_executable().0)
    }

    /// Create flags for access the memory for reading from any region
    #[inline(always)]
    pub const fn read() -> Self {
        Self(AccessFlag::Read as u32 | Self::region_all().0)
    }

    /// Create flags for access the memory for writing to any region
    #[inline(always)]
    pub const fn write() -> Self {
        Self(AccessFlag::Write as u32 | Self::region_writable().0)
    }

    /// Create flags with all region bits on
    #[inline(always)]
    pub const fn region_all() -> Self {
        Self(AccessFlag::Text as u32 | AccessFlag::Rodata as u32 | AccessFlag::Data as u32 | AccessFlag::Stack as u32 | AccessFlag::Heap as u32)
    }

    /// Create flags with region bits on for writable regions
    #[inline(always)]
    pub const fn region_writable() -> Self {
        Self(AccessFlag::Data as u32 | AccessFlag::Stack as u32 | AccessFlag::Heap as u32)
    }

    /// Create flags with region bits on for executable
    #[inline(always)]
    pub const fn region_executable() -> Self {
        Self(AccessFlag::Text as u32)
    }

    /// Create flags with region bits on for program regions
    #[inline(always)]
    pub const fn region_program() -> Self {
        Self(AccessFlag::Text as u32 | AccessFlag::Rodata as u32 | AccessFlag::Data as u32)
    }

    /// Create flags with all permission bits on
    #[inline(always)]
    pub const fn perm_all() -> Self {
        Self(AccessFlag::Read as u32 | AccessFlag::Write as u32 | AccessFlag::Execute as u32)
    }

    /// Convert self to a permission bitmask
    #[inline(always)]
    pub const fn to_perm(self) -> PermFlags {
        PermFlags(Self(self.0 & Self::perm_all().0))
    }

    #[inline(always)]
    pub fn has_all<T: Into<u32>>(self, flags: T) -> bool {
        self.0 & flags.into() == self.0
    }

    #[inline(always)]
    pub fn has_any<T: Into<u32>>(self, flags: T) -> bool {
        self.0 & flags.into() != 0
    }
}

/// Flags wrapper for [`AccessFlags`] indicating only the permission bits are set
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PermFlags(AccessFlags);

impl From<PermFlags> for u32 {
    fn from(x: PermFlags) -> Self {
        x.0.0
    }
}

// can only bitor with other PermFlags because of the invariant
impl std::ops::BitOr for PermFlags {
    type Output = PermFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

// macro helpers
#[rustfmt::skip]
impl PermFlags {
    /// See [`perm`] macro
    #[inline(always)] pub const fn r() -> Self { Self(AccessFlags(AccessFlag::Read as u32)) }
    // #[inline(always)] pub const fn rw() { Self(AccessFlags(AccessFlags::Read as u32 | AccessFlags)) }
}

// container
impl PermFlags {
    /// Get the inner [`AccessFlags`] value
    #[inline(always)]
    pub const fn decay(self) -> AccessFlags {
        self.0
    }

    #[inline(always)]
    pub fn has_all<T: Into<u32>>(self, flags: T) -> bool {
        self.0.0 & flags.into() == self.0.0
    }

    #[inline(always)]
    pub fn has_any<T: Into<u32>>(self, flags: T) -> bool {
        self.0.0 & flags.into() != 0
    }
}
