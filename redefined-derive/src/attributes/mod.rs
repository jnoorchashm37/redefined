pub mod primitives;
pub mod symbol;
pub mod type_attr;

use syn::{parse::Parse, Token};

use crate::attributes::type_attr::TypeAttribute;

pub struct ContainerAttributes(pub Vec<TypeAttribute>);

impl Parse for ContainerAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs: Vec<TypeAttribute> = input
            .parse_terminated(TypeAttribute::parse, Token![,])?
            .into_iter()
            .collect();

        //panic!("LEN: {:?}", attrs);

        Ok(Self(attrs))
    }
}
