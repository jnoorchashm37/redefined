# Redefined
Implements a conversion trait between any 2 types iff they have the same fields.
- Alternative to wrapper types when types are defined in another crate.

### Derive Macro
Derive a type with: 
``` rust
#[derive(Redefined)]
#[redefined(<Type Converting From>)]
```
---
``` rust
#[derive(Debug, PartialEq, Clone)]
pub enum ComplexEnumA {
    A(u64),
    B(ComplexStructA), // ComplexStructA defined in another crate
    C { value: Vec<OutsideStruct> }, // OutsideStruct defined in another crate
}

#[derive(Debug, Clone, PartialEq, Redefined)]
#[redefined(ComplexEnumA)] // define the type converting from
pub enum ComplexEnumB {
    A(u64),
    B(ComplexStructB), // ComplexStructB defined in our crate and derives `Redefined` from ComplexStructA
    C { value: Vec<InsideStruct> }, // InsideStruct defined in our crate and derives `Redefined` from OutsideStruct
}

// case 1
let enum_a = ComplexEnumA::A(100);
let enum_b = ComplexEnumB::from_source(enum_a.clone());
assert_eq!(ComplexEnumB::A(100), enum_b);
let enum_b_to_a: ComplexEnumA = enum_b.to_source();
assert_eq!(enum_b_to_a, enum_a);

// case 2
let enum_a = ComplexEnumA::B(ComplexStructA::default());
let enum_b = ComplexEnumB::from_source(enum_a.clone());
let enum_b_to_a: ComplexEnumA = enum_b.to_source();
assert_eq!(enum_b_to_a, enum_a);

// case 3
let enum_a = ComplexEnumA::C {
    value: vec![OutsideStruct::default()],
};
let enum_b = ComplexEnumB::from_source(enum_a.clone());
let enum_b_to_a: ComplexEnumA = enum_b.to_source();
assert_eq!(enum_b_to_a, enum_a);
```
