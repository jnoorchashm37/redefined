use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, parse::Parse, spanned::Spanned, Attribute, DataStruct, Field, Fields, Generics, Ident, Type, Visibility};

use crate::attributes::{symbol::REDEFINED_FIELD, ContainerAttributes};

pub fn parse_new_struct(
    data_struct: &DataStruct,
    struct_name: &Ident,
    new_struct_name: &Ident,
    generics: &Generics,
    visibility: &Visibility,
    attributes: &[Attribute],
) -> syn::Result<TokenStream> {
    let fields = match &data_struct.fields {
        Fields::Named(fields_named) => &fields_named.named,
        Fields::Unnamed(fields_unnamed) => &fields_unnamed.unnamed,
        _ => return Err(syn::Error::new_spanned(&data_struct.fields, "Expected a struct with named fields")),
    };

    let struct_fields = fields
        .iter()
        .map(|field| parse_field(field.clone()))
        .collect::<syn::Result<Vec<_>>>()?;

    // semi_token
    let tokens = if let Some(semi_token) = data_struct.semi_token {
        quote! {
            #[Derive(Redefined)]
            #[redefined(#struct_name)]
            #(#attributes)*
            #visibility struct #new_struct_name #generics (#(#struct_fields),*)#semi_token
        }
    } else {
        quote! {
            #[derive(Redefined)]
            #[redefined(#struct_name)]
            #(#attributes)*
            #visibility struct #new_struct_name #generics {
                #(#struct_fields),*
            }
        }
    };

    Ok(tokens)
}

pub fn parse_field(field: Field) -> syn::Result<TokenStream> {
    let ident = field.ident;
    let _mutability = field.mutability;
    let colon_token = field.colon_token;
    let vis = field.vis;
    let mut ty = field.ty;
    let mut copied_field_attrs = Vec::new();
    let mut field_attrs = Vec::new();

    for attr in field.attrs {
        if attr.path().is_ident("redefined") {
            field_attrs = attr.parse_args_with(ContainerAttributes::parse)?.0;
        } else {
            copied_field_attrs.push(attr)
        }
    }

    if let Some(attr) = field_attrs
        .iter()
        .find(|s| s.symbol == REDEFINED_FIELD)
        .cloned()
    {
        let attr_type = attr.list_idents.unwrap();
        let new_type_name = if attr_type.is_empty() {
            None
        } else if attr_type.len() == 1 {
            Some(attr_type[0].clone())
        } else {
            panic!("#[redefined(field(...)) must either have 0 (default redefined type) or 1 (custom redefined type) in 'field(...)'")
        };

        ty = parse_type_to_redefined(ty, new_type_name);
    }

    let tokens = quote! {
        #(#copied_field_attrs)*
        #vis #ident #colon_token #ty
    };

    Ok(tokens)
}

pub fn parse_type_to_redefined(src_type: Type, new_type_name: Option<Ident>) -> Type {
    /*
    match &src_type {
        Type::BareFn(_) => unimplemented!(),
        Type::Group(_) => unimplemented!(),
        Type::ImplTrait(_) => unimplemented!(),
        Type::Infer(_) => unimplemented!(),
        Type::Macro(_) => unimplemented!(),
        Type::Never(_) => unimplemented!(),
        Type::Paren(_) => unimplemented!(),
        Type::Path(_) => unimplemented!(),
        Type::Ptr(_) => unimplemented!(),

        Type::TraitObject(t) => unimplemented!(),
        Type::Tuple(t) => unimplemented!(), // add this 1
        Type::Verbatim(_) => unimplemented!(),
    }
    */

    match src_type {
        Type::Array(a) => {
            let mut array = a.clone();
            let new_type = parse_type_to_redefined(*a.elem, new_type_name);
            array.elem = Box::new(new_type);
            Type::Array(array)
        }
        Type::Reference(r) => {
            let mut refer = r.clone();
            let new_type = parse_type_to_redefined(*r.elem, new_type_name);
            refer.elem = Box::new(new_type);
            Type::Reference(refer)
        }
        Type::Slice(s) => {
            let mut slice = s.clone();
            let new_type = parse_type_to_redefined(*s.elem, new_type_name);
            slice.elem = Box::new(new_type);
            Type::Slice(slice)
        }
        Type::Path(p) => {
            let mut path = p.clone();
            let seg = path.path.segments.first_mut();
            seg.unwrap().ident = new_type_name.unwrap_or(Ident::new(&format!("{}Redefined", seg.as_ref().unwrap().ident), p.span()));

            Type::Path(path)
        }
        _ => panic!("FIELD IS OF TYPE: {}", src_type.to_token_stream()),
    }
}
