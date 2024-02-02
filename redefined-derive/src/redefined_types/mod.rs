pub mod r#enum;
pub mod r#struct;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, spanned::Spanned, Data, DataEnum, DataStruct, GenericParam, Generics, Ident, TypeGenerics};

#[cfg(feature = "unsafe")]
use crate::attributes::symbol::TRANSMUTE;
use crate::{
    attributes::symbol::{FROM_SOURCE_FN, TO_SOURCE_FN},
    outer::OuterContainer,
    redefined_types::{r#enum::EnumContainer, r#struct::StructContainer},
};

pub struct RedefinedContainer<'a> {
    source_type:            Ident,
    target_type:            Ident,
    source_generics_tokens: TokenStream,
    target_generics:        TypeGenerics<'a>,
    impl_generics_tokens:   TokenStream,
    where_clause:           Option<TokenStream>,
    to_source_tokens:       TokenStream,
    from_source_tokens:     TokenStream,
    self_impl:              TokenStream,
}

impl<'a> RedefinedContainer<'a> {
    pub fn parse_sub_containers(outer: OuterContainer, input_data: &'a Data, input_generics: &'a Generics) -> syn::Result<Self> {
        let source_type = outer.source_type.clone().unwrap();

        let (mut to_source_tokens, mut from_source_tokens) = if outer.should_parse_fields() {
            let trait_container = match &input_data {
                Data::Struct(data_struct) => TraitContainer::from_struct(data_struct, &source_type),
                Data::Enum(data_enum) => TraitContainer::from_enum(data_enum, &source_type, &outer.target_type),
                _ => return Err(syn::Error::new_spanned(source_type, "Expected an enum or struct")),
            }?;
            (trait_container.to_source, trait_container.from_source)
        } else {
            (Default::default(), Default::default())
        };

        if let Some(attr) = outer.get_symbol(TO_SOURCE_FN) {
            to_source_tokens = attr.nv_tokens.unwrap().to_token_stream();
        }

        if let Some(attr) = outer.get_symbol(FROM_SOURCE_FN) {
            from_source_tokens = attr.nv_tokens.unwrap().to_token_stream();
        }

        let (target_generics, source_generics_tokens, impl_generics_tokens, where_clause) = if input_generics.type_params().count() == 0 {
            let (impl_generics, ty_generics, _) = input_generics.split_for_impl();
            (ty_generics.clone(), quote!(#ty_generics), impl_generics.to_token_stream(), None)
        } else {
            let (_, ty_generics, _) = input_generics.split_for_impl();
            let (modded_generics, source_generics, where_clause) = build_generics_with_where_clause(input_generics.clone())?;
            let (combined_impl_generics, ..) = modded_generics.split_for_impl();
            (ty_generics, quote!(<#(#source_generics,)*>), combined_impl_generics.to_token_stream(), Some(where_clause))
        };

        #[cfg(feature = "unsafe")]
        if outer.get_symbol(TRANSMUTE).is_some() {
            from_source_tokens = quote! {
               let s = unsafe { std::mem::transmute_copy::<#source_type #source_generics_tokens, Self>(&src) };
               std::mem::forget(src);
               s
            };
            to_source_tokens = quote! {
                let s = unsafe { std::mem::transmute_copy::<Self, #source_type #source_generics_tokens>(&self) };
                std::mem::forget(self);
                s
            };
        }

        let self_impl = make_self_impl(&outer.target_type, input_generics);

        Ok(Self {
            source_type,
            target_type: outer.target_type,
            source_generics_tokens,
            target_generics,
            impl_generics_tokens,
            where_clause,
            to_source_tokens,
            from_source_tokens,
            self_impl,
        })
    }

    pub fn finalize(&self) -> TokenStream {
        let Self {
            source_type,
            target_type,
            source_generics_tokens,
            target_generics,
            impl_generics_tokens,
            where_clause,
            to_source_tokens,
            from_source_tokens,
            self_impl,
        } = self;

        quote! {



             impl #impl_generics_tokens RedefinedConvert<#source_type #source_generics_tokens> for #target_type #target_generics
             #where_clause
                 {
                     fn from_source(src: #source_type #source_generics_tokens) -> Self {
                            #from_source_tokens
                     }

                     fn to_source(self) -> #source_type #source_generics_tokens {
                            #to_source_tokens
                     }
                 }

            impl #impl_generics_tokens From<#source_type #source_generics_tokens> for #target_type #target_generics
            #where_clause
                {
                    fn from(src: #source_type #source_generics_tokens) -> Self {
                        Self::from_source(src)
                    }
                }

            impl #impl_generics_tokens Into<#source_type #source_generics_tokens> for #target_type #target_generics
            #where_clause
                {
                    fn into(self) -> #source_type #source_generics_tokens {
                        Self::to_source(self)
                    }
                }

        }
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

pub fn build_generics_with_where_clause(ty_generics: Generics) -> syn::Result<(Generics, Vec<GenericParam>, TokenStream)> {
    let source_generics = ty_generics
        .params
        .iter()
        .map(|target_generic| {
            let mut source_generic = target_generic.clone();
            if let GenericParam::Type(s) = &mut source_generic {
                s.ident = Ident::new(&format!("{}R", s.ident), target_generic.span())
            }
            source_generic
        })
        .collect::<Vec<_>>();

    // + From<#s> + Into<#s>,
    let zip_generics = source_generics
        .iter()
        .zip(ty_generics.params.clone())
        .map(|(source, target)| {
            let (s, t) = (source.to_token_stream(), target.to_token_stream());
            quote! { #t: RedefinedConvert<#s>,  }
        })
        .collect::<Vec<_>>();

    let where_clause = quote! {
       where
           #(#zip_generics)*
    };

    let mut target_generics = ty_generics.clone();
    target_generics.params.extend(source_generics.clone());

    Ok((target_generics, source_generics, where_clause))
}

fn make_self_impl(target_type: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics RedefinedConvert<#target_type #type_generics> for #target_type #type_generics
        #where_clause
        {
            fn from_source(src: #target_type #type_generics) -> Self {
                src
            }

            fn to_source(self) -> #target_type #type_generics {
                self
            }
        }

    }
}
