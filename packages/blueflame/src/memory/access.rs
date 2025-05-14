use enumset::{enum_set, EnumSet, EnumSetType};

/// Information for accessing memory for tracking and reporting
#[derive(Debug, Clone)]
pub struct MemAccess {
    /// The type of access
    pub typ: AccessType,
    /// The physical address being accessed
    pub addr: u64,
    /// The number of bytes being accessed
    pub bytes: u32,
}

impl std::fmt::Display for MemAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} access to 0x{:x} for {} bytes",
            self.typ, self.addr, self.bytes
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
