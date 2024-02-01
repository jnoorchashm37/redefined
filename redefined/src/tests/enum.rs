use redefined_derive::Redefined;
use redefined_outside_crate_tests::{ComplexOutsideEnumA, OutsideStructA};

use super::r#struct::{ComplexStructA, ComplexStructB, OutsideStructB};
use crate::RedefinedConvert;

/*





Basic Enum
*/
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

/*





Complex Enum
*/
#[derive(Debug, PartialEq, Clone)]
pub enum ComplexEnumA {
    A(u64),
    B(ComplexStructA),
    C { value: Vec<OutsideStructA> },
}

#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(ComplexEnumA)]
pub enum ComplexEnumB {
    A(u64),
    B(ComplexStructB),
    C { value: Vec<OutsideStructB> },
}

#[test]
fn test_complex_enum() {
    // case 1
    let enum_a = ComplexEnumA::A(100);
    let enum_b: ComplexEnumB = enum_a.clone().into();
    assert_eq!(ComplexEnumB::A(100), enum_b);
    let enum_b_to_a: ComplexEnumA = enum_b.into();
    assert_eq!(enum_b_to_a, enum_a);

    // case 2
    let enum_a = ComplexEnumA::B(ComplexStructA::default());
    let enum_b: ComplexEnumB = enum_a.clone().into();
    let enum_b_to_a: ComplexEnumA = enum_b.into();
    assert_eq!(enum_b_to_a, enum_a);

    // case 3
    let enum_a = ComplexEnumA::C { value: vec![OutsideStructA::default()] };
    let enum_b: ComplexEnumB = enum_a.clone().into();
    let enum_b_to_a: ComplexEnumA = enum_b.into();
    assert_eq!(enum_b_to_a, enum_a);
}
/*





Complex Outside Enum
*/
#[derive(Debug, PartialEq, Clone, Redefined)]
#[redefined(ComplexOutsideEnumA)]
pub enum ComplexOutsideEnumB {
    A(u64),
    C { value: Vec<OutsideStructB> },
}

#[test]
fn test_complex_outside_enum() {
    // case 1
    let enum_a = ComplexOutsideEnumA::A(100);
    let enum_b: ComplexOutsideEnumB = enum_a.clone().into();
    assert_eq!(ComplexOutsideEnumB::A(100), enum_b);
    let enum_b_to_a: ComplexOutsideEnumA = enum_b.into();
    assert_eq!(enum_b_to_a, enum_a);

    // case 2
    let enum_a = ComplexOutsideEnumA::C { value: vec![OutsideStructA::default()] };
    let enum_b: ComplexOutsideEnumB = enum_a.clone().into();
    let enum_b_to_a: ComplexOutsideEnumA = enum_b.into();
    assert_eq!(enum_b_to_a, enum_a);
}
