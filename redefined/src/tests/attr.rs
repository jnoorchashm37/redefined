use redefined_derive::Redefined;
use redefined_outside_crate_tests::{GenericConstantStructA, NonPubFieldStructA, TransmuteStructA};

use super::{
    generics::{GenericConstantStructB, GenericStructB},
    r#enum::{ComplexEnumB, ComplexOutsideEnumB},
};
use crate::RedefinedConvert;

/*





Source struct with private field
- Uses 'to_source' attribute to create the source struct
- Calls the 'get_p()' function to get the field for this struct
*/
#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(NonPubFieldStructA)]
#[redefined_attr(to_source = "NonPubFieldStructA::new(self.p, self.d, self.vals)")]
pub struct NonPubFieldStructB {
    #[redefined_attr(func = "src.get_p()")]
    pub p:    u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

#[test]
fn test_struct_non_pub_and_new_source_fn_field() {
    let struct_a = NonPubFieldStructA::default();
    let struct_b: NonPubFieldStructB = struct_a.clone().into();
    let struct_b_to_a: NonPubFieldStructA = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}

/*





Source struct with private field
- Uses 'to_source' attribute to create the source struct
- Uses 'from_source' attribute to create the self
*/
#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(NonPubFieldStructA)]
#[redefined_attr(to_source = "NonPubFieldStructA::new(self.p, self.d, self.vals)", from_source = "ToFromSourceFieldStructB::new(src)")]
pub struct ToFromSourceFieldStructB {
    #[redefined_attr(func = "src.get_p()")]
    pub p:    u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

impl ToFromSourceFieldStructB {
    pub fn new(val: NonPubFieldStructA) -> Self {
        Self { p: val.get_p(), d: val.d, vals: val.vals }
    }
}

#[test]
fn test_struct_new_self_and_new_source_fn_field() {
    let struct_a = NonPubFieldStructA::default();
    let struct_b: NonPubFieldStructB = struct_a.clone().into();
    let struct_b_to_a: NonPubFieldStructA = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}
/*





/*
Transmute between the types
- Uses 'to_source' attribute to create the source struct
- Uses 'from_source' attribute to create the self
*/
#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(TransmuteStructA)]
#[redefined_attr(transmute, source_generics(A, B))]
pub struct TransmuteStructB<X, Y> {
    p: ComplexOutsideEnumB,
    d: GenericStructB<X, Y>,
}

#[test]
fn test_transmute_struct() {
    let struct_a = NonPubFieldStructA::default();
    let struct_b = ToFromSourceFieldStructB::from_source(struct_a.clone());
    let struct_b_to_a: NonPubFieldStructA = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}
*/
