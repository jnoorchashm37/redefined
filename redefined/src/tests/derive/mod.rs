use redefined_test_types::structs::*;

use crate::{struct_test, Redefined, RedefinedConvert};
pub mod attributes;
//pub mod enums;
//pub mod generics;
pub mod structs;

/*





Basic Struct with tagged source type
*/
#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(BasicStruct)]
pub struct BasicStructA {
    pub val1: u64,
    pub val2: f64,
    pub val3: String,
}

/// struct with generics types
#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(GenericTypeStruct)]
pub struct GenericTypeStructA<X, Y> {
    pub p:    u64,
    pub d:    X,
    pub vals: Vec<Y>,
}

/// struct with constant generics
#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(GenericConstantStruct)]
pub struct GenericConstantStructA<const XVAL: usize> {
    pub p: u64,
    pub d: [i128; XVAL],
}

/// struct with constant generics
#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(GenericLifetimeStruct)]
pub struct GenericLifetimeStructA<'a, 'b> {
    pub p: &'a u64,
    pub d: &'b [i128; 10],
}

struct_test!(BasicStructA, BasicStruct);
struct_test!(GenericTypeStructA, GenericTypeStruct);
struct_test!(GenericConstantStructA, GenericConstantStruct, { GenericConstantStructA::new([100; 2]) });
struct_test!(GenericLifetimeStructA, GenericLifetimeStruct);
