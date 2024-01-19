use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, punctuated::Punctuated, DataStruct, Field, Fields, Ident, Meta, Token};

use crate::{
    attributes::{
        symbol::FIELD_FN,
        type_attr::{parse_attr_meta_into_fields, TypeAttribute},
    },
    parse::parse_str_expr_into_lit_expr,
};

pub struct StructContainer {
    pub fields: Vec<StructField>,
}

impl StructContainer {
    pub fn parse_new(data_struct: &DataStruct) -> syn::Result<Self> {
        let fields = match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => return Err(syn::Error::new_spanned(&data_struct.fields, "Expected a struct with named fields")),
        };

        let struct_fields = fields
            .iter()
            .flat_map(|field| {
                field.ident.as_ref().map(|idt| {
                    let mut this = StructField::new(idt.clone(), field.clone());
                    this.parse_attributes_for_field()?;
                    Ok(this)
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(Self { fields: struct_fields })
    }

    pub fn from_source_tokens(&self) -> syn::Result<TokenStream> {
        let from_source_tokens = self
            .fields
            .iter()
            .map(|field| field.from_source_tokens())
            .collect::<syn::Result<Vec<_>>>()?;

        let gen = quote! {
            Self {
                #(#from_source_tokens)*
            }
        };

        Ok(gen)
    }

    pub fn to_source_tokens(&self, source_type: &Ident) -> syn::Result<TokenStream> {
        let tokens = self
            .fields
            .iter()
            .map(|field| field.to_source_tokens())
            .collect::<syn::Result<Vec<_>>>()?;

        let gen = quote! {
            #source_type {
                #(#tokens)*
            }
        };

        Ok(gen)
    }
}

impl From<Vec<StructField>> for StructContainer {
    fn from(value: Vec<StructField>) -> Self {
        Self { fields: value }
    }
}

pub struct StructField {
    pub ident:       Ident,
    pub field:       Field,
    pub field_attrs: Vec<TypeAttribute>,
}

impl StructField {
    pub fn new(ident: Ident, field: Field) -> Self {
        Self { ident, field, field_attrs: Vec::new() }
    }

    pub fn parse_attributes_for_field(&mut self) -> syn::Result<()> {
        if let Some(attr) = self.field.attrs.first() {
            if attr.path().is_ident("redefined_attr") {
                let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                self.field_attrs = parse_attr_meta_into_fields(&nested)?;
            }
        }
        Ok(())
    }

    pub fn from_source_tokens(&self) -> syn::Result<TokenStream> {
        let fields_attrs = &self.field_attrs;
        let ident = &self.ident;

        let gen = if FIELD_FN.contained_in(fields_attrs) {
            let attr = FIELD_FN
                .find_type_attr(fields_attrs)
                .ok_or(syn::Error::new_spanned(&self.ident, "FIELD FN ERROR"))?;
            let name_val = attr
                .meta
                .require_name_value()
                .map_err(|_| syn::Error::new_spanned(&attr.meta, "#[redefined_attr(func = \"..\")] must be a Meta::NameValue"))?;
            let func_name = parse_str_expr_into_lit_expr(&name_val.value)?;
            quote! { #ident: RedefinedConvert::from_source(src.#func_name()), }
        } else if fields_attrs.is_empty() {
            quote! { #ident: RedefinedConvert::from_source(src.#ident),}
        } else {
            unreachable!("cannot reach");
        };

        Ok(gen)
    }

    pub fn to_source_tokens(&self) -> syn::Result<TokenStream> {
        let matched_field = &self.ident;

        let gen = quote! { #matched_field: self.#matched_field.to_source(), };

        Ok(gen)
    }
}
