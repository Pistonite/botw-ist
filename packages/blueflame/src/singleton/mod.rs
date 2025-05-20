
mod _impl;
pub use _impl::*;

pub struct SingletonInfo {
    /// Name of the singleton for debugging purposes
    pub name: &'static str,

    /// Start of the singleton relative to root heap
    pub rel_start: u32,

    /// Size of the singleton in bytes
    pub size: u32,

    /// Offset of the instance static variable compared to the
    /// start of the main module (i.e. `*((main + main_offset) as u64*)` is the
    /// pointer to the singleton instance)
    pub main_offset: u32,
}

/// Get the singleton info for the given path
macro_rules! singleton_info {
    ($($path:ident)::* ( $env:expr )) => {
        $crate::singleton::SingletonInfo {
            name: $($path)::*::NAME,
            rel_start: $($path)::*::rel_start($env),
            size: $($path)::*::size($env),
            main_offset: $($path)::*::main_offset($env),
        }
    };
}
pub(crate) use singleton_info;

#[cfg(test)]
mod tests {
    use crate::singleton::{self as self_, crate_};

    use self_::SingletonInfo;
    use crate_::env::{DlcVer, Environment, GameVer};

    #[test]
    fn test_not_overlap() {
        // TODO --160: all environments
        let singletons = get_singletons(Environment::new(GameVer::X150, DlcVer::V300));

        for i in 0..singletons.len() {
            for j in i+1..singletons.len() {
                assert!(!overlaps(
                    singletons[i].rel_start,
                    singletons[i].size,
                    singletons[j].rel_start,
                    singletons[j].size
                ));
            }
        }
    }

    fn get_singletons(env: Environment) -> Vec<SingletonInfo> {
        vec![
            singleton_info!(self_::pmdm(env)),
            singleton_info!(self_::gdtm(env)),
            singleton_info!(self_::info_data(env)),
            singleton_info!(self_::aocm(env)),
        ]
    }

    fn overlaps(start1: u32, size1: u32, start2: u32, size2: u32) -> bool {
        let end1 = start1 + size1;
        let end2 = start2 + size2;
        if start1 >= end2 || start2 >= end1 {
            return false;
        }
        true
    }
}
