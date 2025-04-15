use redefined::Redefined;
use redefined_test_types::structs::*;

use crate::struct_test;

mod derive_transmute_source {
    use super::*;

    /// basic struct
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined(BasicStruct)]
    #[redefined_attr(transmute)]
    pub struct BasicStructA {
        pub val1: u64,
        pub val2: f64,
        pub val3: String,
    }

    /// struct with type generics
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined(GenericTypeStruct)]
    #[redefined_attr(transmute)]
    pub struct GenericTypeStructA<X, Y> {
        pub p: u64,
        pub d: X,
        pub vals: Vec<Y>,
    }

    /// struct with constant generics
    #[derive(Debug, Clone, PartialEq, Redefined)]
    #[redefined(GenericConstantStruct)]
    #[redefined_attr(transmute)]
    pub struct GenericConstantStructA<const XVAL: usize> {
        pub p: u64,
        pub d: [i128; XVAL],
    }

    /// struct with lifetime generics
    #[derive(Debug, Clone, PartialEq, Redefined)]
    #[redefined(GenericLifetimeStruct)]
    #[redefined_attr(transmute)]
    pub struct GenericLifetimeStructA<'a, 'b> {
        pub p: &'a u64,
        pub d: &'b [i128; 10],
    }

    impl<'a, 'b> Default for GenericLifetimeStructA<'a, 'b> {
        fn default() -> Self {
            Self { p: &100, d: &[0; 10] }
        }
    }

    /// complex struct 1
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined(ComplexStructA)]
    #[redefined_attr(transmute)]
    pub struct ComplexStructAA<'a, 'b> {
        pub n: i128,
        pub inner_a: GenericLifetimeStructA<'a, 'b>,
        pub inner_b: Vec<BasicStructA>,
    }

    struct_test!(BasicStructA, BasicStruct);
    struct_test!((GenericTypeStructA, String, u64), GenericTypeStruct);
    struct_test!((GenericConstantStructA, 100), GenericConstantStruct, { GenericConstantStruct::new([2; 100]) });
    struct_test!(GenericLifetimeStructA, GenericLifetimeStruct);
    struct_test!(ComplexStructAA, ComplexStructA);
}

mod derive_transmute_no_source {
    use super::*;

    /// basic struct
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined_attr(derive(Debug, Clone, PartialEq, Default))]
    #[redefined_attr(transmute)]
    pub struct BasicStructA {
        pub val1: u64,
        pub val2: f64,
        pub val3: String,
    }

    /// struct with type generics
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined_attr(transmute)]
    pub struct GenericTypeStructA<X, Y> {
        pub p: u64,
        pub d: X,
        pub vals: Vec<Y>,
    }

    /// struct with constant generics
    #[derive(Debug, Clone, PartialEq, Redefined)]
    #[redefined_attr(transmute)]
    pub struct GenericConstantStructA<const XVAL: usize> {
        pub p: u64,
        pub d: [i128; XVAL],
    }

    impl<const XVAL: usize> GenericConstantStructA<XVAL> {
        pub fn new(d: [i128; XVAL]) -> Self {
            Self { p: Default::default(), d }
        }
    }

    /// struct with lifetime generics
    #[derive(Debug, Clone, PartialEq, Redefined)]
    #[redefined_attr(derive(Debug, Clone, PartialEq))]
    #[redefined_attr(transmute)]
    pub struct GenericLifetimeStructA<'a, 'b> {
        pub p: &'a u64,
        pub d: &'b [i128; 10],
    }

    impl<'a, 'b> Default for GenericLifetimeStructA<'a, 'b> {
        fn default() -> Self {
            // let t: std::mem::MaybeUninit<Vec<u128>> = std::mem::MaybeUninit::uninit();
            // let f = t.as_mut_ptr();

            Self { p: &100, d: &[0; 10] }
        }
    }

    impl<'a, 'b> Default for GenericLifetimeStructARedefined<'a, 'b> {
        fn default() -> Self {
            Self { p: &100, d: &[0; 10] }
        }
    }

    /// complex struct 1
    #[derive(Debug, Clone, PartialEq, Default, Redefined)]
    #[redefined_attr(derive(Debug, Clone, PartialEq))]
    #[redefined_attr(transmute)]
    pub struct ComplexStructAA<'a, 'b> {
        pub n: i128,
        #[redefined(field((GenericLifetimeStructA, default)))]
        pub inner_a: GenericLifetimeStructA<'a, 'b>,
        #[redefined(field((BasicStructA, default)))]
        pub inner_b: Vec<BasicStructA>,
    }

    struct_test!(BasicStructARedefined, BasicStructA);
    struct_test!((GenericTypeStructARedefined, String, u64), GenericTypeStructA);
    struct_test!((GenericConstantStructARedefined, 100), GenericConstantStructA, { GenericConstantStructA::new([2; 100]) });
    struct_test!(GenericLifetimeStructARedefined, GenericLifetimeStructA);
    struct_test!(ComplexStructAARedefined, ComplexStructAA);
}
