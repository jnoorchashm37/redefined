use redefined_derive::Redefined;
use redefined_outside_crate_tests::{NonPubFieldStructA, TransmuteStructA};

use crate::{
    tests::{attributes::to_from_src::ToFromSourceFieldStructB, generics::GenericStructB, r#enum::ComplexOutsideEnumB},
    RedefinedConvert,
};

/*






Transmute between types
*/
#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(TransmuteStructA)]
#[redefined_attr(transmute)]
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
