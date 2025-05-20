extern crate self as blueflame;

#[layered_crate::layers]
mod src {
    /// Memory emulation
    #[depends_on(env)]
    pub extern crate memory;


    /// Information and utilities for initializing singletons in the game
    #[depends_on(processor)]
    #[depends_on(env)]
    pub extern crate singleton;

    /// High-level implementation of the processor (CPU)
    #[depends_on(env)]
    pub extern crate processor;

    /// Utilities for handling BlueFlame program images
    #[depends_on(env)]
    pub extern crate program;

    /// Shared environment utilities
    pub extern crate env;

}

// currently, our proc-macros are not allowed to be used outside of the crate

/// Proc-macro implementation
#[doc(hidden)]
pub(crate) use blueflame_macros as macros;

/// Re-export other crates to use in proc macros
#[doc(hidden)]
pub(crate) mod __re {
    pub use enumset;
    pub use phf;
}
