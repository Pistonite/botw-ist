use crate::game::{self as self_, crate_};
use crate_::memory::MemObject;

#[allow(non_snake_case)]
#[derive(MemObject, Default, Clone)]
#[size(0xdc8)]
pub struct GdtManager {
    #[offset(0xc00)]
    pub mFlagBuffer: u64,
}
