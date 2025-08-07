use skybook_parser::search::{search_item_by_ident, search_item_by_ident_all};

pub fn test_item_search(input: &str, output: &str) {
    assert_eq!(search_item_by_ident(input).unwrap().actor, output);
    assert_eq!(search_item_by_ident_all(input)[0].actor, output);
}

#[allow(unused)]
pub fn test_item_search_effect(input: &str, output: &str, effect: i32) {
    let result = search_item_by_ident(input).unwrap();
    assert_eq!(result.actor, output);
    assert_eq!(result.meta.unwrap().effect_id.unwrap(), effect);
    let result = search_item_by_ident_all(input).into_iter().next().unwrap();
    assert_eq!(result.actor, output);
    assert_eq!(result.meta.unwrap().effect_id.unwrap(), effect);
}
