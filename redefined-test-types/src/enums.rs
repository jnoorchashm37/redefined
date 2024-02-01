use crate::structs::BasicStruct;

/// complex enum 1
#[derive(Debug, PartialEq, Clone)]
pub enum ComplexEnumA {
    A(u64),
    C { value: Vec<BasicStruct> },
}
