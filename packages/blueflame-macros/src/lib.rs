use proc_macro::TokenStream;

mod util;

/// Derive macro for MemObject
#[proc_macro_derive(MemObject, attributes(offset, size))]
pub fn derive_mem_object(input: TokenStream) -> TokenStream {
    mem_object::expand(input)
}
mod mem_object;

#[proc_macro_derive(FeatureFlags, attributes(on))]
pub fn derive_feature_set(input: TokenStream) -> TokenStream {
    features::expand(input)
}
mod features;

/// Macro to check if a feature is enabled
#[proc_macro]
pub fn enabled(input: TokenStream) -> TokenStream {
    features::expand_enable_macro(input)
}

