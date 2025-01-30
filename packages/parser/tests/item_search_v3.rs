use skybook_parser::search::{ResolvedItem, search_item_by_ident, COOK_EFFECT_NAMES};
use skybook_parser::cir;

/// Test items with priority in V3
///
/// For example Axe -> WoodcuttersAxe
#[test]
fn test_item_search_v3_priority() {
    assert_eq!(search_item_by_ident("branch").unwrap().actor, "Weapon_Sword_044");
    assert_eq!(search_item_by_ident("torch").unwrap().actor, "Weapon_Sword_043");
    assert_eq!(search_item_by_ident("soup").unwrap().actor, "Weapon_Sword_022");
    assert_eq!(search_item_by_ident("boomerang").unwrap().actor, "Weapon_Sword_009");
    assert_eq!(search_item_by_ident("axe").unwrap().actor, "Weapon_Lsword_032");
    assert_eq!(search_item_by_ident("trident").unwrap().actor, "Weapon_Spear_050");
    assert_eq!(search_item_by_ident("spear").unwrap().actor, "Weapon_Spear_004");
    assert_eq!(search_item_by_ident("arrow").unwrap().actor, "NormalArrow");
    assert_eq!(search_item_by_ident("lid").unwrap().actor, "Weapon_Shield_040");
    assert_eq!(search_item_by_ident("shroom").unwrap().actor, "Item_Mushroom_E");
    assert_eq!(search_item_by_ident("beetle").unwrap().actor, "Animal_Insect_AA");
}

/// Test items with alias in V3
///
/// For example GEB -> GreatEagleBow
#[test]
fn test_item_search_v3_alias() {
    assert_eq!(search_item_by_ident("trav_sword").unwrap().actor, "Weapon_Sword_001");
    assert_eq!(search_item_by_ident("soldier_sword").unwrap().actor, "Weapon_Sword_002");
    assert_eq!(search_item_by_ident("knightsword").unwrap().actor, "Weapon_Sword_003");
    assert_eq!(search_item_by_ident("royal_sword").unwrap().actor, "Weapon_Sword_024");
    assert_eq!(search_item_by_ident("fdsword").unwrap().actor, "Weapon_Sword_025");
    assert_eq!(search_item_by_ident("rgsword").unwrap().actor, "Weapon_Sword_047");
    assert_eq!(search_item_by_ident("oho").unwrap().actor, "Weapon_Sword_502");

    assert_eq!(search_item_by_ident("trav_claymore").unwrap().actor, "Weapon_Lsword_001");
    assert_eq!(search_item_by_ident("soldier_claymore").unwrap().actor, "Weapon_Lsword_002");
    assert_eq!(search_item_by_ident("knight_claymore").unwrap().actor, "Weapon_Lsword_003");
    assert_eq!(search_item_by_ident("royal_claymore").unwrap().actor, "Weapon_Lsword_024");
    assert_eq!(search_item_by_ident("rgc").unwrap().actor, "Weapon_Lsword_047");
    assert_eq!(search_item_by_ident("eod").unwrap().actor, "Weapon_Lsword_055");

    assert_eq!(search_item_by_ident("fdspear").unwrap().actor, "Weapon_Spear_025");
    assert_eq!(search_item_by_ident("rgspear").unwrap().actor, "Weapon_Spear_047");

    assert_eq!(search_item_by_ident("trav_bow").unwrap().actor, "Weapon_Bow_001");
    assert_eq!(search_item_by_ident("soldier_bow").unwrap().actor, "Weapon_Bow_002");
    assert_eq!(search_item_by_ident("knight_bow").unwrap().actor, "Weapon_Bow_035");
    assert_eq!(search_item_by_ident("royal_bow").unwrap().actor, "Weapon_Bow_036");

    assert_eq!(search_item_by_ident("fdb").unwrap().actor, "Weapon_Bow_013");
    assert_eq!(search_item_by_ident("rgb").unwrap().actor, "Weapon_Bow_033");
    assert_eq!(search_item_by_ident("geb").unwrap().actor, "Weapon_Bow_028");

    assert_eq!(search_item_by_ident("aa").unwrap().actor, "AncientArrow");

    assert_eq!(search_item_by_ident("royal_shield").unwrap().actor, "Weapon_Shield_022");
    assert_eq!(search_item_by_ident("rgshield").unwrap().actor, "Weapon_Shield_033");
    assert_eq!(search_item_by_ident("fdshield").unwrap().actor, "Weapon_Shield_023");

    assert_eq!(search_item_by_ident("poop").unwrap().actor, "Obj_ProofKorok");
    assert_eq!(search_item_by_ident("thunderhelmkey").unwrap().actor, "Obj_Armor_115_Head");
}

#[test]
fn test_item_search_v3_effect() {
    for (effect_name, effect_id) in COOK_EFFECT_NAMES {
        assert_eq!(search_item_by_ident(&format!("{}_elixir", effect_name)), Some(ResolvedItem {
            actor: "Item_Cook_C_17".to_string(),
            meta: Some(cir::ItemMeta {
                effect_id: Some(*effect_id),
                ..Default::default()
            })
        }));
    }
}
