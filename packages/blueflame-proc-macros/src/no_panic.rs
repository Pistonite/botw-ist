use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let expanded = quote! {
        #[cfg_attr(not(debug_assertions), blueflame::__re::no_panic::no_panic)]
        #input
    };

    expanded.into()
}
