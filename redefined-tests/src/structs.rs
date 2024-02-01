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

/// struct with constant generics
#[derive(Debug, Clone, PartialEq)]
pub struct GenericConstantStruct<const XVAL: usize> {
    pub p: u64,
    pub d: [i128; XVAL],
}

/// struct with constant generics
#[derive(Debug, Clone, PartialEq)]
pub struct GenericLifetimeStruct<'a, 'b> {
    pub p: &'a u64,
    pub d: &'b [i128; 10],
}

/// transmute struct A
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransmuteStructA<X, Y> {
    p: ComplexOutsideEnumA,
    d: GenericStructA<X, Y>,
}

/// struct with constant generics
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexStructB<'a, 'b> {
    pub p: &'a u64,
    pub d: &'b [i128; 10],
}
