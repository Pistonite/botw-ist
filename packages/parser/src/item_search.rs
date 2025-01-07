use crate::cir;


pub trait ItemResolver {

    /// Resolve an item to its actor name
    async fn resolve(&self, word: &str) -> ResolvedItem;

    /// Resolve a quote item word "like this" to its actor name
    async fn resolve_quoted(&self, word: &str) -> ResolvedItem;


}

#[derive(Debug, PartialEq)]
pub struct ResolvedItem {
    /// The actor found
    pub actor: String,
    /// The meta data of the item, if any
    pub meta: Option<cir::ItemMeta>,
}
