use skybook_parser::search::{ResolvedItem, search_item_by_ident};
use skybook_parser::cir;

/// Test items with hard coded names in V2
///
/// For example core -> AncientCore
#[test]
fn test_item_search_v2() {
    assert_eq!(search_item_by_ident("slate").unwrap().actor, "Obj_DRStone_Get");

    assert_eq!(search_item_by_ident("glider").unwrap().actor, "PlayerStole2");
    assert_eq!(search_item_by_ident("spiritorb").unwrap().actor, "Obj_DungeonClearSeal");
    assert_eq!(search_item_by_ident("lotus").unwrap().actor, "Item_Fruit_E");
    assert_eq!(search_item_by_ident("silentprincess").unwrap().actor, "Item_PlantGet_J");
    assert_eq!(search_item_by_ident("honey").unwrap().actor, "BeeHome");
    assert_eq!(search_item_by_ident("acorn").unwrap().actor, "Item_Fruit_K");
    assert_eq!(search_item_by_ident("faroshscale").unwrap().actor, "Item_Enemy_53");
    assert_eq!(search_item_by_ident("faroshclaw").unwrap().actor, "Item_Enemy_54");
    assert_eq!(search_item_by_ident("faroshhorn").unwrap().actor, "Item_Enemy_56");
    assert_eq!(search_item_by_ident("heartybass").unwrap().actor, "Item_FishGet_B");
    assert_eq!(search_item_by_ident("beetle").unwrap().actor, "Animal_Insect_AA");
    assert_eq!(search_item_by_ident("opal").unwrap().actor, "Item_Ore_E");
    assert_eq!(search_item_by_ident("diamond").unwrap().actor, "Item_Ore_A");
    assert_eq!(search_item_by_ident("tail").unwrap().actor, "Item_Enemy_05");
    assert_eq!(search_item_by_ident("spring").unwrap().actor, "Item_Enemy_28");
    assert_eq!(search_item_by_ident("shaft").unwrap().actor, "Item_Enemy_29");
    assert_eq!(search_item_by_ident("core").unwrap().actor, "Item_Enemy_30");
    assert_eq!(search_item_by_ident("wood").unwrap().actor, "Obj_FireWoodBundle");
    assert_eq!(search_item_by_ident("rushroom").unwrap().actor, "Item_MushroomGet_D");
    assert_eq!(search_item_by_ident("screw").unwrap().actor, "Item_Enemy_27");
    assert_eq!(search_item_by_ident("hyrulebass").unwrap().actor, "Item_FishGet_A");
    assert_eq!(search_item_by_ident("lizalfoshorn").unwrap().actor, "Item_Enemy_03");
    assert_eq!(search_item_by_ident("lizalfostalon").unwrap().actor, "Item_Enemy_04");

    assert_eq!(search_item_by_ident("normalarrow").unwrap().actor, "NormalArrow");
    assert_eq!(search_item_by_ident("firearrow").unwrap().actor, "FireArrow");
    assert_eq!(search_item_by_ident("icearrow").unwrap().actor, "IceArrow");
    assert_eq!(search_item_by_ident("shockarrow").unwrap().actor, "ElectricArrow");
    assert_eq!(search_item_by_ident("bombarrow").unwrap().actor, "BombArrow_A");
    assert_eq!(search_item_by_ident("ancientarrow").unwrap().actor, "AncientArrow");

    assert_eq!(search_item_by_ident("apple").unwrap().actor, "Item_Fruit_A");
    assert_eq!(search_item_by_ident("hylianshroom").unwrap().actor, "Item_Mushroom_E");
    assert_eq!(search_item_by_ident("spicypepper").unwrap().actor, "Item_Fruit_I");
    assert_eq!(search_item_by_ident("endurashroom").unwrap().actor, "Item_Mushroom_O");
    assert_eq!(search_item_by_ident("heartyradish").unwrap().actor, "Item_PlantGet_B");
    assert_eq!(search_item_by_ident("bigheartyradish").unwrap().actor, "Item_PlantGet_C");
    assert_eq!(search_item_by_ident("fairy").unwrap().actor, "Animal_Insect_F");
    assert_eq!(search_item_by_ident("masterSword").unwrap().actor, "Weapon_Sword_070");
    assert_eq!(search_item_by_ident("zoraarmor").unwrap().actor, "Armor_006_Upper");
    
    // Legacy food. Need to add metadata
    assert_eq!(search_item_by_ident("speedfood"), Some(ResolvedItem {
        actor: "Item_Cook_A_03".to_string(),
        meta: Some(cir::ItemMeta {
            effect_id: Some(13),
            ..Default::default()
        })
    }));

    assert_eq!(search_item_by_ident("endurafood"), Some(ResolvedItem {
        actor: "Item_Cook_A_01".to_string(),
        meta: Some(cir::ItemMeta {
            effect_id: Some(15),
            ..Default::default()
        })
    }));
}

