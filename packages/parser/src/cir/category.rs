use crate::syn;

pub enum Category {
    Weapon,
    Bow,
    Shield,
    Armor,
    Material,
    Food,
    KeyItem,
}

pub fn parse_category(category: &syn::Category) -> Category {
    match category {
        syn::Category::Weapon(_) => Category::Weapon,
        syn::Category::Bow(_) => Category::Bow,
        syn::Category::Shield(_) => Category::Shield,
        syn::Category::Armor(_) => Category::Armor,
        syn::Category::Material(_) => Category::Material,
        syn::Category::Food(_) => Category::Food,
        syn::Category::KeyItem(_) => Category::KeyItem,
    }
}

pub struct CategorySpec {
    /// Which category to select
    pub category: Category,
    /// The meaning depends on the context
    ///
    /// For example, this is the page to entangle
    pub amount: i64,
    /// Which row to select (1 indexed, 0 means unspecified)
    pub row: i8,
    /// Which column to select (1 indexed, 0 means unspecified)
    pub col: i8,
}


