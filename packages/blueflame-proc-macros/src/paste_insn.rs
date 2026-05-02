use pm::pre::*;
use syn::spanned::Spanned;

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
            let error = format!("invalid byte: `{byte}`");
            return syn::Error::new_spanned(input2, error)
                .to_compile_error()
                .into();
        }

        let part: u32 = match u8::from_str_radix(byte, 16) {
            Ok(val) => val as u32,
            Err(_) => {
                let error = format!("invalid byte value: `{byte}`");
                return syn::Error::new_spanned(input2, error)
                    .to_compile_error()
                    .into();
            }
        };

        result |= part << (8 * i);
    }
    TokenStream::from(pm::quote_spanned! { input2.span() => #result })
}
