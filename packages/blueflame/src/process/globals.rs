
use crate::memory::{self, Ptr};
use crate::structs::PauseMenuDataMgr;
use crate::process::Process;

impl Process {
    /// Read the global PauseMenuDataMgr instance pointer from memory
    pub fn global_pmdm(&self) -> Result<Ptr<PauseMenuDataMgr>, memory::Error> {
        todo!()
    }
}
