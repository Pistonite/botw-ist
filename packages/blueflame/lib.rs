#![allow(unused)]

extern crate self as blueflame;

#[layered_crate::layers]
mod src {
    /// Simulation of the game's data and functions
    #[depends_on(processor)]
    #[depends_on(memory)]
    #[depends_on(vm)]
    #[depends_on(env)]
    pub extern crate game;

    /// Implementation of the processor (CPU), including the ARMv8 architecture
    #[depends_on(memory)]
    #[depends_on(env)]
    pub extern crate processor;

    /// Low-level memory emulation
    #[depends_on(program)]
    #[depends_on(env)]
    pub extern crate memory;

    /// Utilities for handling BlueFlame program images
    #[depends_on(env)]
    pub extern crate program;

    /// Virtual machine to execute logical main module gadgets
    #[depends_on(env)]
    pub extern crate vm;

    /// Shared environment utilities
    pub extern crate env;

    #[cfg(test)]
    pub extern crate test_utils;
}

/// Re-export other crates to use in macros
#[doc(hidden)]
pub(crate) mod __re {
    pub use enumset;
    pub use no_panic;
    pub use phf;
    pub use static_assertions;
}
