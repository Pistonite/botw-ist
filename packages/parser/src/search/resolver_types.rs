use std::future::Future;

use crate::cir;

/// A trait for resolving quoted (localized) items
///
/// The parser itself does not contain the localization data
/// for resolving this, so the simulator runtime must provide
/// an implementation that is connected to the localization data.
pub trait QuotedItemResolver {
    type Future: Future<Output = Option<ResolvedItem>>;
    /// Resolve a quote item word "like this" to its actor name
    fn resolve_quoted(&self, word: &str) -> Self::Future;
}

/// The result returned by item searcher
#[derive(Debug, PartialEq)]
pub struct ResolvedItem {
    /// The actor found
    pub actor: String,
    /// The meta data of the item, if any
    pub meta: Option<cir::ItemMeta>,
}
