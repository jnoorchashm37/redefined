#![allow(clippy::wrong_self_convention)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod parse;

mod redefined_types;

mod attributes;

mod outer;

mod new_types;

#[proc_macro_derive(Redefined, attributes(redefined, redefined_attr))]
pub fn derive_redefined(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    parse::expand_derive_redefined(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
