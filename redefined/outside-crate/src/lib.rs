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
