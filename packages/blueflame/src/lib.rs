#![feature(string_from_utf8_lossy_owned)]

extern crate self as blueflame;

/// Top-level setup implementation for starting and connecting
/// everything in a program
#[cfg(all(feature = "disarm", feature = "data"))]
pub mod linker;

/// Implementation of the processor (CPU) layers
#[cfg(all(feature = "disarm", feature = "data"))]
pub mod processor;

/// Mid-level simulation of some of the game's types and systems
#[cfg(feature = "data")]
pub mod game;

/// Low-level memory emulation
pub mod memory;

/// Utilities for handling BlueFlame program images
pub mod program;

/// Virtual machine to execute logical main module gadgets
pub mod vm;

/// Shared environment utilities
pub mod env;

#[cfg(test)]
pub mod test_utils;

/// Re-export other crates to use in macros
#[doc(hidden)]
pub mod __re {
    pub use enumset;
    #[allow(unused_imports)] // release mode only
    pub use no_panic;
    pub use phf;
    pub use static_assertions;
}
