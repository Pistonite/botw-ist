extern crate self as blueflame;

#[layered_crate::layers]
mod src {
    // /// High-level facade for the processor to access different parts of the process (memory, etc)
    // #[depends_on(memory)]
    // pub extern crate process;



    /// Information and utilities for initializing singletons in the game
    #[depends_on(processor)]
    #[depends_on(memory)]
    #[depends_on(env)]
    pub extern crate singleton;

    /// High-level implementation of the processor (CPU)
    #[depends_on(env)]
    pub extern crate processor;

    /// Types for interfacing with structs used in the game's code
    #[depends_on(memory)]
    pub extern crate structs;

    /// Low-level memory emulation
    #[depends_on(program)]
    #[depends_on(env)]
    pub extern crate memory;

    /// Utilities for handling BlueFlame program images
    #[depends_on(env)]
    pub extern crate program;

    /// Shared environment utilities
    pub extern crate env;


}

/// Re-export other crates to use in macros
#[doc(hidden)]
pub(crate) mod __re {
    pub use enumset;
    pub use phf;
    pub use static_assertions;
}
