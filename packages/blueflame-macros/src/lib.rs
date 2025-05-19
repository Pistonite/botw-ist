use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;
use syn::spanned::Spanned as _;

use proc_macro2::TokenStream as TokenStream2;

mod util;
use util::syn_error;

#[proc_macro_derive(MemObject, attributes(offset, size))]
pub fn derive_mem_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    util::unwrap_result(derive_mem_object_internal(input))
}

fn derive_mem_object_internal(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let blueflame = util::crate_ident();

    let size = util::get_struct_size(&input)?;
    let fields = util::get_struct_fields(&input)?;
    let struct_name = &input.ident;

    let mut fields_ordered_by_offset = Vec::with_capacity(fields.named.len());

    // let mut field_accessors = Vec::new();
    // let mut field_names = Vec::new();
    for field in fields.named.iter() {
        let offset = util::get_field_offset(field)?;
        let field_size = util::get_field_size(field)?;
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let (size_tokens, size_span) = match field_size {
            None => {
                let size_span = field_type.span();
                // use the size from the type
                let size_tokens = quote_spanned! {
                    size_span => {
                        <#field_type as #blueflame::memory::MemSized>::SIZE
                    }
                };
                (size_tokens, size_span)
            },
            Some((size, lit)) => {
                let size_span = lit.span();
                let size_tokens = quote_spanned! {
                    size_span => {
                        #size
                    }
                };
                (size_tokens, size_span)
            }
        };
        fields_ordered_by_offset.push(FieldData {
            offset,
            size_tokens,
            size_span,
            name: field_name,
            typ: field_type,
        });


        // field_accessors.push(quote! {
        //     reader.skip(#offset - cur_offset);
        //     cur_offset = #offset;
        //     let #field_name = <#field_type>::read_from_mem(reader)?;
        //     cur_offset += <#field_type as #blueflame::memory::MemObject>::SIZE;
        // });
        // field_names.push(quote! { #field_name, });
    }

    fields_ordered_by_offset.sort_by_key(|f| f.offset);
    let mut size_asserts = TokenStream2::new();
    let mut reader_impl = TokenStream2::new();
    let mut constructor_impl = TokenStream2::new();
    let mut writer_impl = TokenStream2::new();
    let mut layout_struct_impl = TokenStream2::new();
    let mut layout_new_impl = TokenStream2::new();

    let last_field_offset = match fields_ordered_by_offset.last() {
        Some(f) => f.offset,
        None => {
            syn_error!(struct_name, "Need at least one field in the struct");
        }
    };

    for (i, field_data) in fields_ordered_by_offset.iter().enumerate() {
        let curr_offset = field_data.offset;
        let next_offset = fields_ordered_by_offset.get(i + 1).map(|f| f.offset).unwrap_or(size);
        if next_offset <= curr_offset {
            syn_error!(field_data.size_tokens.clone(), "Fields cannot start at the same offset or overlap");
        }
        let max_size = next_offset - curr_offset;

        let size_tokens = &field_data.size_tokens;
        size_asserts.extend(quote_spanned! {
            field_data.size_span => {
                #blueflame::memory::macro_impl::assert_size_less_than!(#size_tokens, #max_size);
            }
        });

        let field_offset = curr_offset;
        let field_name = &field_data.name;
        let field_type = &field_data.typ;
        if i == fields_ordered_by_offset.len() - 1 {
            // special for last field, we want to obey the input size
            reader_impl.extend(quote! {
                let #field_name = {
                    reader.skip(#field_offset- offset);
                    let read_size = (size - #field_offset).min({ #size_tokens });
                    offset = #field_offset + read_size;
                    <#field_type as #blueflame::memory::MemObject>::read_sized(reader, read_size)?
                };
            });
            writer_impl.extend(quote! {
                writer.skip(#field_offset- offset);
                let write_size = (size - #field_offset).min({ #size_tokens });
                offset = #field_offset + write_size;
                <#field_type as #blueflame::memory::MemObject>::write_sized(&self.#field_name, writer, write_size)?;
            });
        } else {
            reader_impl.extend(quote! {
                let #field_name = {
                    reader.skip(#field_offset - offset);
                    let read_size = { #size_tokens };
                    offset = #field_offset + read_size;
                    <#field_type as #blueflame::memory::MemObject>::read_sized(reader, read_size)?
                };
            });
            writer_impl.extend(quote! {
                writer.skip(#field_offset - offset);
                let write_size = { #size_tokens };
                offset = #field_offset + write_size;
                <#field_type as #blueflame::memory::MemObject>::write_sized(&self.#field_name, writer, write_size)?;
            });
        }

        constructor_impl.extend(quote! {
            #field_name,
        });

        layout_struct_impl.extend(quote!{
            pub #field_name: #blueflame::memory::macro_impl::FieldMetadata<#field_type, #curr_offset, #size_tokens>,
        });

        layout_new_impl.extend(quote! {
            #field_name: #blueflame::memory::macro_impl::FieldMetadata::new(),
        });
    }

    let expanded = quote! {
        const _: () = {

            #[automatically_derived]
            impl #blueflame::memory::MemObject for #struct_name {
                fn read_sized(reader: &mut #blueflame::memory::Reader, size: u32) -> ::std::result::Result<Self, #blueflame::memory::Error> {
                    #blueflame::memory::macro_impl::assert_size_range::<Self>(
                        #last_field_offset,
                        #size,
                        size,
                        "read_sized"
                    )?;
                    let mut offset: u32 = 0;
                    #reader_impl
                    reader.skip(size - offset);
                    Ok(Self { #constructor_impl })
                }
                fn write_sized(&self, writer: &mut #blueflame::memory::Writer, size: u32) -> ::std::result::Result<(), #blueflame::memory::Error> {
                    #blueflame::memory::macro_impl::assert_size_range::<Self>(
                        #last_field_offset,
                        #size,
                        size,
                        "read_sized"
                    )?;
                    let mut offset: u32 = 0;
                    #writer_impl
                    writer.skip(size - offset);
                    Ok(())
                }
            }

            #[automatically_derived]
            impl #blueflame::memory::MemSized for #struct_name {
                const SIZE: u32 = #size;
            }

            #[automatically_derived]
            impl #blueflame::memory::macro_impl::GetLayout for #struct_name {
                type Layout = __Layout;
                fn __layout() -> Self::Layout { __Layout::new() }
            }

            #[automatically_derived]
            pub struct __Layout {
                #layout_struct_impl
            }
            #[automatically_derived]
            impl __Layout {
                pub const fn new() -> Self {
                    Self {
                        #layout_new_impl
                    }
                }
            }

            #size_asserts
        };
    };

    Ok(expanded.into())
}

// #[proc_macro_derive(MemWrite, attributes(offset, size))]
// pub fn derive_mem_write(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = input.ident;
//
//     let size_attr = input.attrs.iter().find(|attr| attr.path().is_ident("size"));
//     let size_value = size_attr.and_then(|attr| {
//         let meta_list = attr.meta.require_list().ok()?;
//         let lit = meta_list.parse_args::<Lit>().ok()?;
//         if let Lit::Int(v) = lit {
//             v.base10_parse::<u32>().ok()
//         } else {
//             None
//         }
//     });
//
//     let fields = match input.data {
//         Data::Struct(DataStruct {
//             fields: Fields::Named(fields),
//             ..
//         }) => fields.named,
//         Data::Enum(DataEnum { variants, .. }) => {
//             let _variant_writers = variants.iter().map(|variant| {
//                 let variant_name = &variant.ident;
//                 let result = variant.discriminant.as_ref().ok_or_else(|| {
//                     quote! { return Err(Error::Mem(crate::memory::Error::Unexpected("Missing discriminant for enum variant".to_string()))) }
//                 });
//
//                 match result {
//                     Ok(discriminant) => match &discriminant.1 {
//                         syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(value), .. }) => {
//                             let value = value.base10_parse::<i32>().unwrap_or(-1);
//                             quote! { #value => Ok(#struct_name::#variant_name), }
//                         }
//                         syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Neg(_), expr, .. }) => {
//                             if let syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(value), .. }) = &**expr {
//                                 let value = -value.base10_parse::<i32>().unwrap_or(1);
//                                 quote! { #value => Ok(#struct_name::#variant_name), }
//                             } else {
//                                 quote! { _ => Err(Error::Mem(crate::memory::Error::Unexpected("Unsupported negative enum discriminant".to_string()))), }
//                             }
//                         }
//                         _ => quote! { _ => Err(Error::Mem(crate::memory::Error::Unexpected("Enum discriminant must be an integer".to_string()))), },
//                     },
//                     Err(q) => q,
//                 }
//             });
//
//             let expanded = quote! {
//                 impl MemWrite for #struct_name {
//                     fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//                         writer.write_i32(self as i32)?;
//                         Ok(())
//                     }
//
//                     // // // TODO --cleanup: remove
//                     // fn write_total_offset(&self) -> u32 {
//                     //     let t = 0i32;
//                     //     t.write_total_offset()
//                     // }
//                 }
//             };
//
//             return TokenStream::from(expanded);
//         }
//         _ => {
//             return TokenStream::from(
//                 quote! { compile_error!("MemWrite can only be used on structs with named fields"); },
//             )
//         }
//     };
//
//     let mut field_writers = Vec::new();
//     for field in fields.iter() {
//         let field_name = match &field.ident {
//             Some(name) => name,
//             None => return quote!(compile_error!("All fields must be named")).into(),
//         };
//
//         let mut offset = None;
//         for attr in &field.attrs {
//             if attr.path().is_ident("offset") {
//                 if let Ok(meta_list) = attr.meta.require_list() {
//                     if let Ok(lit) = meta_list.parse_args::<Lit>() {
//                         if let Lit::Int(v) = lit {
//                             offset = Some(v.base10_parse::<u32>().unwrap_or(0));
//                         }
//                     }
//                 }
//             }
//         }
//
//         let offset = match offset {
//             Some(o) => o,
//             None => return quote!(compile_error!("Missing offset attribute")).into(),
//         };
//
//         field_writers.push(quote! {
//             writer.skip(#offset - cur_offset);
//             cur_offset = #offset;
//             let write_offset = self.#field_name.write_total_offset();
//             let #field_name = self.#field_name.write_to_mem(writer)?;
//             cur_offset += write_offset;
//         });
//     }
//
//     let last_field = match fields.last() {
//         Some(f) => f,
//         None => return TokenStream::from(quote! { compile_error!("No fields in struct"); }),
//     };
//
//     let last_field_name = last_field.ident.as_ref().expect("Missing ident");
//     let mut last_offset = None;
//     for attr in &last_field.attrs {
//         if attr.path().is_ident("offset") {
//             if let Ok(meta_list) = attr.meta.require_list() {
//                 if let Ok(lit) = meta_list.parse_args::<Lit>() {
//                     if let Lit::Int(v) = lit {
//                         last_offset = Some(v.base10_parse::<u32>().unwrap_or(0));
//                     }
//                 }
//             }
//         }
//     }
//
//     let last_offset = last_offset.unwrap_or(0);
//
//     let write_total_offset = match size_value {
//         Some(size) => quote! { #size },
//         None => quote! { #last_offset + self.#last_field_name.write_total_offset() },
//     };
//
//     let writer_skip = match size_value {
//         Some(size) => quote! { writer.skip(#size - cur_offset); },
//         None => quote! {},
//     };
//
//     let expanded = quote! {
//         impl MemWrite for #struct_name {
//             fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
//                 let mut cur_offset: u32 = 0;
//                 #(#field_writers)*
//                 #writer_skip
//                 Ok(())
//             }
//
//             fn write_total_offset(&self) -> u32 {
//                 #write_total_offset
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }

struct FieldData<'a, 'b> {
    pub offset: u32,
    pub size_tokens: TokenStream2,
    pub size_span: proc_macro2::Span,
    pub name: &'a syn::Ident,
    pub typ: &'b syn::Type,
}
