#![feature(string_from_utf8_lossy_owned)]
#![feature(optimize_attribute)]
#![feature(proc_macro_hygiene)]

extern crate self as blueflame;

#[layered_crate::layers]
mod src {
    /// Top-level setup implementation for starting and connecting
    /// everything in a program
    #[depends_on(processor)]
    #[depends_on(game)]
    #[depends_on(memory)]
    #[depends_on(program)]
    #[depends_on(env)]
    #[cfg(feature = "data")]
    pub mod linker;

    /// Implementation of the processor (CPU) layers
    #[depends_on(game)]
    #[depends_on(memory)]
    #[depends_on(program)]
    #[depends_on(vm)]
    #[depends_on(env)]
    #[cfg(feature = "data")]
    pub mod processor;

    /// Mid-level simulation of some of the game's types and systems
    #[depends_on(memory)]
    #[depends_on(vm)]
    #[depends_on(env)]
    #[cfg(feature = "data")]
    pub mod game;

    /// Low-level memory emulation
    #[depends_on(program)]
    #[depends_on(env)]
    pub mod memory;

    /// Utilities for handling BlueFlame program images
    #[depends_on(env)]
    pub mod program;

    /// Virtual machine to execute logical main module gadgets
    #[depends_on(env)]
    pub mod vm;

    /// Shared environment utilities
    pub mod env;

    #[cfg(test)]
    pub mod test_utils;
}

/// Re-export other crates to use in macros
#[doc(hidden)]
pub mod __re {
    pub use enumset;
    #[allow(unused_imports)] // release mode only
    pub use no_panic;
    pub use phf;
    pub use static_assertions;
}
