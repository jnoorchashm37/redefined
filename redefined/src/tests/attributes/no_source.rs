use std::default;

use redefined_derive::Redefined;
use redefined_outside_crate_tests::derive::OutsideStructA;

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

/*






Derives a new enum from an owned enum with fields with redefined types
*/
#[derive(Debug, Clone, PartialEq, Redefined)]
pub enum SelfSourceEnumA<X, Y> {
    P,
    C(Y),
    D { d: Vec<X> },
}

#[test]
fn test_self_source_enum() {
    let enum_a: SelfSourceEnumA<u64, OutsideStructA> = SelfSourceEnumA::C(Default::default());
    let enum_b: SelfSourceEnumARedefined<u64, OutsideStructB> = SelfSourceEnumARedefined::from_source(enum_a.clone());
    let enum_b_to_a: SelfSourceEnumA<u64, OutsideStructA> = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);
}
/*






Derives a new enum from an owned enum with fields with redefined types
Uses a default redefined type (i.e. the type concatenated with 'Redefined') for 1 of the fields
*/
#[derive(Debug, Clone, PartialEq, Redefined)]
pub enum SelfSourceEnumWithDefaultAttributeA<X> {
    P,
    C(X),
    D {
        #[redefined(field())]
        d: SelfSourceStructWithAttributeA<X>,
    },
}

#[test]
fn test_self_source_enum_with_default_attr() {
    let enum_a: SelfSourceEnumWithDefaultAttributeA<u64> = SelfSourceEnumWithDefaultAttributeA::D { d: Default::default() };
    let enum_b: SelfSourceEnumWithDefaultAttributeARedefined<u64> = SelfSourceEnumWithDefaultAttributeARedefined::from_source(enum_a.clone());
    let enum_b_to_a: SelfSourceEnumWithDefaultAttributeA<u64> = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);
}

/*






Derives a new struct from an owned struct with fields with redefined types
Uses a custom redefined type for 1 of the fields
*/
#[derive(Debug, Clone, PartialEq, Redefined)]
pub enum SelfEnumWithAttributeA<X> {
    P,
    C(#[redefined(field(OutsideStructB))] OutsideStructA),
    D { d: Vec<X> },
}

#[test]
fn test_self_source_enum_with_attr() {
    let enum_a: SelfEnumWithAttributeA<OutsideStructA> = SelfEnumWithAttributeA::C(Default::default());
    let enum_b: SelfEnumWithAttributeARedefined<OutsideStructB> = SelfEnumWithAttributeARedefined::from_source(enum_a.clone());
    let enum_b_to_a: SelfEnumWithAttributeA<OutsideStructA> = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);
}
