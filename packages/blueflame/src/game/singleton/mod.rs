mod _impl {
    /// uking::aoc::Manager
    pub mod aocm;
    /// uking::ui::PauseMenuDataMgr
    pub mod pmdm;

    /// ksys::gdt::Manager
    pub mod gdtm;
    /// ksys::act::InfoData
    pub mod info_data;
}
pub use _impl::*;

mod singleton_info;
pub use singleton_info::*;

pub use blueflame_macros::singleton_instance;
