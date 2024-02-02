use std::fmt::{self, Display};

use syn::{parse::Parse, Ident, Path};

use super::type_attr::TypeAttribute;

pub const _ALL_FIELD_SYMBOLS: [&Symbol; 1] = [&FIELD_FN];

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum SymbolMeta {
    Path,
    List,
    NameValue,
}

#[cfg(feature = "unsafe")]
pub const TRANSMUTE: Symbol = Symbol { s: "transmute", is_container: true, meta: SymbolMeta::Path };
pub const TO_SOURCE_FN: Symbol = Symbol { s: "to_source", is_container: true, meta: SymbolMeta::NameValue };
pub const FROM_SOURCE_FN: Symbol = Symbol { s: "from_source", is_container: true, meta: SymbolMeta::NameValue };
pub const DERIVE: Symbol = Symbol { s: "derive", is_container: true, meta: SymbolMeta::List };
pub const FIELD_FN: Symbol = Symbol { s: "func", is_container: false, meta: SymbolMeta::NameValue };
pub const USE_FIELD: Symbol = Symbol { s: "field", is_container: false, meta: SymbolMeta::List };
pub const USE_DEFAULT_FIELD: Symbol = Symbol { s: "default", is_container: false, meta: SymbolMeta::Path };
pub const USE_SAME_FIELD_VALUE: Symbol = Symbol { s: "same", is_container: false, meta: SymbolMeta::Path };

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Symbol {
    pub s:            &'static str,
    pub is_container: bool,
    pub meta:         SymbolMeta,
}

impl Symbol {
    pub fn contained_in(&self, vals: &[TypeAttribute]) -> bool {
        vals.iter().any(|val| val.symbol.s == self.s)
    }

    pub fn find_type_attr(&self, vals: &[TypeAttribute]) -> Option<TypeAttribute> {
        vals.iter().find(|val| val.symbol.s == self.s).cloned()
    }

    pub fn illegal_pairings(symbols: &[Symbol], has_source_type: bool) {
        #[cfg(feature = "unsafe")]
        if symbols.contains(&TRANSMUTE) && symbols.len() > 1 {
            panic!("Cannot have transmute attribute with other container attributes: {:?}", symbols);
        }

        if (symbols.contains(&FROM_SOURCE_FN) || symbols.contains(&TO_SOURCE_FN)) && !has_source_type {
            panic!("Cannot have to/from attributes without a source type: {:?}", symbols);
        }
    }
}

impl Parse for Symbol {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let binding = ident.to_string();
        let s = binding.as_str();

        Ok(s.into())
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        match value {
            #[cfg(feature = "unsafe")]
            "transmute" => TRANSMUTE,
            "to_source" => TO_SOURCE_FN,
            "from_source" => FROM_SOURCE_FN,
            "func" => FIELD_FN,
            "field" => USE_FIELD,
            "derive" => DERIVE,
            "default" => USE_DEFAULT_FIELD,
            "same" => USE_SAME_FIELD_VALUE,
            _ => panic!("No attribute for {}", value),
        }
    }
}

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.s
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.s
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.s)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.s)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.s)
    }
}
