use crate::util::Environment;

/// For getting global variables
mod globals;

/// The Process is the container for everything the core tracks
/// that is not in the Processor.
pub struct Process {
    env: Environment
    
}

