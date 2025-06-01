#![allow(non_snake_case)]

use crate::memory::MemObject;

#[derive(MemObject, Default, Clone)]
#[size(0xdc8)]
pub struct GdtManager {
    #[offset(0xc00)]
    pub mFlagBuffer: u64,
}
