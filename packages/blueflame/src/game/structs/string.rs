#[layered_crate::import]
use memory::{self, Memory, MemObject, Ptr};

#[derive(MemObject, Default, Clone, Copy)]
#[size(0x10)]
pub struct SafeString {
    #[offset(0x0)]
    pub vtable: u64,
    #[offset(0x8)]
    pub mStringTop: Ptr![u8],
}

impl Ptr![SafeString] {
    /// Get the cstr pointer (does not ensure termination, like the safe string in game)
    pub fn cstr(self, memory: &Memory) -> Result<Ptr![u8], memory::Error> {
        Ptr!(&self->mStringTop).load(memory)
    }
}
