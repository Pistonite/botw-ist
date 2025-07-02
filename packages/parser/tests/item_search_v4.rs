mod item_search_common;
use item_search_common::test_item_search;
use skybook_parser::cir;
use skybook_parser::search::{
    COOK_EFFECT_NAMES, ResolvedItem, search_item_by_ident, search_item_by_ident_all,
};

/// Test items with alias that are added in V4
#[test]
fn test_item_search_v4_alias() {
    test_item_search("ice-cream", "Item_Cook_C_17");
}

#[test]
fn test_item_search_v4_effect() {
    for (effect_name, effect_id) in COOK_EFFECT_NAMES {
        assert_eq!(
            search_item_by_ident(&format!("{effect_name}-ice-cream")),
            Some(ResolvedItem {
                actor: "Item_Cook_C_17".to_string(),
                meta: Some(cir::ItemMeta {
                    effect_id: Some(*effect_id),
                    ..Default::default()
                })
            })
        );
        assert_eq!(
            search_item_by_ident_all(&format!("{effect_name}-ice-cream"))[0],
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
