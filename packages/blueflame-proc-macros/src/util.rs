use pm::pre::*;

pub const CRATE: &str = "blueflame";

pub fn crate_ident() -> TokenStream2 {
    // this won't work if user rename blueflame crate - a case we don't support right now
    let ident = syn::Ident::new(CRATE, Span2::call_site());
    pm::quote! { #ident }
}

/// Get the #[size] attribute on a struct
pub fn get_struct_size(input: &syn::DeriveInput) -> pm::Result<u32> {
    let Some(size_attr) = input.attrs.iter().find(|attr| attr.path().is_ident("size")) else {
        pm::bail!(input, "Missing #[size] attribute for MemObject derive");
    };
    let (size, lit) = parse_u32_attribute("size", size_attr)?;

    if size == 0 {
        pm::bail!(lit, "0 is not a valid size for C structs");
    }

    Ok(size)
}

/// Get the #[size] attribute on a field
pub fn get_field_size(input: &syn::Field) -> pm::Result<Option<(u32, syn::LitInt)>> {
    let Some(size_attr) = input.attrs.iter().find(|attr| attr.path().is_ident("size")) else {
        return Ok(None);
    };
    let (size, lit) = parse_u32_attribute("size", size_attr)?;

    if size == 0 {
        pm::bail!(lit, "0 is not a valid size for C structs");
    }

    Ok(Some((size, lit)))
}

pub fn get_struct_fields(input: &syn::DeriveInput) -> pm::Result<&syn::FieldsNamed> {
    let syn::Data::Struct(data) = &input.data else {
        pm::bail!(input, "MemObject can only be derived for structs");
    };

    let syn::Fields::Named(fields) = &data.fields else {
        pm::bail!(
            &data.fields,
            "MemObject can only be derived for structs with named fields"
        );
    };

    Ok(fields)
}

pub fn get_field_offset(input: &syn::Field) -> pm::Result<u32> {
    let Some(attr) = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("offset"))
    else {
        pm::bail!(
            input,
            "Missing #[offset] attribute for field in MemObject derive"
        );
    };

    let (offset, _) = parse_u32_attribute("offset", attr)?;
    Ok(offset)
}

pub fn parse_u32_attribute(id: &str, attr: &syn::Attribute) -> pm::Result<(u32, syn::LitInt)> {
    let Ok(meta_list) = attr.meta.require_list() else {
        pm::bail!(
            attr,
            "Attribute #[{}(...)] should contain a single u32 literal",
            id
        );
    };

    let Ok(syn::Lit::Int(lit)) = meta_list.parse_args::<syn::Lit>() else {
        pm::bail!(
            meta_list,
            "Attribute #[{}(...)] should contain a valid u32 literal",
            id
        );
    };

    let Ok(n) = lit.base10_parse::<u32>() else {
        pm::bail!(
            lit,
            "Attribute #[{}(...)] should contain a valid u32 literal",
            id
        );
    };

    Ok((n, lit))
}
