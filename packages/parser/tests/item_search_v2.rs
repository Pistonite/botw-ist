mod item_search_common;
use item_search_common::test_item_search;
use skybook_parser::cir;
use skybook_parser::search::{ResolvedItem, search_item_by_ident, search_item_by_ident_all};

#[test]
fn test_item_search_empty() {
    assert_eq!(search_item_by_ident(""), None);
    assert_eq!(search_item_by_ident(" "), None);
    assert_eq!(search_item_by_ident("_"), None);
    assert_eq!(search_item_by_ident("-"), None);
    assert_eq!(search_item_by_ident_all(""), vec![]);
    assert_eq!(search_item_by_ident_all(" "), vec![]);
    assert_eq!(search_item_by_ident_all("_"), vec![]);
    assert_eq!(search_item_by_ident_all("-"), vec![]);
}

/// Test items with hard coded names in V2
///
/// For example core -> AncientCore
#[test]
fn test_item_search_v2() {
    test_item_search("slate", "Obj_DRStone_Get");

    test_item_search("glider", "PlayerStole2");
    test_item_search("spiritorb", "Obj_DungeonClearSeal");
    test_item_search("lotus", "Item_Fruit_E");
    test_item_search("silentprincess", "Item_PlantGet_J");
    test_item_search("honey", "BeeHome");
    test_item_search("acorn", "Item_Fruit_K");
    test_item_search("faroshscale", "Item_Enemy_53");
    test_item_search("faroshclaw", "Item_Enemy_54");
    test_item_search("faroshhorn", "Item_Enemy_56");
    test_item_search("heartybass", "Item_FishGet_B");
    test_item_search("beetle", "Animal_Insect_AA");
    test_item_search("opal", "Item_Ore_E");
    test_item_search("diamond", "Item_Ore_A");
    test_item_search("tail", "Item_Enemy_05");
    test_item_search("spring", "Item_Enemy_28");
    test_item_search("shaft", "Item_Enemy_29");
    test_item_search("core", "Item_Enemy_30");
    test_item_search("wood", "Obj_FireWoodBundle");
    test_item_search("rushroom", "Item_MushroomGet_D");
    test_item_search("screw", "Item_Enemy_27");
    test_item_search("hyrulebass", "Item_FishGet_A");
    test_item_search("lizalfoshorn", "Item_Enemy_03");
    test_item_search("lizalfostalon", "Item_Enemy_04");

    test_item_search("normalarrow", "NormalArrow");
    test_item_search("firearrow", "FireArrow");
    test_item_search("icearrow", "IceArrow");
    test_item_search("shockarrow", "ElectricArrow");
    test_item_search("bombarrow", "BombArrow_A");
    test_item_search("ancientarrow", "AncientArrow");

    test_item_search("apple", "Item_Fruit_A");
    test_item_search("hylianshroom", "Item_Mushroom_E");
    test_item_search("spicypepper", "Item_Fruit_I");
    test_item_search("endurashroom", "Item_Mushroom_O");
    test_item_search("heartyradish", "Item_PlantGet_B");
    test_item_search("bigheartyradish", "Item_PlantGet_C");
    test_item_search("fairy", "Animal_Insect_F");
    test_item_search("masterSword", "Weapon_Sword_070");
    test_item_search("zoraarmor", "Armor_006_Upper");

    // Legacy food. Need to add metadata
    assert_eq!(
        search_item_by_ident("speedfood"),
        Some(ResolvedItem {
            actor: "Item_Cook_A_03".to_string(),
            meta: Some(cir::ItemMeta {
                effect_id: Some(13),
                ..Default::default()
            })
        })
    );

    assert_eq!(
        search_item_by_ident("endurafood"),
        Some(ResolvedItem {
            actor: "Item_Cook_A_01".to_string(),
            meta: Some(cir::ItemMeta {
                effect_id: Some(15),
                ..Default::default()
            })
        })
    );
}
