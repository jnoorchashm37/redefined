use proc_macro2::TokenStream;
use syn::{self, Attribute, Data, DeriveInput, Error, Ident, LitStr};

use crate::{r#enum::generate_enum_impl, r#struct::generate_struct_impl};

pub fn expand_derive_redefined(input: &mut DeriveInput) -> syn::Result<TokenStream> {
    let target_struct = &input.ident;
    let (source_struct, new_source_func) = &extract_source_type(&input.attrs)?;

    let gen = match &input.data {
        Data::Struct(data_struct) => {
            generate_struct_impl(target_struct, source_struct, data_struct, input, new_source_func)?
        }
        Data::Enum(data_enum) => generate_enum_impl(data_enum, target_struct, &source_struct)?,
        _ => return Err(syn::Error::new_spanned(input, "Expected an enum")),
    };

    Ok(gen)
}

fn extract_source_type(attrs: &[Attribute]) -> syn::Result<(Ident, Option<LitStr>)> {
    let mut res = None;
    let mut new_src_func = None;
    for attr in attrs {
        if attr.path().is_ident("redefined") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("to_source") {
                    let args: LitStr = meta.value()?.parse()?;

                    new_src_func = Some(args.clone());
                } else {
                    res = Some(
                        meta.path
                            .get_ident()
                            .cloned()
                            .ok_or(syn::Error::new_spanned(
                                attr,
                                format!("Error parsing 'redefined' attribute"),
                            ))?,
                    );
                }

                Ok(())
            })?;
        }
    }
    if let Some(r) = res {
        Ok((r, new_src_func))
    } else {
        Err(Error::new_spanned(
            attrs.first().expect("Expected at least one attribute"),
            "Attribute 'redefined' not found or not correctly formatted",
        ))
    }
}
