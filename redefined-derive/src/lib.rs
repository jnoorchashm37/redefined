#![allow(clippy::wrong_self_convention)]
#![allow(non_upper_case_globals)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive;

mod redefined_types;

mod attributes;

mod outer;

mod new_types;

#[cfg(feature = "remote")]
mod remote;

/*

DERIVE MACRO:
------------

Container Source Attribute WITH: #[redefined(<TYPE IDENTIFIER>)]
    - Only used to specify the source struct (i.e. the struct converting from)

Example:
{
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined(BasicStruct)]
    pub struct BasicStructA {
        pub val1: u64,
        pub val2: f64,
        pub val3: String,
    }
}


---
Container Source Attribute WITHOUT: #[redefined(<TYPE IDENTIFIER>)]
    - Omitted when the current type is the source type to a new type
    - The new type will be created with the same fields and attributes (except redefined attributes)
    - The new type's name will be the same as this type's name with a concatinated 'Redefined'
    - The new type implements from/into this type via transmute
    - TODO: other methods besides transmute


Example:
{
    #[derive(Debug, Clone, PartialEq, Redefined)]
    pub struct GenericConstantStructA<const XVAL: usize> {
        pub p: u64,
        pub d: [i128; XVAL],
    }

    /// GenericConstantStructARedefined is created
}


---
Container Attributes: #[redefined_attr(...)]

    1) to_source = "..."
        - specifies a function to call when converting this type back into it's source type
        - (i.e. for when fields are private)

    2) from_source = "..."
        - specifies a function to call when converting the source type into this type
        - NOTE WHEN USING THIS: the source type is a variable called 'src' (see first example)
        - (i.e. for when fields are private)

    3) transmute
        - converts between the types using an unsafe transmute
        - conflicts with [1] and/or [2]

    4) derive(...)
        - specifies #[derive(...)] values for the new type

Examples:
to_source + from_source
{
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined(PrivateFieldStruct)]
    #[redefined_attr(to_source = "PrivateFieldStruct::new(self.p, self.d, self.vals)", from_source = "ToFromSourceFieldStructB::new(src)")]
    pub struct ToFromSourceFieldStructB {
        pub p:    u64,
        pub d:    u64,
        pub vals: Vec<String>,
    }
}

transmute
{
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined(BasicStruct)]
    #[redefined_attr(transmute)]
    pub struct BasicStructA {
        pub val1: u64,
        pub val2: f64,
        pub val3: String,
    }
}

derive
{
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined_attr(derive(Debug, Clone, PartialEq, Default))]
    pub struct BasicStructA {
        pub val1: u64,
        pub val2: f64,
        pub val3: String,
    }
}


---
Field Attributes: #[redefined(...)]
    - These attributes go on over fields of the type

    1) func = "..."
        - specifies a function to call when getting a field from the source struct
        - NOTE WHEN USING THIS: the source type is a variable called 'src' (same as in 'from_source = '...'' container attribute above)

    2) field(...)
        - specifies a type to use when converting between target and source types
        - syntax: comma seperated list of '(source type, target type)'
        - NOTE: only used when the container attribute 'redefined(...)' is omitted

        Sub-attributes
        a) field((.., default))
            - 'default' can be used in place of the target type, when the target type's identifier is the same as the source type's concatenated with 'Redefined'

Examples:

func:
{
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined(PrivateFieldStruct)]
    #[redefined_attr(to_source = "PrivateFieldStruct::new(self.p, self.d, self.vals)")]
    pub struct NonPubFieldStructB {
        #[redefined(func = "src.get_p()")]
        pub p:    u64,
        pub d:    u64,
        pub vals: Vec<String>,
    }
}

field + field((.., default)):
{
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined_attr(derive(Debug, Clone, PartialEq))]
    pub struct ComplexStructAA<'a, 'b> {
        pub n:       i128,
        #[redefined(field((GenericLifetimeStructA, default)))]
        pub inner_a: GenericLifetimeStructA<'a, 'b>,
        #[redefined(field((BasicStructA, BasicStructARedefined)))]
        pub inner_b: Vec<BasicStructA>,
    }
}




*/

#[proc_macro_derive(Redefined, attributes(redefined, redefined_attr))]
pub fn derive_redefined(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::expand_derive_redefined(&input, false)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/*
REMOTE MACRO:
------------


Macro Input
    1 (Optional): #[derives(...)] for derives on new type
    2: Identifier of the remote type
    3: the crate of the remote type as it appears in Cargo.toml/lock

Macro Output
    - a new type with the same fields as the old type
    - it's name is the same expect with a concatenated 'Redefined'
    - TODO: deal with nested types


Functions for struct/enum 'Abc':
    1) From<Abc> for AbcRedefined
    2) Into<Abc> for AbcRedefined


Note:
    - TODO: Currently only uses transmute to convert between the types, implement other methods

-----
Examples:

{
    use ruint::Uint;

    redefined_remote!(Uint : "ruint");
    redefined_remote!(#[derive(Clone)] Uint : "ruint");

}

*/

#[cfg(feature = "remote")]
#[proc_macro]
pub fn redefined_remote(input: TokenStream) -> TokenStream {
    remote::expand_redefined_remote(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
