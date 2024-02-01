#![allow(clippy::wrong_self_convention)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive;

mod redefined_types;

mod attributes;

mod outer;

mod new_types;

#[cfg(feature = "remote")]
mod remote;

#[proc_macro_derive(Redefined, attributes(redefined, redefined_attr))]
pub fn derive_redefined(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::expand_derive_redefined(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(feature = "remote")]
#[proc_macro]
pub fn redefined_remote(input: TokenStream) -> TokenStream {
    remote::expand_redefined_remote(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
