use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{new_types::parse_type_without_source, outer::OuterContainer, redefined_types::Container};

pub fn expand_derive_redefined(input: &DeriveInput) -> syn::Result<TokenStream> {
    let outer = OuterContainer::parse(input.clone().ident, &input.attrs)?;

    let derive_tokens = if outer.source_type.is_none() {
        parse_type_without_source(outer, input)?
    } else {
        let container = Container::parse_sub_containers(outer, &input.data, &input.generics)?;
        container.finalize()
    };

    Ok(derive_tokens)
}
