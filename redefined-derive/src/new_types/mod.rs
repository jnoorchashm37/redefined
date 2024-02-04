pub mod r#enum;
pub mod r#struct;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Parse, Attribute, Data, DeriveInput};

use self::{r#enum::parse_new_enum, r#struct::parse_new_struct};
use crate::{attributes::ContainerAttributes, new_types::r#struct::parse_type_to_redefined, outer::OuterContainer};

pub fn parse_type_without_source(outer: OuterContainer, input: &DeriveInput, is_remote: bool) -> syn::Result<TokenStream> {
    let input_data = &input.data;

    let mut input_generics = input.generics.clone();
    if is_remote {
        input_generics
            .params
            .iter_mut()
            .for_each(|param| match param {
                syn::GenericParam::Type(path) => {
                    // if let Some(default_val) = path.default.as_mut() {
                    //     *default_val = parse_type_to_redefined(default_val, &Default::default(),
                    // Default::default()) }

                    if path.default.is_some() {
                        path.default = None;
                        path.eq_token = None;
                    }
                }
                _ => (),
            });
        //panic!("DEF: \n{:?}", input_generics.to_token_stream().to_string());
    }
    let source_type = Ident::new(&format!("{}Redefined", outer.target_type), outer.target_type.span());

    let generic_vec = input_generics
        .type_params()
        .map(|p| p.ident.clone())
        .collect::<Vec<_>>();

    let new_type_tokens = match &input_data {
        Data::Struct(data_struct) => {
            parse_new_struct(data_struct, &outer.target_type, &source_type, &input_generics, &input.vis, &input.attrs, &generic_vec)
        }
        Data::Enum(data_enum) => parse_new_enum(data_enum, &outer.target_type, &source_type, &input_generics, &input.vis, &input.attrs, &generic_vec),
        _ => return Err(syn::Error::new_spanned(source_type, "Expected an enum or struct")),
    }?;

    // panic!("NEW TYPE: \n{}", new_type_tokens.to_string());

    Ok(quote!( #new_type_tokens ))
}

pub fn parse_attributes(attrs: &[Attribute], span: Span) -> syn::Result<(Vec<Ident>, Vec<Attribute>, Vec<TokenStream>)> {
    let mut derive_attrs = vec![Ident::new("Redefined", span.clone())];
    let mut container_attrs = Vec::new();
    let mut new_attrs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("redefined_attr") {
            let redef_attrs = attr
                .parse_args_with(ContainerAttributes::parse)?
                .0
                .into_iter()
                .filter_map(|a| a.list_idents)
                .flatten()
                .collect::<Vec<_>>();

            new_attrs.extend(
                attr.parse_args_with(ContainerAttributes::parse)?
                    .0
                    .into_iter()
                    .filter_map(|a| a.list_other_attrs)
                    .flatten()
                    .collect::<Vec<_>>(),
            );

            derive_attrs.extend(redef_attrs);
        } else {
            container_attrs.push(attr.clone())
        }
    }

    Ok((derive_attrs, container_attrs, new_attrs))
}
