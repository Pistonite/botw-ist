use std::future::Future;
use std::pin::Pin;

use js_sys::{Function, Promise, Reflect};
use skybook_parser::search::{QuotedItemResolver, ResolvedItem};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// QuotedItemResolver implementation that delegates to JS
#[derive(Clone)]
pub struct JsQuotedItemResolver {
    delegate: Function,
}

impl JsQuotedItemResolver {
    /// Create a new resolver that throws an error when called
    pub fn new(delegate: Function) -> Self {
        Self {
            delegate
        }
    }

    pub async fn call(self, word: String) -> Option<ResolvedItem> {
        match self.call_internal(&word).await {
            Ok(Some(item)) => Some(item),
            _ => None,
        }
    }
    pub async fn call_internal(&self, word: &str) -> Result<Option<ResolvedItem>, JsValue> {
        let search_result = self.delegate.call1(&JsValue::NULL, &JsValue::from_str(word))?;
        // delegate must return a promise
        let promise = search_result.dyn_into::<Promise>()?;
        let result = JsFuture::from(promise).await?;
        if result.is_null() || result.is_undefined() {
            return Ok(None);
        }
        // actor must be a string
        let actor = match Reflect::get(&result, &JsValue::from_str("actor"))?.as_string() {
            Some(actor) => actor,
            None => return Ok(None)
        };
        let effect_id = Reflect::get(&result, &JsValue::from_str("cookEffect"))?.as_f64().unwrap_or_default();
        Ok(Some(ResolvedItem::with_effect_id(actor, effect_id as i32)))
    }
}

impl QuotedItemResolver for JsQuotedItemResolver {
    type Future = Pin<Box<dyn Future<Output = Option<ResolvedItem>>>>;

    fn resolve_quoted(&self, word: &str) -> Self::Future {
        Box::pin(self.clone().call(word.to_string()))
    }
}

