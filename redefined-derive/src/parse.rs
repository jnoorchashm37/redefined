use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, parse_quote, Data, DataEnum, DataStruct, DeriveInput, Expr, GenericParam, Generics, Ident, LitStr, TypeParam};

use crate::{
    attributes::symbol::{FROM_SOURCE_FN, TO_SOURCE_FN, TRANSMUTE},
    outer::OuterContainer,
    r#enum::EnumContainer,
    r#struct::StructContainer,
};

pub fn expand_derive_redefined(input: &DeriveInput) -> syn::Result<TokenStream> {
    let outer = OuterContainer::parse(input.ident.clone(), input.generics.clone(), &input.attrs)?;

    let inner = match &input.data {
        Data::Struct(data_struct) => TraitContainer::from_struct(data_struct, &outer.source_type),
        Data::Enum(data_enum) => TraitContainer::from_enum(data_enum, &outer.source_type, &outer.target_type),
        _ => return Err(syn::Error::new_spanned(input, "Expected an enum or struct")),
    }?;

    let container = Container::new(outer, inner);

    container.parse_container_to_token_stream()
}

pub struct Container {
    pub outer: OuterContainer,
    pub inner: TraitContainer,
}

impl Container {
    pub fn new(outer: OuterContainer, inner: TraitContainer) -> Self {
        Self { outer, inner }
    }
}

impl Container {
    pub fn parse_container_to_token_stream(&self) -> syn::Result<TokenStream> {
        let source_type = &self.outer.source_type;
        let target_type = &self.outer.target_type;

        let is_transmute = self
            .outer
            .container_attrs
            .iter()
            .any(|attr| attr.symbol == TRANSMUTE);

        let mut from_source_tokens = if let Some(func) = self
            .outer
            .container_attrs
            .iter()
            .find(|attr| attr.symbol == FROM_SOURCE_FN)
        {
            let new_func_name = parse_str_expr_into_lit_expr(&func.meta.require_name_value()?.value)?;
            new_func_name.to_token_stream()
        } else {
            let inner_from_source_tokens = &self.inner.from_source;
            quote! { #inner_from_source_tokens }
        };

        let mut to_source_tokens = if let Some(func) = self
            .outer
            .container_attrs
            .iter()
            .find(|attr| attr.symbol == TO_SOURCE_FN)
        {
            let new_func_name = parse_str_expr_into_lit_expr(&func.meta.require_name_value()?.value)?;
            new_func_name.to_token_stream()
        } else {
            let inner_to_source_tokens = &self.inner.to_source;
            quote! { #inner_to_source_tokens }
        };

        let (impl_generics, ty_generics, _) = self.outer.target_generics.split_for_impl();

        if is_transmute {
            from_source_tokens = quote! { unsafe { std::mem::transmute::<#source_type #ty_generics, Self>()} };
            to_source_tokens = quote! { unsafe { std::mem::transmute::<Self, #source_type #ty_generics>()} };
        }

        let gen = if self.outer.source_generics.is_empty() {
            quote! {
                 impl #impl_generics RedefinedConvert<#source_type #ty_generics> for #target_type #ty_generics
                     {
                         fn from_source(src: #source_type #ty_generics) -> Self {
                                #from_source_tokens
                         }

                         fn to_source(self) -> #source_type #ty_generics {
                                #to_source_tokens
                         }
                     }


            }
        } else {
            let (_, target_type_generics, _) = self.outer.target_generics.split_for_impl();
            let (generics, source_generics, where_clause) = self.build_generics_with_where_clause()?;
            let (combined_impl_generics, ..) = generics.split_for_impl();

            if is_transmute {
                from_source_tokens = quote! { unsafe { std::mem::transmute::<#source_type <#(#source_generics,)*>, Self>()} };
                to_source_tokens = quote! { unsafe { std::mem::transmute::<Self, #source_type <#(#source_generics,)*>>()} };
            }

            quote! {
                 impl #combined_impl_generics RedefinedConvert<#source_type <#(#source_generics,)*>> for #target_type #target_type_generics
                 #where_clause
                     {
                         fn from_source(src: #source_type <#(#source_generics,)*>) -> Self {
                                #from_source_tokens
                         }

                         fn to_source(self) -> #source_type <#(#source_generics,)*> {
                                #to_source_tokens
                         }
                     }
            }
        };

        Ok(gen)
    }

    pub fn build_generics_with_where_clause(&self) -> syn::Result<(Generics, Vec<GenericParam>, TokenStream)> {
        let ext_source_generic_params = self
            .outer
            .source_generics
            .iter()
            .map(|source_param| {
                syn::GenericParam::Type(TypeParam {
                    attrs:       Vec::new(),
                    ident:       source_param.clone(),
                    colon_token: None,
                    bounds:      Default::default(),
                    eq_token:    None,
                    default:     None,
                })
            })
            .collect::<Vec<_>>();

        let zip_generics = ext_source_generic_params
            .clone()
            .into_iter()
            .zip(self.outer.target_generics.params.clone())
            .map(|(source, target)| {
                let (s, t) = (source.to_token_stream(), target.to_token_stream());
                quote! { #t: RedefinedConvert<#s>, }
            })
            .collect::<Vec<_>>();

        let where_clause = quote! {
           where
               #(#zip_generics)*
        };

        let mut target_generics = self.outer.target_generics.clone();
        target_generics
            .params
            .extend(ext_source_generic_params.clone());

        Ok((target_generics, ext_source_generic_params, where_clause))
    }
}

pub struct TraitContainer {
    pub from_source: TokenStream,
    pub to_source:   TokenStream,
}

impl TraitContainer {
    pub fn from_struct(data_struct: &DataStruct, source_type: &Ident) -> syn::Result<Self> {
        let container = StructContainer::parse_new(data_struct)?;

        Ok(Self { from_source: container.from_source_tokens()?, to_source: container.to_source_tokens(source_type)? })
    }

    pub fn from_enum(enum_struct: &DataEnum, source_type: &Ident, target_type: &Ident) -> syn::Result<Self> {
        let container = EnumContainer::parse_new(enum_struct)?;

        Ok(Self {
            from_source: container.from_source_tokens(source_type, target_type)?,
            to_source:   container.to_source_tokens(source_type, target_type)?,
        })
    }
}

pub fn parse_str_expr_into_lit_expr(expr: &Expr) -> syn::Result<Expr> {
    let str_expr = &expr;
    let lit_str_expr: LitStr = parse_quote!(#str_expr);
    lit_str_expr.parse::<Expr>()
}
