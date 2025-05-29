use enumset::{enum_set, EnumSet, EnumSetType};

#[layered_crate::import]
use memory::Unsigned32;

#[doc(inline)]
pub use blueflame_deps::{access, perm, region};

/// Information for accessing memory for tracking and reporting
#[derive(Debug, Clone)]
pub struct MemAccess {
    /// The type of access
    pub flags: AccessFlags,
    /// The physical address being accessed
    pub addr: u64,
    // /// The number of bytes being accessed
    // pub bytes: u32,
}

impl std::fmt::Display for MemAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO --cleanup: fix debug and display
        // write!(
        //     f,
        //     "{:?} access to 0x{:x} for {} bytes",
        //     self.flags, self.addr, self.bytes
        // )
        todo!()
    }
}

// TODO --cleanup
// #[derive(Debug, EnumSetType)]
// pub enum AccessType {
//     /// Reading data from the memory
//     Read,
//     /// Writing data to the memory
//     Write,
//     /// Reading instruction from memory
//     Execute,
// }
//
//
// impl AccessType {
//     /// Convert a permission bitmask to a permission set
//     /// The mask is:
//     /// - 0x4 for read
//     /// - 0x2 for write
//     /// - 0x1 for execute
//     pub const fn from_perms(perm: u32) -> EnumSet<Self> {
//         match perm {
//             0x4 => enum_set!(AccessType::Read),
//             0x2 => enum_set!(AccessType::Write),
//             0x1 => enum_set!(AccessType::Execute),
//             0x6 => enum_set!(AccessType::Read | AccessType::Write),
//             0x5 => enum_set!(AccessType::Read | AccessType::Execute),
//             0x3 => enum_set!(AccessType::Write | AccessType::Execute),
//             0x7 => enum_set!(AccessType::Read | AccessType::Write | AccessType::Execute),
//             _ => EnumSet::empty(),
//         }
//     }
//
//     pub fn to_perm(&self) -> u32 {
//         match self {
//             AccessType::Read => 0x4,
//             AccessType::Write => 0x2,
//             AccessType::Execute => 0x1,
//         }
//     }
//
//     pub fn to_perms(set: &EnumSet<Self>) -> u32 {
//         let mut perms = 0;
//         for perm in set.iter() {
//             perms |= perm.to_perm();
//         }
//         perms
//     }
// }

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

    /// Permission checks are disabled
    Force = 0x8,


    /// Access the .text (RX) section(s) of the program
    Text = 0x20,
    /// Access the .rodata (RO) section(s) of the program
    Rodata = 0x40,
    /// Access the data or bss (RW) section(s) of the program
    Data = 0x80,
    /// Access the stack region
    Stack = 0x100,
    /// Access the heap region
    Heap = 0x200,
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AccessFlags(u32);
static_assertions::assert_eq_size!(AccessFlag, AccessFlags);

impl AccessFlags {
    #[inline(always)]
    pub const fn default_const() -> Self {
        Self(0)
    }

    /// Create flags for access the memory for executing
    #[inline(always)]
    pub const fn execute() -> Self {
        Self(AccessFlag::Execute as u32 | AccessFlag::Read as u32 | __Region::text().0)
    }

    /// Create flags for access the memory for reading from any region
    #[inline(always)]
    pub const fn read() -> Self {
        Self(AccessFlag::Read as u32 | __Region::all().0)
    }

    /// Create flags for access the memory with no permission checks and all regions allowed
    #[inline(always)]
    pub const fn force() -> Self {
        Self(AccessFlag::Force as u32 | __Perm::rwx().0 | __Region::all().0)
    }

    /// Create flags for access the memory for writing to writable regions
    #[inline(always)]
    pub const fn write() -> Self {
        Self(AccessFlag::Write as u32 | __Region::writable().0)
    }

    /// Check if all bits are on
    #[inline(always)]
    pub fn all<T: Into<u32>>(self, flags: T) -> bool {
        let flags = flags.into();
        self.0 & flags == flags
    }

    /// Check if any of the bits is on
    #[inline(always)]
    pub fn any<T: Into<u32>>(self, flags: T) -> bool {
        self.0 & flags.into() != 0
    }

    /// Get the permission bits
    #[inline(always)]
    pub fn perms(self) -> Self {
        Self(self.0 & __Perm::rwx().0)
    }
}


