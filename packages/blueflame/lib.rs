#![feature(string_from_utf8_lossy_owned)]

extern crate self as blueflame;

#[layered_crate::layers]
mod src {
    /// Top-level setup implementation for starting and connecting
    /// everything in a program
    #[depends_on(processor)]
    #[depends_on(game)]
    #[depends_on(memory)]
    #[depends_on(env)]
    pub extern crate linker;

    /// Implementation of the processor (CPU) layers
    #[depends_on(game)]
    #[depends_on(memory)]
    #[depends_on(env)]
    pub extern crate processor;

    /// Mid-level simulation of some of the game's types and systems
    #[depends_on(memory)]
    #[depends_on(vm)]
    #[depends_on(env)]
    pub extern crate game;

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
