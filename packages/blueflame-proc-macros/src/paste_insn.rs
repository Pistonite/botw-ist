use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;
use syn::spanned::Spanned as _;

pub fn expand(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input.clone());
    let mut result = 0u32;
    for (i, byte) in input.to_string().split_whitespace().enumerate() {
        if i > 3 {
            return syn::Error::new_spanned(input2, "too many bytes")
                .to_compile_error()
                .into();
        }
        let byte = byte.strip_prefix('x').unwrap_or(byte);
        if !byte.chars().all(|c| c.is_ascii_hexdigit()) {
            let error = format!("invalid byte: `{}`", byte);
            return syn::Error::new_spanned(input2, error)
                .to_compile_error()
                .into();
        }

        let part: u32 = match u8::from_str_radix(byte, 16) {
            Ok(val) => val as u32,
            Err(_) => {
                let error = format!("invalid byte value: `{}`", byte);
                return syn::Error::new_spanned(input2, error)
                    .to_compile_error()
                    .into();
            }
        };

        result = result | (part << (8 * i));
    }
    TokenStream::from(quote_spanned! { input2.span() => #result })
}
