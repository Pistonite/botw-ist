use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, Lit};

#[proc_macro_derive(MemRead, attributes(offset, size))]
pub fn derive_mem_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let size_attr = input.attrs.iter().find(|attr| attr.path().is_ident("size"));
    let size_value = size_attr.and_then(|attr| {
        let meta_list = attr.meta.require_list().ok()?;
        let lit = meta_list.parse_args::<Lit>().ok()?;
        if let Lit::Int(v) = lit {
            v.base10_parse::<u32>().ok()
        } else {
            None
        }
    });

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        Data::Enum(DataEnum { variants, .. }) => {
            let variant_readers = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let result = variant.discriminant.as_ref().ok_or_else(|| {
                    quote! { return Err(Error::Mem(crate::memory::Error::Unexpected("Missing discriminant for enum variant".to_string()))) }
                });

                match result {
                    Ok(discriminant) => match &discriminant.1 {
                        syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(value), .. }) => {
                            let value = value.base10_parse::<i32>().unwrap_or(-1);
                            quote! { #value => Ok(#struct_name::#variant_name), }
                        }
                        syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Neg(_), expr, .. }) => {
                            if let syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(value), .. }) = &**expr {
                                let value = -value.base10_parse::<i32>().unwrap_or(1);
                                quote! { #value => Ok(#struct_name::#variant_name), }
                            } else {
                                quote! { _ => Err(Error::Mem(crate::memory::Error::Unexpected("Unsupported negative enum discriminant".to_string()))), }
                            }
                        }
                        _ => quote! { _ => Err(Error::Mem(crate::memory::Error::Unexpected("Enum discriminant must be an integer".to_string()))), },
                    },
                    Err(q) => q,
                }
            });

            let expanded = quote! {
                impl MemRead for #struct_name {
                    fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
                        let value = reader.read_i32()?;
                        match value {
                            #(#variant_readers)*
                            _ => Err(Error::Mem(crate::memory::Error::Unexpected("Bad Enum variant".to_string()))),
                        }
                    }

                    fn read_total_offset() -> u32 {
                        <i32>::read_total_offset()
                    }
                }
            };

            return TokenStream::from(expanded);
        }
        _ => {
            return TokenStream::from(
                quote! { compile_error!("MemRead can only be used on structs with named fields"); },
            )
        }
    };

    let mut field_accessors = Vec::new();
    let mut field_names = Vec::new();
    for field in fields.iter() {
        let field_name = match &field.ident {
            Some(name) => name,
            None => return quote!(compile_error!("All fields must be named")).into(),
        };
        let field_type = &field.ty;

        let mut offset = None;
        for attr in &field.attrs {
            if attr.path().is_ident("offset") {
                if let Ok(meta_list) = attr.meta.require_list() {
                    if let Ok(lit) = meta_list.parse_args::<Lit>() {
                        if let Lit::Int(v) = lit {
                            offset = Some(v.base10_parse::<u32>().unwrap_or(0));
                        }
                    }
                }
            }
        }

        let offset = match offset {
            Some(o) => o,
            None => return quote!(compile_error!("Missing offset attribute")).into(),
        };

        field_accessors.push(quote! {
            reader.skip(#offset - cur_offset);
            cur_offset = #offset;
            let #field_name = <#field_type>::read_from_mem(reader)?;
            cur_offset += <#field_type>::read_total_offset();
        });
        field_names.push(quote! { #field_name, });
    }

    let last_field = match fields.last() {
        Some(f) => f,
        None => return TokenStream::from(quote! { compile_error!("No fields in struct"); }),
    };

    let last_field_type = &last_field.ty;
    let mut last_offset = None;
    for attr in &last_field.attrs {
        if attr.path().is_ident("offset") {
            if let Ok(meta_list) = attr.meta.require_list() {
                if let Ok(lit) = meta_list.parse_args::<Lit>() {
                    if let Lit::Int(v) = lit {
                        last_offset = Some(v.base10_parse::<u32>().unwrap_or(0));
                    }
                }
            }
        }
    }

    let last_offset = last_offset.unwrap_or(0);

    let read_total_offset = match size_value {
        Some(size) => quote! { #size },
        None => quote! { #last_offset + <#last_field_type>::read_total_offset() },
    };

    let expanded = quote! {
        impl MemRead for #struct_name {
            fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
                let mut cur_offset: u32 = 0;
                #(#field_accessors)*
                reader.skip(#read_total_offset - cur_offset);
                Ok(#struct_name {
                    #(#field_names)*
                })
            }

            fn read_total_offset() -> u32 {
                #read_total_offset
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(MemWrite, attributes(offset, size))]
pub fn derive_mem_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let size_attr = input.attrs.iter().find(|attr| attr.path().is_ident("size"));
    let size_value = size_attr.and_then(|attr| {
        let meta_list = attr.meta.require_list().ok()?;
        let lit = meta_list.parse_args::<Lit>().ok()?;
        if let Lit::Int(v) = lit {
            v.base10_parse::<u32>().ok()
        } else {
            None
        }
    });

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        Data::Enum(DataEnum { variants, .. }) => {
            let _variant_writers = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let result = variant.discriminant.as_ref().ok_or_else(|| {
                    quote! { return Err(Error::Mem(crate::memory::Error::Unexpected("Missing discriminant for enum variant".to_string()))) }
                });

                match result {
                    Ok(discriminant) => match &discriminant.1 {
                        syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(value), .. }) => {
                            let value = value.base10_parse::<i32>().unwrap_or(-1);
                            quote! { #value => Ok(#struct_name::#variant_name), }
                        }
                        syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Neg(_), expr, .. }) => {
                            if let syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(value), .. }) = &**expr {
                                let value = -value.base10_parse::<i32>().unwrap_or(1);
                                quote! { #value => Ok(#struct_name::#variant_name), }
                            } else {
                                quote! { _ => Err(Error::Mem(crate::memory::Error::Unexpected("Unsupported negative enum discriminant".to_string()))), }
                            }
                        }
                        _ => quote! { _ => Err(Error::Mem(crate::memory::Error::Unexpected("Enum discriminant must be an integer".to_string()))), },
                    },
                    Err(q) => q,
                }
            });

            let expanded = quote! {
                impl MemWrite for #struct_name {
                    fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
                        writer.write_i32(self as i32)?;
                        Ok(())
                    }

                    fn write_total_offset(&self) -> u32 {
                        let t = 0i32;
                        t.write_total_offset()
                    }
                }
            };

            return TokenStream::from(expanded);
        }
        _ => {
            return TokenStream::from(
                quote! { compile_error!("MemWrite can only be used on structs with named fields"); },
            )
        }
    };

    let mut field_writers = Vec::new();
    for field in fields.iter() {
        let field_name = match &field.ident {
            Some(name) => name,
            None => return quote!(compile_error!("All fields must be named")).into(),
        };

        let mut offset = None;
        for attr in &field.attrs {
            if attr.path().is_ident("offset") {
                if let Ok(meta_list) = attr.meta.require_list() {
                    if let Ok(lit) = meta_list.parse_args::<Lit>() {
                        if let Lit::Int(v) = lit {
                            offset = Some(v.base10_parse::<u32>().unwrap_or(0));
                        }
                    }
                }
            }
        }

        let offset = match offset {
            Some(o) => o,
            None => return quote!(compile_error!("Missing offset attribute")).into(),
        };

        field_writers.push(quote! {
            writer.skip(#offset - cur_offset);
            cur_offset = #offset;
            let write_offset = self.#field_name.write_total_offset();
            let #field_name = self.#field_name.write_to_mem(writer)?;
            cur_offset += write_offset;
        });
    }

    let last_field = match fields.last() {
        Some(f) => f,
        None => return TokenStream::from(quote! { compile_error!("No fields in struct"); }),
    };

    let last_field_name = last_field.ident.as_ref().expect("Missing ident");
    let mut last_offset = None;
    for attr in &last_field.attrs {
        if attr.path().is_ident("offset") {
            if let Ok(meta_list) = attr.meta.require_list() {
                if let Ok(lit) = meta_list.parse_args::<Lit>() {
                    if let Lit::Int(v) = lit {
                        last_offset = Some(v.base10_parse::<u32>().unwrap_or(0));
                    }
                }
            }
        }
    }

    let last_offset = last_offset.unwrap_or(0);

    let write_total_offset = match size_value {
        Some(size) => quote! { #size },
        None => quote! { #last_offset + self.#last_field_name.write_total_offset() },
    };

    let writer_skip = match size_value {
        Some(size) => quote! { writer.skip(#size - cur_offset); },
        None => quote! {},
    };

    let expanded = quote! {
        impl MemWrite for #struct_name {
            fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
                let mut cur_offset: u32 = 0;
                #(#field_writers)*
                #writer_skip
                Ok(())
            }

            fn write_total_offset(&self) -> u32 {
                #write_total_offset
            }
        }
    };

    TokenStream::from(expanded)
}
