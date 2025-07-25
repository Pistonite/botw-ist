use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned as _};

use crate::util::{self, syn_error};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    util::unwrap_result(expand_internal(input))
}

fn expand_internal(mut input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let blueflame = util::crate_ident();
    let name = input.ident.clone();

    let mut default_features_impl = TokenStream2::new();
    let mut feature_map_impl = TokenStream2::new();
    {
        let syn::Data::Enum(input) = &mut input.data else {
            syn_error!(input, "can only be derived for enums");
        };

        for v in input.variants.iter_mut() {
            let ident = &v.ident;
            let ident_kebab_str = ident.to_string().replace("_", "-");
            if v.attrs.iter().any(|a| a.path().is_ident("on")) {
                default_features_impl.extend(quote! {
                    #name::#ident |
                });
                v.attrs.retain(|a| !a.path().is_ident("on"));
            }
            feature_map_impl.extend(quote! {
                #ident_kebab_str => #name::#ident,
            });
        }
    }

    let expanded = quote! {
        #input
        const _: () = {
            #[automatically_derived]
            impl #name {
                /// Get the default set of features for this enum, See the enum doc for more
                pub const fn default_const() -> #blueflame::env::FeatureSet {
                    use enumset::EnumSet;
                    enumset::enum_set!( #default_features_impl )
                }

                /// Parse the kebab-case string feature name to the feature enum
                pub fn parse(input: &str) -> ::std::option::Option<blueflame::env::Feature> {
                    FEATURE_MAP.get(input).copied()
                }
            }

            static FEATURE_MAP: phf::Map<&'static str, #name> = phf::phf_map! {
                #feature_map_impl
            };
        };
    };

    Ok(expanded.into())
}

pub fn expand_enable_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);

    let feature_name = input.value();
    let feature_ident = syn::Ident::new(&feature_name.replace("-", "_"), feature_name.span());

    let blueflame = util::crate_ident();
    let expanded = quote! {
        #blueflame::env::is_feature_enabled(#blueflame::env::Feature::#feature_ident)
    };

    expanded.into()
}
