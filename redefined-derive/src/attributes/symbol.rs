use std::fmt::{self, Display};

use syn::{Ident, Path};

use super::type_attr::TypeAttribute;

pub const _ALL_FIELD_SYMBOLS: [&Symbol; 1] = [&FIELD_FN];

#[derive(Copy, Clone, PartialEq)]
pub struct Symbol(pub &'static str);

pub const SOURCE_FN: Symbol = Symbol("to_source");
pub const FIELD_FN: Symbol = Symbol("func");
pub const SOURCE_GENERICS: Symbol = Symbol("source_generics");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl Symbol {
    pub fn contained_in(&self, vals: &[TypeAttribute]) -> bool {
        vals.iter().any(|val| &val.symbol == self)
    }

    pub fn find_type_attr(&self, vals: &[TypeAttribute]) -> Option<TypeAttribute> {
        vals.iter()
            .find(|val| &val.symbol == self).cloned()
    }
}
