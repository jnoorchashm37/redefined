use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, parse::Parse, spanned::Spanned, Attribute, DataEnum, DataStruct, Field, Fields, Generics, Ident, Type, Variant, Visibility};

use crate::{
    attributes::{symbol::USE_FIELD, ContainerAttributes},
    new_types::r#struct::parse_field,
};

pub fn parse_new_enum(
    data_enum: &DataEnum,
    enum_name: &Ident,
    new_enum_name: &Ident,
    generics: &Generics,
    visibility: &Visibility,
    attributes: &[Attribute],
) -> syn::Result<TokenStream> {
    let enum_fields = data_enum
        .variants
        .iter()
        .map(|variant| parse_enum_variant(variant))
        .collect::<syn::Result<Vec<_>>>()?;

    let tokens = quote! {
        #[derive(Redefined)]
        #[redefined(#enum_name)]
        #(#attributes)*
        #visibility enum #new_enum_name #generics {
            #(#enum_fields),*
        }
    };

    Ok(tokens)
}

fn parse_enum_variant(variant: &Variant) -> syn::Result<TokenStream> {
    let discriminant = &variant.discriminant;
    let ident = &variant.ident;
    let mut copied_field_attrs = Vec::new();

    let fields = match &variant.fields {
        Fields::Named(fields) => {
            let f = fields
                .named
                .iter()
                .map(|f| parse_field(f))
                .collect::<Result<Vec<_>, _>>()?;
            quote! { {#(#f),* }}
        }
        Fields::Unnamed(fields) => {
            let f = fields
                .unnamed
                .iter()
                .map(|f| parse_field(f))
                .collect::<Result<Vec<_>, _>>()?;
            quote! { (#(#f),*)}
        }
        Fields::Unit => Default::default(),
    };

    for attr in &variant.attrs {
        if !attr.path().is_ident("redefined") {
            copied_field_attrs.push(attr)
        }
    }

    let tokens = if let Some((eq, expr)) = discriminant {
        quote! {
            #(#copied_field_attrs)*
            #ident #eq #expr,
        }
    } else {
        quote! {
            #(#copied_field_attrs)*
            #ident #fields
        }
    };

    Ok(tokens)
}
