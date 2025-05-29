#[layered_crate::import]
use memory::AccessFlags;

/// Memory errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("unable to construct section: {0}")]
    SectionConstruction(String),

    #[error(
        "[mem-strict-heap] attempt to access part of the heap that is not allocated: 0x{0:016x}, flags: {1}"
    )]
    HeapUnallocated(u64, AccessFlags),
    #[error(
        "[mem-strict-section] attempt to access invalid memory that is not in any section: 0x{0:016x}, flags: {1}"
    )]
    InvalidSection(u64, AccessFlags),
    #[error("[mem-permission] permission denied: 0x{0:016x}, flags: {1}")]
    PermissionDenied(u64, AccessFlags),
    #[error(
        "attempting to access across boundary of a page or valid memory region: 0x{0:016x}, flags: {1}"
    )]
    Boundary(u64, AccessFlags),

    /// An invalid memory access was made, but it was bypassed,
    /// do whatever you want since there's nothing to be accessed
    #[error("")]
    Bypassed,

    #[error("size mismatch in {0}: expected: 0x{1:x}, got 0x{2:x}")]
    SizeAssert(String, u32, u32),
    #[error("size out of range in {0}: expected: 0x{1:x} <= SIZE <= 0x{2:x}, got 0x{3:x}")]
    SizeRangeAssert(String, u32, u32, u32),

    #[error("heap out of memory")]
    HeapOutOfMemory,

    #[error("proxy object is too small: {0} bytes, need at least 4 bytes")]
    InvalidProxyObjectSize(u32),
    #[error("proxy object 0x{1:08x} is corrupted: handle {0} is invalid")]
    InvalidProxyHandle(u32, u64),
    #[error("proxy object 0x{1:08x}#{0} is corrupted: written outside of proxy. size: {2}")]
    CorruptedProxyObject(u32, u64, u32),
    #[error("too many proxy objects")]
    ProxyOutOfMemory,
}
