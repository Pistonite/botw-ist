use crate::memory::MemObject;

#[derive(MemObject, Default, Clone, Copy)]
#[size(0x98)]
pub struct ActorInfoData {
    #[offset(0x0)]
    vtable: u64,
}
