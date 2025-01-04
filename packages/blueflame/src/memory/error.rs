use enumset::EnumSet;

use super::region::RegionType;
use super::access::MemAccess;

/// Memory errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("permission denied: {0}")]
    PermissionDenied(MemAccess),
    #[error("page boundary hit: {0}")]
    PageBoundary(MemAccess),
    #[error("attempt to access invalid memory region: 0x{0:08x}")]
    InvalidRegion(u64),
    #[error("attempt to access address: 0x{0:08x}, which is not in {1:?}")]
    DisallowedRegion(u64, EnumSet<RegionType>),

    /// Region must be valid, but it's not allocated
    /// (suppressable with :disable mem-check-allocated
    #[error("attempt to access unallocated memory: 0x{0:08x}")]
    Unallocated(u64),
    #[error("{1} region out of memory: 0x{0:08x}")]
    OutOfMemory(u64, RegionType),

    #[error("unexpected error: {0}")]
    Unexpected(String),


}
