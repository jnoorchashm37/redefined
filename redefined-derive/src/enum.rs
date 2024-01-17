use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

use syn::{
    self, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, LitStr, Variant,
};
use syn::{Attribute, Error};

pub fn generate_enum_impl(
    data_enum: &syn::DataEnum,
    target_struct: &Ident,
    source_struct: &TokenStream,
) -> syn::Result<TokenStream> {
    let variants_to = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        generate_variant_match(variant, source_struct, &target_struct.to_token_stream(), variant_name, false)
    });

    let variants_from = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        generate_variant_match(variant, &target_struct.to_token_stream(), source_struct, variant_name, true)
    });

    let gen = quote! {
        impl RedefinedConvert<#source_struct> for #target_struct {
            fn from_source(src: #source_struct) -> Self {
                match src {
                    #(#variants_to)*
                }
            }

            fn to_source(self) -> #source_struct {
                match self {
                    #(#variants_from)*
                }
            }
        }
    };

    Ok(gen)
}

pub fn generate_variant_match(
    variant: &Variant,
    source_struct: &TokenStream,
    target_struct: &TokenStream,
    variant_name: &Ident,
    is_backwards: bool,
) -> TokenStream {
    match &variant.fields {
        Fields::Unit => quote! {
            #source_struct::#variant_name =>  #target_struct::#variant_name,
        },
        Fields::Named(FieldsNamed { named, .. }) => {
            let field_names: Vec<_> = named
                .into_iter()
                .map(|f| f.ident.as_ref().unwrap().clone())
                .collect();
            let cloned_field_names = field_names.clone();
            if !field_names.is_empty() {
                let field_mappings = cloned_field_names.into_iter().map(|field_name| {
                    if is_backwards {
                        quote! { #field_name: #field_name.to_source() }
                    } else {
                        quote! { #field_name: RedefinedConvert::from_source(#field_name) }
                    }
                });

                quote! {
                    #source_struct::#variant_name { #( #field_names ),* } => #target_struct::#variant_name { #( #field_mappings ),* },
                }
            } else {
                TokenStream::new()
            }
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let field_vars: Vec<_> = (0..unnamed.len())
                .map(|i| syn::Ident::new(&format!("x{}", i), Span::call_site()))
                .collect();

            let destructuring = quote! { (#(#field_vars),*) };

            let construction = if is_backwards {
                quote! {
                    #target_struct::#variant_name(#(#field_vars.to_source()),*)
                }
            } else {
                quote! {
                    #target_struct::#variant_name(#(RedefinedConvert::from_source(#field_vars)),*)
                }
            };

            quote! {
                #source_struct::#variant_name #destructuring => #construction,
            }
        }
    }
}
