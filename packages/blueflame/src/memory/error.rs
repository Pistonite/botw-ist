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

    #[error("proxy object is too small: {0} bytes, need at least 4 bytes")]
    InvalidProxyObjectSize(u32),
    #[error("proxy object 0x{1:08x} is corrupted: handle {0} is invalid")]
    InvalidProxyHandle(u32, u64),
    #[error("proxy object 0x{1:08x}#{0} is corrupted: written outside of proxy. size: {2}")]
    CorruptedProxyObject(u32, u64, u32),
    #[error("too many proxy objects")]
    ProxyOutOfMemory,

    #[error("unexpected error: {0}")]
    Unexpected(String),


}
