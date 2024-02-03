mod file_parser;
mod package;
mod types;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{bracketed, parenthesized, parse::Parse, spanned::Spanned, DeriveInput, LitStr, Token};

use self::{package::Package, types::write_to_file_cache};
use crate::derive;

pub fn expand_redefined_remote(input: TokenStream) -> syn::Result<TokenStream> {
    let parsed: RemoteType = syn::parse2(input)?;

    parsed.execute()
}

/// parses the remote type into tokens
fn parse_remote_type_text(remote_type_name: &str, remote_type_text: &str, derives: Vec<Ident>, no_impl: bool) -> syn::Result<TokenStream> {
    let remote_type_text = remote_type_text.replace(remote_type_name, &format!("{}Redefined", remote_type_name));

    let struct_def: DeriveInput = syn::parse_str(&remote_type_text)?;
    let redefined_struct_def = derive::expand_derive_redefined(&struct_def, true).unwrap_or_else(syn::Error::into_compile_error);

    let remote_type = Ident::new(remote_type_name, struct_def.span());

    let tokens = if no_impl {
        let mut derives = derives;
        derives.retain(|d| d.to_string() != "Redefined");
        quote! {
        #[derive(#(#derives),*)]
        #redefined_struct_def
        }
    } else {
        quote! {

            #[derive(#(#derives),*)]
            #[redefined(#remote_type)]
            #[redefined_attr(transmute)]
            #redefined_struct_def
        }
    };

    Ok(tokens)
}

#[derive(Debug, Clone)]
pub struct RemoteType {
    pub name:    Ident,
    pub package: Package,
    pub derives: Vec<Ident>,
    pub no_impl: bool,
}

impl RemoteType {
    /// runs the remote type execution
    /// added for future use in fields of structs
    pub fn execute(self) -> syn::Result<TokenStream> {
        let derives = self.derives.clone();

        let remote_type_text = self
            .package
            .fetch_from_file_cache(&self.name.to_string())
            .type_text;

        let tokens = parse_remote_type_text(&self.name.to_string(), &remote_type_text, derives, self.no_impl);

        tokens
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
            parenthesized!(paran_derive in bracketed_derive);

            derives.extend(
                paran_derive
                    .parse_terminated(Ident::parse, Token![,])?
                    .into_iter()
                    .collect::<Vec<_>>(),
            );
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

        let this = Self { name, package, derives, no_impl };

        //panic!("NO IMPL: \n{:?}", this);

        Ok(this)
    }
}
