use redefined_derive::Redefined;
use redefined_test_types::structs::{GenericConstantStruct, GenericTypeStruct};

use crate::RedefinedConvert;

/*





Struct with type generics
*/

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(GenericTypeStruct)]
pub struct GenericStructB<X, Y> {
    pub p:    u64,
    pub d:    X,
    pub vals: Vec<Y>,
}

#[test]
fn test_type_generic_struct() {
    let struct_a: GenericTypeStruct<i32, String> = GenericTypeStruct { p: 10, d: 100, vals: vec![String::new()] };
    let struct_b: GenericStructB<i32, String> = struct_a.clone().into();
    let struct_b_to_a: GenericTypeStruct<i32, String> = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}

/*





Struct with constant generics
*/

#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(GenericConstantStruct)]
pub struct GenericConstantStructB<const XVAL: usize> {
    pub p: u64,
    pub d: [i128; XVAL],
}

#[test]
fn test_const_generic_struct() {
    let struct_a = GenericConstantStruct { p: 100, d: [100, 231, -12356] };
    let struct_b: GenericConstantStructB<3> = struct_a.clone().into();
    let struct_b_to_a: GenericConstantStruct<3> = struct_b.into();
    assert_eq!(struct_b_to_a, struct_a);
}
