#![allow(non_snake_case)]

use crate::memory::MemObject;

#[derive(MemObject, Default, Clone)]
#[size(0xdc8)]
pub struct GdtManager {
    #[offset(0xc00)]
    pub mFlagBuffer: u64,
}

// putting this here now, no better place
#[derive(MemObject, Default, Clone)]
#[size(0xd4)]
pub struct AocManager {
    #[offset(0xd0)]
    pub mVersion: u32,
}
