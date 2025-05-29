#[layered_crate::import]
use memory::MemObject;

#[allow(non_snake_case)]
#[derive(MemObject, Default, Clone)]
#[size(0xdc8)]
pub struct GdtManager {
    #[offset(0xc00)]
    pub mFlagBuffer: u64,
}
