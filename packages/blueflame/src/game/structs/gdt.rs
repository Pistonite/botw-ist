#![allow(non_snake_case)]

use crate::env::DlcVer;
use crate::memory::{self, MemObject, Memory, Ptr, mem};

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

impl Ptr![AocManager] {
    /// Set the DLC version. Also updates the memory environment
    pub fn set_dlc_version(self, ver: u32, memory: &mut Memory) -> Result<(), memory::Error> {
        mem! { memory:
            *(&self->mVersion) = (ver * 0x100);
        }
        let mut env = memory.env();
        env.dlc_ver = DlcVer::from_num(ver).unwrap_or(DlcVer::V300);
        memory.set_env(env);
        Ok(())
    }
}
