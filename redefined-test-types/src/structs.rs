use std::collections::HashMap;

use crate::enums::ComplexEnumA;

/// basic struct
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BasicStruct {
    pub val1: u64,
    pub val2: f64,
    pub val3: String,
}

/// struct with a private field
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrivateFieldStruct {
    p:        u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

impl PrivateFieldStruct {
    pub fn new(p: u64, d: u64, vals: Vec<String>) -> Self {
        Self { p, d, vals }
    }

    pub fn get_p(&self) -> u64 {
        self.p
    }
}

/// struct with generics types
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GenericTypeStruct<X, Y> {
    pub p:    u64,
    pub d:    X,
    pub vals: Vec<Y>,
}

impl<X, Y> GenericTypeStruct<X, Y> {
    pub fn new(d: X, vals: Vec<Y>) -> Self {
        Self { p: Default::default(), d, vals }
    }
}

/// struct with constant generics
#[derive(Debug, Clone, PartialEq)]
pub struct GenericConstantStruct<const XVAL: usize> {
    pub p: u64,
    pub d: [i128; XVAL],
}

impl<const XVAL: usize> GenericConstantStruct<XVAL> {
    pub fn new(d: [i128; XVAL]) -> Self {
        Self { p: Default::default(), d }
    }
}

/// struct with constant generics
#[derive(Debug, Clone, PartialEq)]
pub struct GenericLifetimeStruct<'a, 'b> {
    pub p: &'a u64,
    pub d: &'b [i128; 10],
}

impl<'a, 'b> Default for GenericLifetimeStruct<'a, 'b> {
    fn default() -> Self {
        Self { p: &100, d: &[0; 10] }
    }
}

/// transmute struct 1
#[derive(Debug, Clone, PartialEq)]
pub struct TransmuteStructA<X, Y> {
    p: ComplexEnumA,
    d: GenericTypeStruct<X, Y>,
}

/// complex struct 1
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComplexStructA {
    pub n:       i128,
    pub inner_a: PrivateFieldStruct,
    pub inner_b: Vec<BasicStruct>,
}

/// complex struct 2
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexStructB<'a, 'b, const XVAL: usize, X, Y, Z> {
    pub p: &'a u64,
    pub d: &'b [i128; XVAL],
    k:     HashMap<u64, (X, Y)>,
    t:     TransmuteStructA<Y, Z>,
}

impl<'a, 'b, const XVAL: usize, X, Y, Z> ComplexStructB<'a, 'b, XVAL, X, Y, Z>
where
    Y: Default + Clone,
    X: Clone,
    Z: Clone,
{
    pub fn new(p: &'a u64, d: &'b [i128; XVAL]) -> Self {
        Self {
            p,
            d,
            k: Default::default(),
            t: TransmuteStructA {
                p: ComplexEnumA::C { value: Default::default() },
                d: GenericTypeStruct { p: Default::default(), d: Default::default(), vals: Vec::new() },
            },
        }
    }

    pub fn get_k(&self) -> HashMap<u64, (X, Y)> {
        self.k.clone()
    }

    pub fn get_t(&self) -> TransmuteStructA<Y, Z> {
        self.t.clone()
    }
}
