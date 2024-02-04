use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{new_types::parse_type_without_source, outer::OuterContainer, redefined_types::RedefinedContainer};

pub fn expand_derive_redefined(input: &DeriveInput, is_remote: bool) -> syn::Result<TokenStream> {
    let outer = OuterContainer::parse(input.clone().ident, &input.attrs)?;

    let derive_tokens = if outer.source_type.is_none() {
        parse_type_without_source(outer, input, is_remote)?
    } else {
        let container = RedefinedContainer::parse_sub_containers(outer, &input.data, &input.generics)?;
        container.finalize()
    };

    Ok(derive_tokens)
}
