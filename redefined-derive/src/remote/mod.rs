mod file_parser;
mod package;
mod types;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{bracketed, parenthesized, parse::Parse, spanned::Spanned, DeriveInput, LitStr, Token};

use self::{package::Package, types::write_to_file_cache};
use crate::derive;

pub fn expand_redefined_remote(input: TokenStream) -> syn::Result<TokenStream> {
    let parsed: RemoteType = syn::parse2(input)?;

    parsed.execute()
}

#[derive(Debug, Clone)]
pub struct RemoteType {
    pub name:        Ident,
    pub package:     Package,
    pub derives:     Vec<Ident>,
    pub other_attrs: TokenStream,
    pub no_impl:     bool,
}

impl RemoteType {
    /// runs the remote type execution
    /// added for future use in fields of structs
    pub fn execute(self) -> syn::Result<TokenStream> {
        let remote_type_text = self
            .package
            .fetch_from_file_cache(&self.name.to_string())
            .type_text;

        let tokens = self.parse_remote_type_text(&remote_type_text);

        tokens
    }

    /// parses the remote type into tokens
    fn parse_remote_type_text(&self, remote_type_text: &str) -> syn::Result<TokenStream> {
        let remote_type_name = self.name.to_string();
        let (other_attr, derives) = (&self.other_attrs, &self.derives);
        let tokens = if self.no_impl {
            let struct_def: DeriveInput = syn::parse_str(&remote_type_text)?;
            let redefined_struct_def = derive::expand_derive_redefined(&struct_def).unwrap_or_else(syn::Error::into_compile_error);

            let mod_redefined_struct_def = redefined_struct_def
                .to_string()
                .replace("#[derive(Redefined)]", "")
                .replace(&format!("#[redefined({})]", remote_type_name), "");

            let final_struct_def: DeriveInput = syn::parse_str(&mod_redefined_struct_def)?;

            let mut derives = derives.clone();
            derives.retain(|d| d.to_string() != "Redefined");
            quote! {
            #[derive(#(#derives),*)]
            #other_attr
            #final_struct_def
            }
        } else {
            // let remote_type_text = remote_type_text
            //     .replace(&format!("struct {}", remote_type_name), &format!("struct
            // {}Redefined", remote_type_name))     .replace(&format!("enum {}",
            // remote_type_name), &format!("enum {}Redefined", remote_type_name));

            let struct_def: DeriveInput = syn::parse_str(&remote_type_text)?;
            let redefined_struct_def = derive::expand_derive_redefined(&struct_def).unwrap_or_else(syn::Error::into_compile_error);

            //panic!("DEF: \n{:?}", redefined_struct_def.to_string());

            let mod_redefined_struct_def = redefined_struct_def
                .to_string()
                .replace("#[derive(Redefined)]", "")
                .replace(&format!("#[redefined({})]", remote_type_name), "");

            let final_struct_def: DeriveInput = syn::parse_str(&mod_redefined_struct_def)?;

            //panic!("DEF: \n{:?}", final_struct_def.to_token_stream().to_string());

            // let struct_def: DeriveInput = syn::parse_str(&remote_type_text)?;

            let remote_type = Ident::new(&remote_type_name, struct_def.span());
            quote! {

                #[derive(#(#derives),*)]
                #[redefined(#remote_type)]
                #[redefined_attr(transmute)]
                #other_attr
                #final_struct_def
            }
        };

        Ok(tokens)
    }
}

impl Parse for RemoteType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut derives = vec![Ident::new("Redefined", input.span())];
        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?; // #

            let bracketed_derive;
            bracketed!(bracketed_derive in input);
            bracketed_derive.parse::<Ident>()?; // derive

            let paran_derive;
            parenthesized!(paran_derive in bracketed_derive); // (..)

            derives.extend(
                paran_derive
                    .parse_terminated(Ident::parse, Token![,])?
                    .into_iter()
                    .collect::<Vec<_>>(),
            );
        }

        let mut other_attrs = Default::default();
        while input.peek(Token![#]) {
            let hash_idnt = input.parse::<Token![#]>()?;

            let bracketed_derive;
            let bracket = bracketed!(bracketed_derive in input);
            let attr: TokenStream = bracketed_derive.parse()?;

            other_attrs = quote! {
                #other_attrs
                #[ #attr ]
            };
        }

        let name: Ident = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Failed to parse name of remote type"))?;

        input.parse::<Token![:]>()?;

        let package_name: LitStr = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Failed to parse url of the remote type's crate/package"))?;

        let package = Package::new(package_name.value())
            .map_err(|_| syn::Error::new(package_name.span(), "Failed to parse the cargo lock for this package"))?;

        let mut no_impl = false;
        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;

            let no_impl_ident: Ident = input
                .parse()
                .map_err(|e| syn::Error::new(e.span(), "Failed to parse no_impl ident - MUST BE 'no_impl'"))?;

            if no_impl_ident.to_string() == "no_impl" {
                no_impl = true
            }
        }

        let this = Self { name, package, derives, no_impl, other_attrs };

        //panic!("NO IMPL: \n{:?}", this);

        Ok(this)
    }
}
