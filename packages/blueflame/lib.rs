
#[layered_crate::layers]
mod src {
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

/// Proc-macro implementation
#[doc(inline)]
pub use blueflame_macros as macros;
