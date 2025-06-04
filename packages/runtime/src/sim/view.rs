use blueflame::game::singleton_instance;
use blueflame::memory;
use blueflame::processor::Process;

use crate::iv;

/// Read the pouch view from the process memory.
pub fn extract_pouch_view(proc: &Process) -> Result<iv::PouchList, memory::Error> {
    let memory = proc.memory();

    let pmdm = singleton_instance!(pmdm(memory))?;
}
