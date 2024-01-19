use syn::{self, punctuated::Punctuated, Attribute, Generics, Ident, Meta, Token};

use crate::attributes::{
    symbol::SOURCE_GENERICS,
    type_attr::{parse_attr_meta_into_container, TypeAttribute},
};

pub struct OuterContainer {
    pub target_type:     Ident,
    pub target_generics: Generics,
    pub source_type:     Ident,
    pub source_generics: Vec<Ident>,
    pub container_attrs: Vec<TypeAttribute>,
}

impl OuterContainer {
    pub fn parse(target_type: Ident, target_generics: Generics, attrs: &[Attribute]) -> syn::Result<Self> {
        let mut container_attrs = vec![];

        let mut source_type = None;

        for attr in attrs.iter() {
            if attr.path().is_ident("redefined_attr") {
                let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                container_attrs = parse_attr_meta_into_container(&nested)?;
            }

            if attr.path().is_ident("redefined") {
                attr.parse_nested_meta(|meta| {
                    source_type = Some(
                        meta.path
                            .get_ident()
                            .ok_or(syn::Error::new_spanned(&meta.path, "Source type should be an ident"))?
                            .clone(),
                    );

                    Ok(())
                })?;
            }
        }

        let source_generics = container_attrs
            .iter()
            .find(|attr| attr.symbol == SOURCE_GENERICS)
            .map(|attr| attr.parse_source_generics_attr())
            .transpose()?
            .unwrap_or_default();

        Ok(Self { target_type, target_generics, source_type: source_type.unwrap(), source_generics, container_attrs })
    }
}
