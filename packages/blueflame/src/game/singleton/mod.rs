
macro_rules! __impl {
    ($( pub mod $xxx:ident; )*) => {
        mod _impl {
            $(
                pub mod $xxx;
            )*
        }
        pub use _impl::*;

        // generate functions that depends on all singletons

        /// Get the rel start of the singleton with the max relative start address.
        pub const fn get_max_rel_start(env: crate::env::Environment) -> u32 {
            let mut out = 0;
            $(
                let x = $xxx::rel_start(env);
                if x > out {
                    out = x;
                }
            )*
            out
        }

        /// Get the rel start of the singleton with the min relative start address.
        pub const fn get_min_rel_start(env: crate::env::Environment) -> u32 {
            let mut out = u32::MAX;
            $(
                let x = $xxx::rel_start(env);
                if x < out {
                    out = x;
                }
            )*
            out
        }
        
    };
}

__impl! {
    // uking::aoc::Manager
    pub mod aocm;
    // uking::ui::PauseMenuDataMgr
    pub mod pmdm;

    // ksys::gdt::Manager
    pub mod gdtm;
    // ksys::act::InfoData
    pub mod info_data;
}

mod singleton_info;
pub use singleton_info::*;

pub use blueflame_deps::singleton_instance;
