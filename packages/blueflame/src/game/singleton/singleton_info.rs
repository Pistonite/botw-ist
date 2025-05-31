pub use blueflame_deps::singleton_info;

// pub use crate::processor::Process;

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

#[cfg(test)]
mod tests {
    use crate::env::{DlcVer, Environment, GameVer};
    use crate::game::singleton;

    #[test]
    fn test_not_overlap() {
        // TODO --160: all environments
        let singletons = singleton::singleton_infos(Environment::new(GameVer::X150, DlcVer::V300));

        for i in 0..singletons.len() {
            for j in i + 1..singletons.len() {
                assert!(!overlaps(
                    singletons[i].rel_start,
                    singletons[i].size,
                    singletons[j].rel_start,
                    singletons[j].size
                ));
            }
        }
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
