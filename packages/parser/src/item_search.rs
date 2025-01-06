
pub trait ItemResolver {
    type Future<T>: std::future::Future<Output = T> + Send + 'static;

    /// Resolve an item to its actor name
    fn resolve(&self, word: &str) -> Self::Future<Option<String>>;

    /// Resolve a quote item word "like this" to its actor name
    fn resolve_quoted(&self, word: &str) -> Self::Future<Option<String>>;


}
