use std::future::Future;

use crate::cir;

/// A trait for resolving quoted (localized) items
///
/// The parser itself does not contain the localization data
/// for resolving this, so the simulator runtime must provide
/// an implementation that is connected to the localization data.
pub trait QuotedItemResolver {
    type Future: Future<Output = Option<ResolvedItem>>;

    /// Resolve a quote item word "like this" to its actor name.
    /// The input does not contain the quotes.
    fn resolve_quoted(&self, word: &str) -> Self::Future;
}

impl<T: Fn(&str) -> F, F: Future<Output = Option<ResolvedItem>>> QuotedItemResolver for T {
    type Future = F;

    fn resolve_quoted(&self, word: &str) -> Self::Future {
        self(word)
    }
}

/// The result returned by item searcher
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedItem {
    /// The actor found
    pub actor: String,
    /// The meta data of the item, if any
    pub meta: Option<cir::ItemMeta>,
}

impl ResolvedItem {
    /// Create a new resolved item without meta
    pub fn new(actor: String) -> Self {
        Self { actor, meta: None }
    }

    /// Create a new resolved item with meta
    pub fn with_meta(actor: String, meta: cir::ItemMeta) -> Self {
        Self {
            actor,
            meta: Some(meta),
        }
    }

    pub fn with_effect_id(actor: String, effect_id: i32) -> Self {
        let effect_id = match effect_id {
            0 => None,
            _ => Some(effect_id),
        };
        Self {
            actor,
            meta: Some(cir::ItemMeta {
                effect_id,
                ..Default::default()
            }),
        }
    }
}
