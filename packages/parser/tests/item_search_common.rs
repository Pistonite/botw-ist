use skybook_parser::search::{search_item_by_ident, search_item_by_ident_all};

pub fn test_item_search(input: &str, output: &str) {
    assert_eq!(search_item_by_ident(&input).unwrap().actor, output);
    assert_eq!(search_item_by_ident_all(&input)[0].actor, output);
}
