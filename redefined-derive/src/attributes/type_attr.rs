use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, bracketed, parenthesized, parse::Parse, parse_quote, Expr, Ident, LitStr, Token, TypeTuple};

use super::symbol::*;

#[derive(Clone)]
pub struct TypeAttribute {
    pub symbol:            Symbol,
    pub nv_tokens:         Option<Expr>,
    pub list_idents:       Option<Vec<Ident>>,
    pub list_tuple_idents: Option<Vec<(Ident, Ident)>>,
    pub list_other_attrs:  Option<Vec<TokenStream>>,
}

impl Debug for TypeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeAttribute")
            .field("symbol", &self.symbol)
            .field(
                "nv_tokens",
                &self
                    .nv_tokens
                    .clone()
                    .map(|e| e.into_token_stream().to_string()),
            )
            .field("list_idents", &self.list_idents)
            .finish()
    }
}

impl Parse for TypeAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let symbol: Symbol = input.parse()?;

        let this = match symbol.meta {
            SymbolMeta::Path => Self { symbol, nv_tokens: None, list_idents: None, list_tuple_idents: None, list_other_attrs: None },
            SymbolMeta::List => {
                //let t = input.parse::<Ident>()?;
                // panic!("NONONO, {}", t);
                let content;
                parenthesized!(content in input);

                if symbol == OTHER_ATTR {
                    let other_container_attrs = content
                        .parse_terminated(TokenStream::parse, Token![#])?
                        .into_iter()
                        .skip(1)
                        .map(|stream| quote!(# #stream ))
                        .collect::<Vec<_>>();

                    Self { symbol, nv_tokens: None, list_idents: None, list_tuple_idents: None, list_other_attrs: Some(other_container_attrs) }
                } else if content.peek(syn::Ident) {
                    let idents = content
                        .parse_terminated(Ident::parse, Token![,])?
                        .into_iter()
                        .collect();
                    Self { symbol, nv_tokens: None, list_idents: Some(idents), list_tuple_idents: None, list_other_attrs: None }
                } else {
                    let idents = content
                        .parse_terminated(TypeTuple::parse, Token![,])?
                        .into_iter()
                        .map(|mut tupl| {
                            let ident1 = match tupl
                                .elems
                                .pop()
                                .expect("Tuple argments must have 2")
                                .into_value()
                            {
                                syn::Type::Path(p) => p.path.get_ident().unwrap().clone(),
                                _ => unreachable!("Tuple Ident 2 must be a path"),
                            };
                            let ident0 = match tupl
                                .elems
                                .pop()
                                .expect("Tuple argments must have 2")
                                .into_value()
                            {
                                syn::Type::Path(p) => p.path.get_ident().unwrap().clone(),
                                _ => unreachable!("Tuple Ident 2 must be a path"),
                            };
                            (ident0, ident1)
                        })
                        .collect::<Vec<_>>();
                    Self { symbol, nv_tokens: None, list_idents: None, list_tuple_idents: Some(idents), list_other_attrs: None }
                }
            }
            SymbolMeta::NameValue => {
                input.parse::<Token![=]>()?;
                let nv = input.parse::<Expr>()?;
                let lit_nv: LitStr = parse_quote!(#nv);
                Self { symbol, nv_tokens: Some(lit_nv.parse()?), list_idents: None, list_tuple_idents: None, list_other_attrs: None }
            }
        };

        Ok(this)
    }
}
