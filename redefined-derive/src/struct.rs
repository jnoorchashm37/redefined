use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Attribute, DataStruct, DeriveInput, Expr, Fields, Ident, LitStr, TypeGenerics, WhereClause};

/*
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
*/

pub fn generate_struct_impl(
    target_struct: &Ident,
    source_struct: &TokenStream,
    data_struct: &DataStruct,
    input: &DeriveInput,
    new_func: &Option<LitStr>,
) -> syn::Result<TokenStream> {
    // Extract fields from the data structure
    let fields = match &data_struct.fields {
        Fields::Named(fields_named) => &fields_named.named,
        _ => return Err(syn::Error::new_spanned(input, "Expected a struct with named fields")),
    };

    let matched_fields = fields.iter().map(|f| f.ident.as_ref()).collect::<Vec<_>>();

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
                        func_name = Some(meta.value()?.parse::<LitStr>()?);
                    }
                    Ok(())
                })
                .expect("Expected function name");

            let fn_name = func_name.as_ref().unwrap().parse::<Expr>().unwrap();
            quote! {
                #ident: RedefinedConvert::from_source(src.#fn_name()),
            }
        } else {
            quote! {
                #ident: RedefinedConvert::from_source(src.#ident),
            }
        }
    });

    let to_source_token = if let Some(litstr) = new_func {
        let str_to_ident = litstr.parse::<Expr>().unwrap();
        quote! { #str_to_ident }
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
                return #to_source_token;
            }
        }
    };

    Ok(gen)
}

pub fn generate_struct_impl_with_generics(
    target_struct: &Ident,
    source_struct: &TokenStream,
    target_generics: &TypeGenerics,
    where_clause: Option<&WhereClause>,
    //source_generics: &Option<Vec<Ident>>,
    data_struct: &DataStruct,
    input: &DeriveInput,
    new_func: &Option<LitStr>,
) -> syn::Result<TokenStream> {
    // Extract fields from the data structure
    let fields = match &data_struct.fields {
        Fields::Named(fields_named) => &fields_named.named,
        _ => return Err(syn::Error::new_spanned(input, "Expected a struct with named fields")),
    };

    let matched_fields = fields.iter().map(|f| f.ident.as_ref()).collect::<Vec<_>>();

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
                        func_name = Some(meta.value()?.parse::<LitStr>()?);
                    }
                    Ok(())
                })
                .expect("Expected function name");

            let fn_name = func_name.as_ref().unwrap().parse::<Expr>().unwrap();
            quote! {
                #ident: RedefinedConvert::from_source(src.#fn_name()),
            }
        } else {
            quote! {
                #ident: RedefinedConvert::from_source(src.#ident),
            }
        }
    });

    let to_source_token = if let Some(litstr) = new_func {
        let str_to_ident = litstr.parse::<Expr>().unwrap();
        quote! { #str_to_ident }
    } else {
        quote! {
            #source_struct {
                #(
                    #matched_fields: self.#matched_fields.to_source(),
                )*
            }
        }
    };

    let gen = if let Some(wc) = where_clause {
        //let required_trait_bounds: Vec<&str> = vec!["RedefinedConvert<>", "std::fmt::Debug"];
        
        quote! {
        impl #target_generics RedefinedConvert<#source_struct #target_generics> for #target_struct #target_generics 
        #wc
        {
            fn from_source(src: #source_struct #target_generics) -> Self {
                #target_struct {
                    #(
                        #field_mappings
                    )*
                }
            }

            fn to_source(self) -> #source_struct #target_generics {
                return #to_source_token;
            }
        }
    }} else {
        quote! {
            impl #target_generics RedefinedConvert<#source_struct #target_generics> for #target_struct #target_generics 
            {
                fn from_source(src: #source_struct #target_generics) -> Self {
                    #target_struct {
                        #(
                            #field_mappings
                        )*
                    }
                }
    
                fn to_source(self) -> #source_struct #target_generics {
                    return #to_source_token;
                }
            }
        }
    };

    Ok(gen)
}



fn add_trait_bounds_to_existing_where_clause_ts(
    where_clause: &Option<syn::WhereClause>,
    traits: &Vec<&str>,
  ) -> proc_macro2::TokenStream {
    // Must parse the `traits.join("+")` string into a [syn::Type].
    let joined_traits: syn::Type =
      syn::parse_str::<syn::Type>(&traits.join(" + ")).unwrap();
  
    let where_clause_ts = match where_clause {
      Some(where_clause) => {
        let where_predicate_punctuated_list = &where_clause.predicates;
  
        let modified_where_predicates_ts = where_predicate_punctuated_list
          .iter()
          .map(
            |where_predicate| match where_predicate {
              syn::WherePredicate::Type(_) => {
                quote! { #where_predicate + #joined_traits }
              }
              _ => quote! {},
            },
          )
          .collect::<Vec<_>>();
  
        quote! { where #(#modified_where_predicates_ts),* }
      }
      None => {
        quote! {}
      }
    };
  
    return where_clause_ts;
  }