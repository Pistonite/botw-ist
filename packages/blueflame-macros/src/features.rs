use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, spanned::Spanned as _};
use quote::quote;

use crate::util::{self, syn_error};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    util::unwrap_result(expand_internal(input))
}

fn expand_internal(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let blueflame = util::crate_ident();
    let name = &input.ident;

    let syn::Data::Enum(input) = input.data else {
        syn_error!(
            input,
            "DefaultFeatures can only be derived for enums"
        );
    };

    let mut default_features_impl = TokenStream2::new();
    for v in input.variants.iter() {
        if v.attrs.iter().find(|a| a.path().is_ident("on")).is_none() {
            continue;
        }
        let ident = &v.ident;
        default_features_impl.extend(quote! {
            #name::#ident |
        });
    }

    let expanded = quote! {
        impl #name {
            pub const fn default_const() -> #blueflame::features::FeatureSet {
                use #blueflame::features::enumset::EnumSet;
                #blueflame::features::enumset::enum_set!( #default_features_impl )
            }
        }
    };

    Ok(expanded.into())
}

pub fn expand_enable_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);

    let feature_name = input.value();
    let feature_ident = syn::Ident::new(&feature_name.replace("-", "_")
        , feature_name.span());


    let blueflame = util::crate_ident();
    let expanded = quote! {
        #blueflame::features::is_enabled(#blueflame::features::Feature::#feature_ident)
    };

    expanded.into()
}
