use redefined_derive::Redefined;
use redefined_outside_crate_tests::OutsideStruct;

use crate::RedefinedConvert;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StructA {
    pub p:    u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(StructA)]
pub struct StructB {
    pub p:    u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(OutsideStruct)]
pub struct InsideStruct {
    pub val1: u64,
    pub val2: f64,
    pub val3: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComplexStructA {
    pub n:       i128,
    pub inner_a: StructA,
    pub inner_b: Vec<OutsideStruct>,
}

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(ComplexStructA)]
pub struct ComplexStructB {
    pub n:       i128,
    pub inner_a: StructB,
    pub inner_b: Vec<InsideStruct>,
}

#[test]
fn test_struct() {
    let struct_a = StructA::default();
    let struct_b = StructB::from_source(struct_a.clone());
    let struct_b_to_a: StructA = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}

#[test]
fn test_outside_crate_struct() {
    let struct_a = OutsideStruct::default();
    let struct_b = InsideStruct::from_source(struct_a.clone());
    let struct_b_to_a: OutsideStruct = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}

#[test]
fn test_complex_struct() {
    let struct_a = ComplexStructA::default();
    let struct_b = ComplexStructB::from_source(struct_a.clone());
    let struct_b_to_a: ComplexStructA = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}
