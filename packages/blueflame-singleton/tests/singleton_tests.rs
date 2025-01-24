use blueflame_singleton::singleton::{self, Singleton};
use blueflame_singleton::Environment;



#[test]
fn test_not_overlap() {
    let singletons = get_singletons(Environment {
        game_ver: blueflame_singleton::GameVer::X150,
        dlc_ver: blueflame_singleton::DlcVer::None,
    });

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

fn get_singletons(env: Environment) -> Vec<Singleton> {
    vec![
        singleton::pmdm(env),
        singleton::gdt_manager(env),
        singleton::info_data(env),
        singleton::aoc_manager(env),
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
