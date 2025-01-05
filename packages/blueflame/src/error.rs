use crate::memory;

pub enum Crash {

    Boot, // TODO: Boot crash (?)

    Cpu, // TODO: CPU errors

    /// Memory error
    Mem(memory::error::Error),
}
