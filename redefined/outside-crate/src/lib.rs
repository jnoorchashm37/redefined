#[derive(Debug, Clone, PartialEq, Default)]
pub struct OutsideStruct {
    pub val1: u64,
    pub val2: f64,
    pub val3: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NonPubFieldStructA {
    p:        u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

impl NonPubFieldStructA {
    pub fn new(p: u64, d: u64, vals: Vec<String>) -> Self {
        Self { p, d, vals }
    }

    pub fn get_p(&self) -> u64 {
        self.p
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GenericStructA<X, Y> {
    pub p:    u64,
    pub d:    X,
    pub vals: Vec<Y>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericConstantStructA<const XVAL: usize> {
    pub p: u64,
    pub d: [i128; XVAL],
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransmuteStructA<X, Y> {
    p: ComplexOutsideEnumA,
    d: GenericStructA<X, Y>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComplexOutsideEnumA {
    A(u64),
    C { value: Vec<OutsideStruct> },
}
