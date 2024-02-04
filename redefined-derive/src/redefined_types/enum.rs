use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{self, parse::Parse, DataEnum, Fields, FieldsNamed, FieldsUnnamed, Ident, Variant};

use crate::attributes::{type_attr::TypeAttribute, ContainerAttributes};

pub struct EnumContainer {
    pub fields: Vec<EnumField>,
}

impl EnumContainer {
    pub fn parse_new(data_enum: &DataEnum) -> syn::Result<Self> {
        let enum_fields = data_enum
            .variants
            .iter()
            .map(|variant| {
                let mut this = EnumField::new(variant.ident.clone(), variant.clone());
                this.parse_attributes_for_field()?;
                Ok(this)
            })
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(Self { fields: enum_fields })
    }

    pub fn from_source_tokens(&self, source_type: &Ident, target_type: &Ident) -> syn::Result<TokenStream> {
        let from_source_tokens = self
            .fields
            .iter()
            .map(|field| field.from_source_tokens(source_type, target_type))
            .collect::<syn::Result<Vec<_>>>()?;

        let gen = quote! {
           match src {
               #(#from_source_tokens)*
        } };

        Ok(gen)
    }

    pub fn to_source_tokens(&self, source_type: &Ident, target_type: &Ident) -> syn::Result<TokenStream> {
        let tokens = self
            .fields
            .iter()
            .map(|field| field.to_source_tokens(source_type, target_type))
            .collect::<syn::Result<Vec<_>>>()?;

        let gen = quote! {
           match self {
               #(#tokens)*
        } };

        Ok(gen)
    }
}

pub struct EnumField {
    pub ident:       Ident,
    pub variant:     Variant,
    pub field_attrs: Vec<TypeAttribute>,
}

impl EnumField {
    pub fn new(ident: Ident, variant: Variant) -> Self {
        Self { ident, variant, field_attrs: Vec::new() }
    }

    pub fn parse_attributes_for_field(&mut self) -> syn::Result<()> {
        let mut attrs = Vec::new();
        for attr in &self.variant.attrs {
            if attr.path().is_ident("redefined") {
                attrs.extend(attr.parse_args_with(ContainerAttributes::parse)?.0);
            }
        }
        self.field_attrs = attrs;

        Ok(())
    }

    pub fn from_source_tokens(&self, source_type: &Ident, target_type: &Ident) -> syn::Result<TokenStream> {
        let variant_name = &self.variant.ident;
        let gen = match &self.variant.fields {
            Fields::Unit => quote! {
                #source_type::#variant_name =>  #target_type::#variant_name,
            },
            Fields::Named(FieldsNamed { named, .. }) => {
                let field_names: Vec<_> = named
                    .into_iter()
                    .map(|f| f.ident.as_ref().unwrap().clone())
                    .collect();
                let cloned_field_names = field_names.clone();

                if !field_names.is_empty() {
                    let field_mappings = cloned_field_names.into_iter().map(|field_name| {
                        quote! { #field_name: redefined::RedefinedConvert::from_source(#field_name) }
                    });

                    quote! {
                        #source_type::#variant_name { #( #field_names ),* } => #target_type::#variant_name { #( #field_mappings ),* },
                    }
                } else {
                    TokenStream::new()
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let field_vars: Vec<_> = (0..unnamed.len())
                    .map(|i| syn::Ident::new(&format!("x{}", i), Span::call_site()))
                    .collect();

                let destructuring = quote! { (#(#field_vars),*) };

                let construction = quote! {
                    #target_type::#variant_name(#(redefined::RedefinedConvert::from_source(#field_vars)),*)
                };

                let unamed_gen = quote! {
                    #source_type::#variant_name #destructuring => #construction,
                };

                unamed_gen
            }
        };

        Ok(gen)
    }

    pub fn to_source_tokens(&self, source_type: &Ident, target_type: &Ident) -> syn::Result<TokenStream> {
        let variant_name = &self.variant.ident;
        let gen = match &self.variant.fields {
            Fields::Unit => quote! {
                #target_type::#variant_name =>  #source_type::#variant_name,
            },
            Fields::Named(FieldsNamed { named, .. }) => {
                let field_names: Vec<_> = named
                    .into_iter()
                    .map(|f| f.ident.as_ref().unwrap().clone())
                    .collect();
                let cloned_field_names = field_names.clone();

                if !field_names.is_empty() {
                    let field_mappings = cloned_field_names.into_iter().map(|field_name| {
                        quote! { #field_name: redefined::RedefinedConvert::to_source(#field_name) }
                    });

                    quote! {
                        #target_type::#variant_name { #( #field_names ),* } => #source_type::#variant_name { #( #field_mappings ),* },
                    }
                } else {
                    TokenStream::new()
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let field_vars: Vec<_> = (0..unnamed.len())
                    .map(|i| syn::Ident::new(&format!("x{}", i), Span::call_site()))
                    .collect();

                let destructuring = quote! { (#(#field_vars),*) };

                let construction = quote! {
                    #source_type::#variant_name(#(redefined::RedefinedConvert::to_source(#field_vars)),*)
                };

                let unamed_gen = quote! {
                    #target_type::#variant_name #destructuring => #construction,
                };

                unamed_gen
            }
        };

        Ok(gen)
    }
}
