use crate::memory::{MemObject, Ptr};

#[derive(MemObject, Default, Clone, Copy)]
#[size(0x98)]
pub struct ActorInfoData {
    #[offset(0x0)]
    vtable: u64,
    #[offset(0x40)]
    pub mHashesBytes: Ptr![u8],
    #[offset(0x48)]
    pub mHashes: Ptr![u32],
    #[offset(0x50)]
    pub mActorsBytes: Ptr![u8],
    #[offset(0x58)]
    pub mActorOffsets: Ptr![u32],
    #[offset(0x60)]
    pub mTagsIdx: i32,
    #[offset(0x78)]
    pub mNumActors: i32,
}
