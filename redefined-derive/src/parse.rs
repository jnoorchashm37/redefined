use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Attribute, Data, DeriveInput, Error, GenericParam, Generics,
    Ident, LitStr, Path, PathArguments, Type,
};

use crate::{
    r#enum::generate_enum_impl,
    r#struct::{generate_struct_impl, generate_struct_impl_with_generics},
};

pub fn expand_derive_redefined(input: &mut DeriveInput) -> syn::Result<TokenStream> {
    let target_struct = &input.ident;
    let (source_struct, _, new_source_func) = &extract_source_type(&input.attrs)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl().clone();

    let gen = match &input.data {
        Data::Struct(data_struct) => {
            //generate_struct_impl(target_struct, source_struct, data_struct, input,
            // new_source_func)?
            generate_struct_impl_with_generics(
                target_struct,
                source_struct,
                &ty_generics,
                where_clause,
                data_struct,
                input,
                new_source_func,
            )?
        }
        Data::Enum(data_enum) => generate_enum_impl(data_enum, target_struct, source_struct)?,
        _ => return Err(syn::Error::new_spanned(input, "Expected an enum")),
    };

    Ok(gen)
}

fn extract_source_type(
    attrs: &[Attribute],
) -> syn::Result<(TokenStream, Option<Vec<Ident>>, Option<LitStr>)> {
    let mut res = None;
    let mut new_src_func = None;
    let mut generics: Option<Vec<Ident>> = None;

    for attr in attrs {
        if attr.path().is_ident("redefined") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("to_source") {
                    let args: LitStr = meta.value()?.parse()?;
                    new_src_func = Some(args.clone());
                } else if meta.path.is_ident("generics") {
                    meta.parse_nested_meta(|meta| {
                        if let Some(idt) = meta.path.get_ident() {
                            if let Some(ge) = &mut generics {
                                ge.push(idt.clone());
                            } else {
                                generics = Some(vec![idt.clone()]);
                            }
                        };
                        Ok(())
                    })?
                } else {
                    let full_path = meta.path.clone();

                    if let PathArguments::AngleBracketed(args) =
                        &full_path.segments.last().unwrap().arguments
                    {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(ty) = arg {
                                if let Some(ge) = &mut generics {
                                    // ge.push(ty.clone());
                                } else {
                                    // generics = Some(vec![ty.clone()]);
                                }
                            }
                        }
                        res = Some(
                            full_path.to_token_stream(), /* .cloned()
                                                         .ok_or(syn::Error::new_spanned(
                                                             meta.path.to_token_stream(),
                                                             "Expected an identifier for 'redefined' attribute",
                                                         ))?, */
                        );
                    } else {
                        res = Some(
                            full_path.to_token_stream(), /* .cloned()
                                                         .ok_or(syn::Error::new_spanned(
                                                             meta.path.to_token_stream(),
                                                             "Expected an identifier for 'redefined' attribute",
                                                         ))?, */
                        );
                    }
                }
                Ok(())
            })?;
        }
    }

    if let Some(r) = res {
        Ok((r, generics, new_src_func))
    } else {
        Err(Error::new_spanned(
            attrs.first().expect("Expected at least one attribute"),
            "Attribute 'redefined' not found or not correctly formatted",
        ))
    }
}

// Define a struct to hold the parsed attribute
struct RedefinedAttribute {
    path: Path,
}

// Implement Parse for your attribute
impl Parse for RedefinedAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        // Parsing something like `GenericStructA<X, Y>`
        let _ = syn::parenthesized!(content in input);
        let path = content.parse()?;

        Ok(RedefinedAttribute { path })
    }
}
