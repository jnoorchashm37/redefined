use redefined_derive::Redefined;
use redefined_outside_crate_tests::OutsideStructA;

use crate::{tests::r#struct::OutsideStructB, RedefinedConvert};
/*






Derives a new struct from an owned struct with fields with redefined types
*/
#[derive(Debug, Default, Clone, PartialEq, Redefined)]
pub struct SelfSourceStructA<X, Y> {
    p: String,
    c: Y,
    d: Vec<X>,
}

#[test]
fn test_self_source_struct() {
    let struct_a: SelfSourceStructA<u64, OutsideStructA> = SelfSourceStructA::default();
    let struct_b: SelfSourceStructARedefined<u64, OutsideStructB> = SelfSourceStructARedefined::from_source(struct_a.clone());
    let struct_b_to_a: SelfSourceStructA<u64, OutsideStructA> = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}
/*






Derives a new struct from an owned struct with fields with redefined types
Uses a default redefined type (i.e. the type concatenated with 'Redefined') for 1 of the fields
*/
#[derive(Debug, Default, Clone, PartialEq, Redefined)]
pub struct SelfSourceStructWithDefaultAttributeA<X> {
    p: String,
    #[redefined(field())]
    c: SelfSourceStructWithAttributeA<X>,
    d: Vec<X>,
}

#[test]
fn test_self_source_struct_with_default_attr() {
    let struct_a: SelfSourceStructWithDefaultAttributeA<u64> = SelfSourceStructWithDefaultAttributeA::default();
    let struct_b: SelfSourceStructWithDefaultAttributeARedefined<u64> = SelfSourceStructWithDefaultAttributeARedefined::from_source(struct_a.clone());
    let struct_b_to_a: SelfSourceStructWithDefaultAttributeA<u64> = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}

/*






Derives a new struct from an owned struct with fields with redefined types
Uses a custom redefined type for 1 of the fields
*/
#[derive(Debug, Default, Clone, PartialEq, Redefined)]
pub struct SelfSourceStructWithAttributeA<X> {
    p: String,
    #[redefined(field(OutsideStructB))]
    c: OutsideStructA,
    d: Vec<X>,
}

#[test]
fn test_self_source_struct_with_attr() {
    let struct_a: SelfSourceStructWithAttributeA<u64> = SelfSourceStructWithAttributeA::default();
    let struct_b: SelfSourceStructWithAttributeARedefined<u64> = SelfSourceStructWithAttributeARedefined::from_source(struct_a.clone());
    let struct_b_to_a: SelfSourceStructWithAttributeA<u64> = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}
