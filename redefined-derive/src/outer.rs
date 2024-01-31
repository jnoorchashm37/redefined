use syn::{self, parse::Parse, Attribute, Ident};

#[cfg(feature = "unsafe")]
use crate::attributes::symbol::TRANSMUTE;
use crate::attributes::{
    symbol::{Symbol, FROM_SOURCE_FN, TO_SOURCE_FN},
    type_attr::TypeAttribute,
    ContainerAttributes,
};

pub struct OuterContainer {
    pub target_type:     Ident,
    pub source_type:     Option<Ident>,
    pub container_attrs: Vec<TypeAttribute>,
}

impl OuterContainer {
    pub fn parse(target_type: Ident, attrs: &[Attribute]) -> syn::Result<Self> {
        let mut container_attrs = Vec::new();
        let mut source_type = None;

        for attr in attrs.iter() {
            if attr.path().is_ident("redefined_attr") {
                container_attrs = attr.parse_args_with(ContainerAttributes::parse)?.0;
            }

            if attr.path().is_ident("redefined") {
                source_type = attr.parse_args_with(SourceType::parse)?.0;
            }
        }

        Symbol::illegal_pairings(&container_attrs.iter().map(|c| c.symbol).collect::<Vec<_>>(), source_type.is_some());

        Ok(Self { target_type, source_type, container_attrs })
    }

    pub fn should_parse_fields(&self) -> bool {
        let symbols = self
            .container_attrs
            .iter()
            .map(|s| s.symbol)
            .collect::<Vec<_>>();

        #[cfg(feature = "unsafe")]
        return !(symbols.contains(&TO_SOURCE_FN) && symbols.contains(&FROM_SOURCE_FN)) || symbols.contains(&TRANSMUTE);

        #[cfg(not(feature = "unsafe"))]
        return !(symbols.contains(&TO_SOURCE_FN) && symbols.contains(&FROM_SOURCE_FN));
    }

    pub fn get_symbol(&self, symbol: Symbol) -> Option<TypeAttribute> {
        self.container_attrs
            .iter()
            .find(|s| s.symbol == symbol)
            .cloned()
    }
}

struct SourceType(Option<Ident>);

impl Parse for SourceType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let source_type = if input.peek(Ident) { Some(input.parse()?) } else { None };

        Ok(Self(source_type))
    }
}
