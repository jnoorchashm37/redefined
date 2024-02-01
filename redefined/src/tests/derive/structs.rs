use redefined_derive::Redefined;
use redefined_test_types::structs::BasicStruct;

use crate::RedefinedConvert;

/*





Basic struct
*/

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

#[test]
fn test_struct() {
    let struct_a = StructA::default();
    let struct_b: StructB = struct_a.clone().into();
    let struct_b_to_a: StructA = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}

/*





Struct with unnamed fields
*/

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UnnamedStructA(u64, String);

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(UnnamedStructA)]
pub struct UnnamedStructB(u64, String);

#[test]
fn test_unnamed_struct() {
    let struct_a = UnnamedStructA::default();
    let struct_b: UnnamedStructB = struct_a.clone().into();
    let struct_b_to_a: UnnamedStructA = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}

/*





Struct with source in another crate (redefined/outside-crate)
*/

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(BasicStruct)]
pub struct OutsideStructB {
    pub val1: u64,
    pub val2: f64,
    pub val3: String,
}

#[test]
fn test_outside_crate_struct() {
    let struct_a = BasicStruct::default();
    let struct_b: OutsideStructB = struct_a.clone().into();
    let struct_b_to_a: BasicStruct = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}

/*





Complex struct
*/

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComplexStructA {
    pub n:       i128,
    pub inner_a: StructA,
    pub inner_b: Vec<BasicStruct>,
}

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(ComplexStructA)]
pub struct ComplexStructB {
    pub n:       i128,
    pub inner_a: StructB,
    pub inner_b: Vec<OutsideStructB>,
}

#[test]
fn test_complex_struct() {
    let struct_a = ComplexStructA::default();
    let struct_b: ComplexStructB = struct_a.clone().into();
    let struct_b_to_a: ComplexStructA = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}
