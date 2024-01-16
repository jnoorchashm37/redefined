use super::r#struct::{ComplexStructA, ComplexStructB, InsideStruct};
use crate::RedefinedConvert;
use redefined_derive::Redefined;
use redefined_outside_crate_tests::OutsideStruct;

#[derive(Debug, PartialEq, Clone)]
pub enum EnumA {
    A,
    B,
    C(u64),
}

#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(EnumA)]
pub enum EnumB {
    A,
    B,
    C(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComplexEnumA {
    A(u64),
    B(ComplexStructA),
    C { value: Vec<OutsideStruct> },
}

#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(ComplexEnumA)]
pub enum ComplexEnumB {
    A(u64),
    B(ComplexStructB),
    C { value: Vec<InsideStruct> },
}

//remake!(ComplexEnumB, ComplexEnumB2);

#[test]
fn test_enum() {
    // case 1
    let enum_a = EnumA::A;
    let enum_b = EnumB::from_source(enum_a.clone());
    assert_eq!(EnumB::A, enum_b);
    let enum_b_to_a: EnumA = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);

    // case 2
    let enum_a = EnumA::C(100);
    let enum_b = EnumB::from_source(enum_a.clone());
    let enum_b_to_a: EnumA = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);
}

#[test]
fn test_complex_enum() {
    // case 1
    let enum_a = ComplexEnumA::A(100);
    let enum_b = ComplexEnumB::from_source(enum_a.clone());
    assert_eq!(ComplexEnumB::A(100), enum_b);
    let enum_b_to_a: ComplexEnumA = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);

    // case 2
    let enum_a = ComplexEnumA::B(ComplexStructA::default());
    let enum_b = ComplexEnumB::from_source(enum_a.clone());
    let enum_b_to_a: ComplexEnumA = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);

    // case 3
    let enum_a = ComplexEnumA::C {
        value: vec![OutsideStruct::default()],
    };
    let enum_b = ComplexEnumB::from_source(enum_a.clone());
    let enum_b_to_a: ComplexEnumA = enum_b.to_source();
    assert_eq!(enum_b_to_a, enum_a);
}
