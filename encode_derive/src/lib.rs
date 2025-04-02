use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let encode_body = match &input.data {
        Data::Struct(data) => {
            let field_encode = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                quote! {encoded.extend(self.#field_name.encode());}
            });

            quote! {
                let mut encoded = Vec::new();
                #(#field_encode)*
                encoded
            }
        }
        _ => panic!("Encode derive is only intended for structs"),
    };

    let expanded = quote! {
        impl Encode for #name {
            fn encode(&self) -> Vec<u8> {
                #encode_body
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let decode_body = match &input.data {
        Data::Struct(data) => {
            let field_decodes = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
                quote! {
                    let #field_name = <#field_type as Decode>::decode(&bytes, &mut offset);
                }
            });

            let field_assignments = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                quote! { #field_name }
            });

            quote! {
                let mut offset = 0;
                #(#field_decodes)*
                Self { #(#field_assignments),* }
            }
        }
        _ => panic!("Decode can only be derived for structs"),
    };

    let expanded = quote! {
        impl Decode for #name {
            fn decode(bytes: &[u8], offset: &mut usize) -> Self {
                #decode_body
            }
        }
    };

    TokenStream::from(expanded)
}
