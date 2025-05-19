use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;
use syn::spanned::Spanned as _;

use crate::util::{self, syn_error};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    util::unwrap_result(expand_internal(input))
}

fn expand_internal(input: syn::DeriveInput) -> syn::Result<TokenStream> {
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

struct FieldData<'a, 'b> {
    pub offset: u32,
    pub size_tokens: TokenStream2,
    pub size_span: proc_macro2::Span,
    pub name: &'a syn::Ident,
    pub typ: &'b syn::Type,
}
