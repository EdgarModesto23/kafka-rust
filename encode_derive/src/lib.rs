use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericParam};

#[proc_macro_derive(Size)]
pub fn derive_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let size = match &input.data {
        Data::Struct(data) => {
            let field_sizes = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                quote! {self.#field_name.size_in_bytes()}
            });

            quote! {0 #(+ #field_sizes)*}
        }
        Data::Enum(data) => {
            let variant_sizes = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unit => {
                        quote! {
                            #name::#variant_name => 0,
                        }
                    }
                    Fields::Unnamed(fields) => {
                        quote! {
                            #name::#variant_name(val) => val.size_in_bytes(),
                        }
                    }
                    Fields::Named(fields) => {
                        let field_sizes = fields.named.iter().map(|field| {
                            let field_name = &field.ident;
                            quote! { #field_name.size_in_bytes() }
                        });
                        let field_names = fields.named.iter().map(|field| &field.ident);
                        quote! {
                            #name::#variant_name { #(#field_names),* } => 0 #(+ #field_sizes)*,
                        }
                    }
                }
            });

            quote! {
                match self {
                    #(#variant_sizes)*
                }
            }
        }
        _ => panic!("Size derive is only intended for structs and enums"),
    };

    let expanded = quote! {
        impl Size for #name {
            fn size_in_bytes(&self) -> usize {
                #size
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let generic_params_iter = generics.params.iter().filter_map(|param| {
        if let GenericParam::Type(type_param) = param {
            Some(&type_param.ident)
        } else {
            None
        }
    });

    let generic_params: Vec<_> = generic_params_iter.collect();

    let field_encode = match &input.data {
        Data::Struct(data) => {
            let field_encode = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                quote! { encoded.extend(self.#field_name.encode()); }
            });

            quote! {
                let mut encoded = Vec::new();
                #(#field_encode)*
                encoded
            }
        }
        Data::Enum(data) => {
            let variant_encodes = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unit => {
                        quote! {
                            #name::#variant_name => encoded.extend(Vec::new()),
                        }
                    }
                    Fields::Unnamed(fields) => {
                        quote! {
                            #name::#variant_name(val) => {
                                encoded.extend(val.encode());
                            }
                        }
                    }
                    Fields::Named(fields) => {
                        let field_encodes = fields.named.iter().map(|field| {
                            let field_name = &field.ident;
                            quote! {
                                encoded.extend( #field_name.encode());
                            }
                        });

                        let field_names = fields.named.iter().map(|field| &field.ident);

                        quote! {
                            #name::#variant_name { #(#field_names),* } => {
                                #(#field_encodes)*
                            }
                        }
                    }
                }
            });

            quote! {
                let mut encoded = Vec::new();
                match self {
                    #(#variant_encodes)*
                }
                encoded
            }
        }
        _ => panic!("Encode derive is only intended for structs and enums"),
    };

    let expanded = quote! {
        impl<#(#generic_params,)*> Encode for #name<#(#generic_params,)*> #where_clause {
            fn encode(&self) -> Vec<u8> {
                #field_encode
            }
        }
    };

    if generics.params.is_empty() {
        let expanded = quote! {
            impl Encode for #name {
                fn encode(&self) -> Vec<u8> {
                    #field_encode
                }
            }
        };
        TokenStream::from(expanded)
    } else {
        TokenStream::from(expanded)
    }
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let generic_params_iter = generics.params.iter().filter_map(|param| {
        if let GenericParam::Type(type_param) = param {
            Some(&type_param.ident)
        } else {
            None
        }
    });
    let generic_params: Vec<_> = generic_params_iter.collect();

    let field_decode = match &input.data {
        Data::Struct(data) => {
            let field_decodes = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
                quote! { let #field_name = <#field_type as Decode>::decode(bytes, offset); }
            });

            let field_assignments = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                quote! { #field_name }
            });

            quote! {
                #(#field_decodes)*
                Self { #(#field_assignments),* }
            }
        }
        Data::Enum(data) => {
            let variant_decodes = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let decode_variant = if let Some(field) = variant.fields.iter().next() {
                    let field_type = &field.ty;
                    quote! {
                        #variant_name => #name::#variant_name(<#field_type as Decode>::decode(bytes, offset)),
                    }
                } else {
                    panic!("non variant enums are not supported")
                };
                decode_variant
            });

            quote! {
                let variant_index = bytes[*offset];
                *offset += 1;
                match variant_index {
                    #(#variant_decodes)*
                    _ => panic!("Unknown variant"),
                }
            }
        }
        _ => panic!("Decode derive is only intended for structs"),
    };

    let expanded = quote! {
        impl<#(#generic_params,)*> Decode for #name<#(#generic_params,)*> #where_clause {
            fn decode(bytes: &[u8], offset: &mut usize) -> #name<#(#generic_params,)*> {
                #field_decode
            }
        }
    };

    if generics.params.is_empty() {
        let expanded = quote! {
            impl Decode for #name {
                fn decode(bytes: &[u8], offset: &mut usize) -> #name {
                    #field_decode
                }
            }
        };
        TokenStream::from(expanded)
    } else {
        TokenStream::from(expanded)
    }
}
