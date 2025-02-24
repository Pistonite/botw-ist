#[cfg(not(feature = "mock-data"))]
mod item_search_common;
#[cfg(not(feature = "mock-data"))]
use item_search_common::test_item_search;
#[cfg(not(feature = "mock-data"))]
use skybook_parser::cir;
#[cfg(not(feature = "mock-data"))]
use skybook_parser::search::{
    search_item_by_ident, search_item_by_ident_all, ResolvedItem, COOK_EFFECT_NAMES,
};

/// Test items with priority in V3
///
/// For example Axe -> WoodcuttersAxe
#[cfg(not(feature = "mock-data"))]
#[test]
fn test_item_search_v3_priority() {
    test_item_search("branch", "Weapon_Sword_044");
    test_item_search("torch", "Weapon_Sword_043");
    test_item_search("soup", "Weapon_Sword_022");
    test_item_search("boomerang", "Weapon_Sword_009");
    test_item_search("axe", "Weapon_Lsword_032");
    test_item_search("trident", "Weapon_Spear_050");
    test_item_search("spear", "Weapon_Spear_004");
    test_item_search("arrow", "NormalArrow");
    test_item_search("lid", "Weapon_Shield_040");
    test_item_search("shroom", "Item_Mushroom_E");
    test_item_search("beetle", "Animal_Insect_AA");
}

/// Test items with plural forms
#[cfg(not(feature = "mock-data"))]
#[test]
fn test_item_search_v3_plural() {
    test_item_search("apples", "Item_Fruit_A");
}

/// Test items with alias in V3
///
/// For example GEB -> GreatEagleBow
#[cfg(not(feature = "mock-data"))]
#[test]
fn test_item_search_v3_alias() {
    test_item_search("trav_sword", "Weapon_Sword_001");
    test_item_search("soldier_sword", "Weapon_Sword_002");
    test_item_search("knightsword", "Weapon_Sword_003");
    test_item_search("royal_sword", "Weapon_Sword_024");
    test_item_search("fdsword", "Weapon_Sword_025");
    test_item_search("rgsword", "Weapon_Sword_047");
    test_item_search("oho", "Weapon_Sword_502");

    test_item_search("trav_claymore", "Weapon_Lsword_001");
    test_item_search("soldier_claymore", "Weapon_Lsword_002");
    test_item_search("knight_claymore", "Weapon_Lsword_003");
    test_item_search("royal_claymore", "Weapon_Lsword_024");
    test_item_search("rgc", "Weapon_Lsword_047");
    test_item_search("eod", "Weapon_Lsword_055");

    test_item_search("fdspear", "Weapon_Spear_025");
    test_item_search("rgspear", "Weapon_Spear_047");

    test_item_search("trav_bow", "Weapon_Bow_001");
    test_item_search("soldier_bow", "Weapon_Bow_002");
    test_item_search("knight_bow", "Weapon_Bow_035");
    test_item_search("royal_bow", "Weapon_Bow_036");

    test_item_search("fdb", "Weapon_Bow_013");
    test_item_search("rgb", "Weapon_Bow_033");
    test_item_search("geb", "Weapon_Bow_028");

    test_item_search("aa", "AncientArrow");

    test_item_search("royal_shield", "Weapon_Shield_022");
    test_item_search("rgshield", "Weapon_Shield_033");
    test_item_search("fdshield", "Weapon_Shield_023");

    test_item_search("poop", "Obj_ProofKorok");
    test_item_search("thunderhelmkey", "Obj_Armor_115_Head");
}

#[cfg(not(feature = "mock-data"))]
#[test]
fn test_item_search_v3_effect() {
    for (effect_name, effect_id) in COOK_EFFECT_NAMES {
        assert_eq!(
            search_item_by_ident(&format!("{}_elixir", effect_name)),
            Some(ResolvedItem {
                actor: "Item_Cook_C_17".to_string(),
                meta: Some(cir::ItemMeta {
                    effect_id: Some(*effect_id),
                    ..Default::default()
                })
            })
        );
        assert_eq!(
            search_item_by_ident_all(&format!("{}_elixir", effect_name))[0],
            ResolvedItem {
                actor: "Item_Cook_C_17".to_string(),
                meta: Some(cir::ItemMeta {
                    effect_id: Some(*effect_id),
                    ..Default::default()
                })
            }
        );
    }
}
