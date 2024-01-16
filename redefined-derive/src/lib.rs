#![feature(stmt_expr_attributes)]

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

mod parse;

#[proc_macro_derive(Redefined, attributes(redefined))]
pub fn derive_redefined(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    parse::expand_derive_redefined(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
