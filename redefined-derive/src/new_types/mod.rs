pub mod r#enum;
pub mod r#struct;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput};

use self::{r#enum::parse_new_enum, r#struct::parse_new_struct};
use crate::outer::OuterContainer;

pub fn parse_type_without_source(outer: OuterContainer, input: &DeriveInput) -> syn::Result<TokenStream> {
    let input_data = &input.data;
    let input_generics = &input.generics;
    let source_type = Ident::new(&format!("{}Redefined", outer.target_type), outer.target_type.span());

    let new_type_tokens = match &input_data {
        Data::Struct(data_struct) => parse_new_struct(data_struct, &outer.target_type, &source_type, input_generics, &input.vis, &input.attrs),
        Data::Enum(data_enum) => parse_new_enum(data_enum, &outer.target_type, &source_type, input_generics, &input.vis, &input.attrs),
        _ => return Err(syn::Error::new_spanned(source_type, "Expected an enum or struct")),
    }?;

    Ok(quote!( #new_type_tokens ))
}
