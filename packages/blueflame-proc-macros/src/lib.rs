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

#[proc_macro_attribute]
pub fn no_panic(_attr: TokenStream, input: TokenStream) -> TokenStream {
    no_panic::expand(input)
}
mod no_panic;

/// Macro to convert a paste-in instruction bytes to u32 literal
#[proc_macro]
pub fn paste_insn(input: TokenStream) -> TokenStream {
    paste_insn::expand(input)
}
mod paste_insn;
