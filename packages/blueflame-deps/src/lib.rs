#![cfg_attr(feature = "data", feature(optimize_attribute))]

mod access;
mod align;
mod asserts;
pub mod gdt;
mod log;
mod pointer;
mod proxy;
mod register;
mod singleton;

#[cfg(feature = "data")]
pub mod generated {
    #[rustfmt::skip]
    pub mod gdt;
    #[rustfmt::skip]
    pub mod actor;
}

#[cfg(feature = "data")]
pub mod actor;
