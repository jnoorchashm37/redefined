use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Attribute, DataStruct, DeriveInput, Expr, Fields, Ident, LitStr};

pub fn generate_struct_impl(
    target_struct: &Ident,
    source_struct: &Ident,
    data_struct: &DataStruct,
    input: &DeriveInput,
    new_func: &Option<LitStr>,
) -> syn::Result<TokenStream> {
    let fields = match &data_struct.fields {
        Fields::Named(fields_named) => &fields_named.named,
        _ => {
            return Err(syn::Error::new_spanned(input, "Expected a struct with named fields"));
        }
    };

    let matched_fields = match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|f| &f.ident)
            .collect::<Vec<_>>(),
        _ => return Err(syn::Error::new_spanned(input, "Expected a struct with named fields")),
    };

    let field_mappings = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();

        let func_attr = f
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("redefined"));

        if let Some(func_attr) = func_attr {
            let mut func_name = None;
            func_attr
                .parse_nested_meta(|meta| {
                    if meta.path.is_ident("func") {
                        let args: LitStr = meta.value()?.parse()?;
                        func_name = Some(args)
                    }
                    Ok(())
                })
                .unwrap();

            let func_name = func_name.unwrap().parse::<Expr>().unwrap();
            quote! {
                #ident:  RedefinedConvert::from_source(src.#func_name()),
            }
        } else {
            quote! {
                #ident:  RedefinedConvert::from_source(src.#ident),
            }
        }
    });

    let to_source_token = if let Some(litstr) = new_func {
        let str_to_ident = litstr.parse::<Expr>()?;
        quote! {
              #str_to_ident
        }
    } else {
        quote! {
            #source_struct {
                #(
                    #matched_fields: self.#matched_fields.to_source(),
                )*
            }
        }
    };

    let gen = quote! {
        impl RedefinedConvert<#source_struct> for #target_struct {
            fn from_source(src: #source_struct) -> Self {
                #target_struct {
                    #(
                        #field_mappings
                    )*
                }
            }

            fn to_source(self) -> #source_struct {
                return #to_source_token
            }
        }
    };

    Ok(gen)
}

pub fn extract_new_source_func(attrs: &[Attribute]) -> syn::Result<Option<(Ident, LitStr)>> {
    let mut new_src_func = None;
    for attr in attrs {
        if attr.path().is_ident("redefined") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("to_source") {
                    let idt = meta.path.get_ident().unwrap();
                    let args: LitStr = meta.value()?.parse()?;

                    new_src_func = Some((idt.clone(), args.clone()));
                    return Ok(());
                }

                Ok(())
            })?;
        }
    }
    Ok(new_src_func)
}

/*
                    let args = match &attr.meta {
                        Meta::NameValue(nv) => &nv.value,
                        _ => unreachable!("unreachable"),
                    };
*/

// let litstr: LitStr = meta.value()?.parse()?;
