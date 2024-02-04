use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{
    new_types::{parse_type_without_source, r#struct::parse_type_to_redefined},
    outer::OuterContainer,
    redefined_types::RedefinedContainer,
};

pub fn expand_derive_redefined(input: &DeriveInput, is_remote: bool) -> syn::Result<TokenStream> {
    let outer = OuterContainer::parse(input.clone().ident, &input.attrs)?;

    let derive_tokens = if outer.source_type.is_none() {
        parse_type_without_source(outer, input)?
    } else {
        let mut generics = input.generics.clone();
        if is_remote {
            generics.params.iter_mut().for_each(|param| match param {
                syn::GenericParam::Type(path) => {
                    if let Some(default_val) = path.default.as_mut() {
                        *default_val = parse_type_to_redefined(default_val, &Default::default(), Default::default())
                    }
                }
                _ => (),
            });
        }
        let container = RedefinedContainer::parse_sub_containers(outer, &input.data, &generics)?;
        container.finalize()
    };

    Ok(derive_tokens)
}
