use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{self, punctuated::Punctuated, AngleBracketedGenericArguments, Ident, Meta, PathArguments, Token};

use super::symbol::{Symbol, FIELD_FN, FROM_SOURCE_FN, SOURCE_GENERICS, TO_SOURCE_FN};

#[derive(Clone)]
pub struct TypeAttribute {
    pub symbol: Symbol,
    pub meta:   Meta,
}

impl TypeAttribute {
    pub fn new(symbol: Symbol, meta: Meta) -> Self {
        Self { symbol, meta }
    }

    pub fn parse_source_type(&self) -> syn::Result<(Ident, Option<AngleBracketedGenericArguments>)> {
        let (source_type, source_generics) = match &self.meta {
            Meta::Path(path) => {
                if path.segments.len() == 1 && path.segments.first().unwrap().arguments.is_none() {
                    let ident = path
                        .get_ident()
                        .ok_or(syn::Error::new_spanned(&self.meta, "Source type should be an ident"))?;
                    (ident.clone(), None)
                } else {
                    let mut inner_ident = None;
                    let mut generics = None;
                    for p in &path.segments {
                        inner_ident = Some(p.ident.clone());
                        match &p.arguments {
                            PathArguments::AngleBracketed(angled) => {
                                generics = Some(angled.clone());
                                break
                            }
                            _ => (),
                        }
                    }
                    (inner_ident.unwrap(), generics)
                }
            }
            _ => unreachable!("The source type should be a Meta::Path(_)"),
        };

        Ok((source_type.clone(), source_generics))
    }

    pub fn try_into_new_source_fn(&self) -> syn::Result<Option<TokenStream>> {
        if self.symbol == TO_SOURCE_FN {
            Ok(Some(self.meta.require_name_value()?.path.to_token_stream()))
        } else {
            Ok(None)
        }
    }

    pub fn try_from_new_source_fn(&self) -> syn::Result<Option<TokenStream>> {
        if self.symbol == FROM_SOURCE_FN {
            Ok(Some(self.meta.require_name_value()?.path.to_token_stream()))
        } else {
            Ok(None)
        }
    }

    pub fn parse_source_generics_attr(&self) -> syn::Result<Vec<Ident>> {
        if self.symbol != SOURCE_GENERICS {
            unreachable!("Called parse_source_generics_attr() when SELF is not SOURCE_GENERICS");
        }

        match &self.meta {
            Meta::List(list) => {
                let nested = list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)?;
                Ok(nested.into_iter().collect::<Vec<_>>())
            }
            _ => unreachable!("SOURCE_GENERICS must be a list"),
        }

        // Ok(vec![])
    }
}

pub fn parse_attr_meta_into_container(inner_meta: &Punctuated<Meta, Token![,]>) -> syn::Result<Vec<TypeAttribute>> {
    let mut attributes = vec![];

    for meta in inner_meta.into_iter() {
        if meta.path() == FROM_SOURCE_FN {
            attributes.push(TypeAttribute::new(FROM_SOURCE_FN, meta.clone()))
        }
        if meta.path() == TO_SOURCE_FN {
            attributes.push(TypeAttribute::new(TO_SOURCE_FN, meta.clone()))
        }
        if meta.path() == SOURCE_GENERICS {
            attributes.push(TypeAttribute::new(SOURCE_GENERICS, meta.clone()))
        }
    }

    Ok(attributes)
}

pub fn parse_attr_meta_into_fields(inner_meta: &Punctuated<Meta, Token![,]>) -> syn::Result<Vec<TypeAttribute>> {
    let mut attributes = vec![];

    for meta in inner_meta.into_iter() {
        if meta.path() == FIELD_FN {
            attributes.push(TypeAttribute::new(FIELD_FN, meta.clone()))
        }
    }

    Ok(attributes)
}
