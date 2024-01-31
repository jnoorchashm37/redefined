use std::fmt::Debug;

use quote::ToTokens;
use syn::{self, parenthesized, parse::Parse, parse_quote, Expr, Ident, LitStr, Token};

use super::symbol::*;

#[derive(Clone)]
pub struct TypeAttribute {
    pub symbol:      Symbol,
    pub nv_tokens:   Option<Expr>,
    pub list_idents: Option<Vec<Ident>>,
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
            SymbolMeta::Path => Self { symbol, nv_tokens: None, list_idents: None },
            SymbolMeta::List => {
                //let t = input.parse::<Ident>()?;
                // panic!("NONONO, {}", t);
                let content;
                parenthesized!(content in input);

                let idents = content
                    .parse_terminated(Ident::parse, Token![,])?
                    .into_iter()
                    .collect();

                Self { symbol, nv_tokens: None, list_idents: Some(idents) }
            }
            SymbolMeta::NameValue => {
                input.parse::<Token![=]>()?;
                let nv = input.parse::<Expr>()?;
                let lit_nv: LitStr = parse_quote!(#nv);
                Self { symbol, nv_tokens: Some(lit_nv.parse()?), list_idents: None }
            }
        };

        Ok(this)
    }
}