/// Helper to create permission flags.
#[doc(hidden)]
pub struct __Perm;
#[rustfmt::skip]
#[doc(hidden)]
impl __Perm {
    #[inline(always)] pub const fn r() -> AccessFlags { AccessFlags(AccessFlag::Read as u32) }
    #[inline(always)] pub const fn w() -> AccessFlags { AccessFlags(AccessFlag::Write as u32) }
    #[inline(always)] pub const fn x() -> AccessFlags { AccessFlags(AccessFlag::Execute as u32) }
    #[inline(always)] pub const fn rx() -> AccessFlags { AccessFlags(AccessFlag::Execute as u32 | AccessFlag::Read as u32) }
    #[inline(always)] pub const fn rw() -> AccessFlags { AccessFlags(AccessFlag::Read as u32 | AccessFlag::Write as u32) }
    #[inline(always)] pub const fn rwx() -> AccessFlags { AccessFlags(AccessFlag::Read as u32 | AccessFlag::Write as u32 | AccessFlag::Execute as u32) }
}

/// Helper to create region/section flags.
#[doc(hidden)]
pub struct __Region;
#[rustfmt::skip]
#[doc(hidden)]
impl __Region {
    #[inline(always)] pub const fn stack() -> AccessFlags { AccessFlags(AccessFlag::Stack as u32) }
    #[inline(always)] pub const fn heap() -> AccessFlags { AccessFlags(AccessFlag::Heap as u32) }
    #[inline(always)] pub const fn text() -> AccessFlags { AccessFlags(AccessFlag::Text as u32) }
    #[inline(always)] pub const fn rodata() -> AccessFlags { AccessFlags(AccessFlag::Rodata as u32) }
    #[inline(always)] pub const fn data() -> AccessFlags { AccessFlags(AccessFlag::Data as u32) }
    #[inline(always)] pub const fn program() -> AccessFlags { AccessFlags(AccessFlag::Data as u32 | AccessFlag::Rodata as u32 | AccessFlag::Text as u32) }
    #[inline(always)] pub const fn writable() -> AccessFlags { AccessFlags(AccessFlag::Data as u32 | AccessFlag::Stack as u32 | AccessFlag::Heap as u32) }
    #[inline(always)] pub const fn all() -> AccessFlags { AccessFlags(AccessFlag::Data as u32 | AccessFlag::Stack as u32 | AccessFlag::Heap as u32 | AccessFlag::Text as u32 | AccessFlag::Rodata as u32) }
}

// conversion implementations
#[rustfmt::skip]
const _: () = {
    impl From<AccessFlag> for u32 { #[inline(always)] fn from(flag: AccessFlag) -> Self { flag as u32 } }
    impl From<AccessFlags> for u32 { #[inline(always)] fn from(flags: AccessFlags) -> Self { flags.0} }
    impl From<u32> for AccessFlags { #[inline(always)] fn from(flag: u32) -> Self { Self(flag) } }
    impl From<AccessFlag> for AccessFlags { #[inline(always)] fn from(flag: AccessFlag) -> Self { Self(flag as u32) } }
};

// bitwise OR implementations:
#[rustfmt::skip]
const _: () = {
    impl std::ops::BitOr for AccessFlag {
        type Output = AccessFlags;
        #[inline(always)] fn bitor(self, rhs: AccessFlag) -> Self::Output { AccessFlags(self as u32 | rhs as u32) }
    }
    impl std::ops::BitOr for AccessFlags {
        type Output = AccessFlags;
        #[inline(always)] fn bitor(self, rhs: AccessFlags) -> Self::Output { AccessFlags(self.0 as u32 | rhs.0 as u32) }
    }
    impl std::ops::BitOr<AccessFlags> for AccessFlag {
        type Output = AccessFlags;
        #[inline(always)] fn bitor(self, rhs: AccessFlags) -> Self::Output { AccessFlags(self as u32 | rhs.0 as u32) }
    }
    impl std::ops::BitOr<AccessFlag> for AccessFlags {
        type Output = AccessFlags;
        #[inline(always)] fn bitor(self, rhs: AccessFlag) -> Self::Output { AccessFlags(self.0 as u32 | rhs as u32) }
    }
};

impl std::fmt::Display for AccessFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut perms = String::new();
        if self.any(AccessFlag::Read) {
            perms.push('r');
        }
        if self.any(AccessFlag::Write) {
            perms.push('w');
        }
        if self.any(AccessFlag::Execute) {
            perms.push('x');
        }
        if self.any(AccessFlag::Force) {
            perms.push('!');
        }
        if !perms.is_empty() {
            perms = format!("perm={}", perms);
        }

        let mut regions = Vec::new();
        if self.any(AccessFlag::Text) {
            regions.push("text");
        }
        if self.any(AccessFlag::Rodata) {
            regions.push("rodata");
        }
        if self.any(AccessFlag::Data) {
            regions.push("data");
        }
        if self.any(AccessFlag::Stack) {
            regions.push("stack");
        }
        if self.any(AccessFlag::Heap) {
            regions.push("heap");
        }
        let regions = if regions.is_empty() {
            "-".to_string()
        } else {
            regions.join("+")
        };

        write!(f, "{perms},{regions}")
    }
}
