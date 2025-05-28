pub fn get_pouch_item_use(actor: &str) -> i32 {
    crate::generated::actor::ACTOR_USE_MAP
        .get(actor)
        .copied()
        .unwrap_or(8) // default is Item
}
pub fn get_pouch_item_type(actor: &str) -> i32 {
    crate::generated::actor::ACTOR_TYPE_MAP
        .get(actor)
        .copied()
        .unwrap_or(7) // default is Material
}
