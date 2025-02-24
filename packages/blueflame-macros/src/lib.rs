use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields,
    Lit
};

#[proc_macro_derive(MemRead, attributes(offset))]
pub fn derive_mem_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("MemRead can only be used on structs with named fields"),
    };

    let field_accessors = fields.clone().into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let field_type = field.ty;
        let mut offset: Option<u32> = None;

        for attr in field.attrs {
            if attr.path().is_ident("offset") {
                let meta_list = attr.meta.require_list().unwrap();
                let l = meta_list.parse_args::<Lit>().unwrap();
                match l {
                    Lit::Int(v) => offset = Some(v.base10_parse::<u32>().unwrap()),
                    _ => offset = None,
                }
            }
        }

        let offset = offset.expect("Need offset value");

        quote! {
            reader.skip(#offset - cur_offset);
            cur_offset = cur_offset + #offset;
            let #field_name = <#field_type>::read_from_mem(reader)?;
            cur_offset = cur_offset + <#field_type>::read_total_offset();
        }
    });

    let last_field = fields.last().unwrap().to_owned();
    let last_field_type = last_field.ty;
    let mut last_offset: Option<u32> = None;

    for attr in last_field.attrs {
        if attr.path().is_ident("offset") {
            let meta_list = attr.meta.require_list().unwrap();
            let l = meta_list.parse_args::<Lit>().unwrap();
            match l {
                Lit::Int(v) => last_offset = Some(v.base10_parse::<u32>().unwrap()),
                _ => last_offset = None,
            }
        }
    }

    let last_offset = last_offset.expect("Need offset value");

    let field_names = fields.clone().into_iter().map(|field| {
        let field_name = field.ident.unwrap();

        quote! {
            #field_name,
        }
    });

    let expanded = quote! {
        impl MemRead for #struct_name {
            fn read_from_mem(reader: &mut Reader) -> Result<Self, Error> {
                let mut cur_offset: u32 = 0;
                #(#field_accessors)*
                Ok(#struct_name {
                    #(#field_names)*
                })
            }

            fn read_total_offset() -> u32 {
                #last_offset + <#last_field_type>::read_total_offset()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(MemWrite, attributes(offset))]
pub fn derive_mem_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("MemRead can only be used on structs with named fields"),
    };

    let field_writers = fields.clone().into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        // let field_type = field.ty;
        let mut offset: Option<u32> = None;

        for attr in field.attrs {
            if attr.path().is_ident("offset") {
                let meta_list = attr.meta.require_list().unwrap();
                let l = meta_list.parse_args::<Lit>().unwrap();
                match l {
                    Lit::Int(v) => offset = Some(v.base10_parse::<u32>().unwrap()),
                    _ => offset = None,
                }
            }
        }

        let offset = offset.expect("Need offset value");

        quote! {
            writer.skip(#offset - cur_offset);
            cur_offset = cur_offset + #offset;
            cur_offset = cur_offset + self.#field_name.write_total_offset();
            let #field_name = self.#field_name.write_to_mem(writer)?;
        }
    });

    let last_field = fields.last().unwrap().to_owned();
    let last_field_name = last_field.ident;
    let mut last_offset: Option<u32> = None;

    for attr in last_field.attrs {
        if attr.path().is_ident("offset") {
            let meta_list = attr.meta.require_list().unwrap();
            let l = meta_list.parse_args::<Lit>().unwrap();
            match l {
                Lit::Int(v) => last_offset = Some(v.base10_parse::<u32>().unwrap()),
                _ => last_offset = None,
            }
        }
    }

    let last_offset = last_offset.expect("Need offset value");

    let expanded = quote! {
        impl MemWrite for #struct_name {
            fn write_to_mem(self, writer: &mut Writer) -> Result<(), Error> {
                let mut cur_offset: u32 = 0;
                #(#field_writers)*
                Ok(())
            }

            fn write_total_offset(&self) -> u32 {
                #last_offset + self.#last_field_name.write_total_offset()
            }
        }
    };

    TokenStream::from(expanded)
}
