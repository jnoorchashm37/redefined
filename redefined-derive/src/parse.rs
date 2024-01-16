use proc_macro2::{Span, TokenStream};
use quote::quote;

use syn::{
    self, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Variant,
};
use syn::{Attribute, Error};

pub fn expand_derive_redefined(input: &mut DeriveInput) -> syn::Result<TokenStream> {
    let target_struct = &input.ident;
    let source_struct = extract_source_struct(&input.attrs)?;

    let gen = match &input.data {
        Data::Struct(data_struct) => {
            generate_struct_impl(target_struct, &source_struct, data_struct, input)?
        }
        Data::Enum(data_enum) => generate_enum_impl(data_enum, target_struct, &source_struct)?,
        _ => return Err(syn::Error::new_spanned(input, "Expected an enum")),
    };

    Ok(gen)
}

fn generate_enum_impl(
    data_enum: &syn::DataEnum,
    target_struct: &Ident,
    source_struct: &Ident,
) -> syn::Result<TokenStream> {
    let variants_to = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        generate_variant_match(variant, source_struct, target_struct, variant_name, false)
    });

    let variants_from = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        generate_variant_match(variant, target_struct, source_struct, variant_name, true)
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

fn generate_variant_match(
    variant: &Variant,
    source_struct: &Ident,
    target_struct: &Ident,
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

fn generate_struct_impl(
    target_struct: &Ident,
    source_struct: &Ident,
    data_struct: &DataStruct,
    input: &DeriveInput,
) -> syn::Result<TokenStream> {
    let matched_fields = match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|f| &f.ident)
            .collect::<Vec<_>>(),
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Expected a struct with named fields",
            ))
        }
    };

    let gen = quote! {
        impl RedefinedConvert<#source_struct> for #target_struct {
            fn from_source(src: #source_struct) -> Self {
                #target_struct {
                    #(
                        #matched_fields: RedefinedConvert::from_source(src.#matched_fields),
                    )*
                }
            }

            fn to_source(self) -> #source_struct {
                #source_struct {
                    #(
                        #matched_fields: self.#matched_fields.to_source(),
                    )*
                }
            }
        }
    };

    Ok(gen)
}

fn extract_source_struct(attrs: &[Attribute]) -> syn::Result<Ident> {
    for attr in attrs {
        if attr.path().is_ident("redefined") {
            return attr.parse_args().map_err(|e| {
                syn::Error::new_spanned(attr, format!("Error parsing 'redefined' attribute: {}", e))
            });
        }
    }
    Err(Error::new_spanned(
        attrs.first().expect("Expected at least one attribute"),
        "Attribute 'redefined' not found or not correctly formatted",
    ))
}
