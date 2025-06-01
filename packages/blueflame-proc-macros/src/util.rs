use proc_macro::TokenStream;
use quote::quote;

pub type Span2 = proc_macro2::Span;

pub const CRATE: &str = "blueflame";

pub fn crate_ident() -> proc_macro2::TokenStream {
    // this won't work if user rename blueflame crate - a case we don't support right now
    let ident = syn::Ident::new(CRATE, Span2::call_site());
    quote! { #ident }
}

pub fn unwrap_result(result: syn::Result<TokenStream>) -> TokenStream {
    match result {
        Ok(x) => x,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Get the #[size] attribute on a struct
pub fn get_struct_size(input: &syn::DeriveInput) -> syn::Result<u32> {
    let Some(size_attr) = input.attrs.iter().find(|attr| attr.path().is_ident("size")) else {
        syn_error!(input, "Missing #[size] attribute for MemObject derive");
    };
    let (size, lit) = parse_u32_attribute("size", size_attr)?;

    if size == 0 {
        syn_error!(lit, "0 is not a valid size for C structs");
    }

    Ok(size)
}

/// Get the #[size] attribute on a field
pub fn get_field_size(input: &syn::Field) -> syn::Result<Option<(u32, syn::LitInt)>> {
    let Some(size_attr) = input.attrs.iter().find(|attr| attr.path().is_ident("size")) else {
        return Ok(None);
    };
    let (size, lit) = parse_u32_attribute("size", size_attr)?;

    if size == 0 {
        syn_error!(lit, "0 is not a valid size for C structs");
    }

    Ok(Some((size, lit)))
}

pub fn get_struct_fields(input: &syn::DeriveInput) -> syn::Result<&syn::FieldsNamed> {
    let syn::Data::Struct(data) = &input.data else {
        syn_error!(input, "MemObject can only be derived for structs");
    };

    let syn::Fields::Named(fields) = &data.fields else {
        syn_error!(
            &data.fields,
            "MemObject can only be derived for structs with named fields"
        );
    };

    Ok(fields)
}

pub fn get_field_offset(input: &syn::Field) -> syn::Result<u32> {
    let Some(attr) = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("offset"))
    else {
        syn_error!(
            input,
            "Missing #[offset] attribute for field in MemObject derive"
        );
    };

    let (offset, _) = parse_u32_attribute("offset", attr)?;
    Ok(offset)
}

pub fn parse_u32_attribute(id: &str, attr: &syn::Attribute) -> syn::Result<(u32, syn::LitInt)> {
    let Ok(meta_list) = attr.meta.require_list() else {
        syn_error!(
            attr,
            "Attribute #[{}(...)] should contain a single u32 literal",
            id
        );
    };

    let Ok(syn::Lit::Int(lit)) = meta_list.parse_args::<syn::Lit>() else {
        syn_error!(
            meta_list,
            "Attribute #[{}(...)] should contain a valid u32 literal",
            id
        );
    };

    let Ok(n) = lit.base10_parse::<u32>() else {
        syn_error!(
            lit,
            "Attribute #[{}(...)] should contain a valid u32 literal",
            id
        );
    };

    Ok((n, lit))
}

/// Macro for creating and returning `syn::Error`
macro_rules! syn_error {
    ($tokens:expr, $msg:expr) => {
        return Err(syn::Error::new_spanned($tokens, $msg))
    };
    ($tokens:expr, $($tt:tt)*) => {
        return Err(syn::Error::new_spanned($tokens, format!($($tt)*)))
    };
}
pub(crate) use syn_error;
