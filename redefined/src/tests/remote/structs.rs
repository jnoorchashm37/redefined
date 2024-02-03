use redefined_test_types::structs::*;

use crate::{redefined_remote, struct_test, Redefined, RedefinedConvert};

mod crates_io {
    use ruint::Uint;

    use super::*;

    redefined_remote!(Uint : "ruint");

    struct_test!((UintRedefined, 256, 4), Uint, { Uint::from_limbs([100; 4]) });
}

mod github {
    use super::*;

    redefined_remote!(BasicStruct : "redefined-test-types");

    redefined_remote!(GenericTypeStruct : "redefined-test-types");

    redefined_remote!(GenericConstantStruct : "redefined-test-types");

    redefined_remote!(GenericLifetimeStruct : "redefined-test-types");

    redefined_remote!(ComplexStructA : "redefined-test-types");

    struct_test!(BasicStructRedefined, BasicStruct);
    struct_test!((GenericTypeStructRedefined, String, u64), GenericTypeStruct);
    struct_test!((GenericConstantStructRedefined, 100), GenericConstantStruct, { GenericConstantStruct::new([2; 100]) });
    struct_test!(GenericLifetimeStructRedefined, GenericLifetimeStruct);
    struct_test!(ComplexStructARedefined, ComplexStructA);

    fn t() {
        let t: ComplexStructARedefined;
    }
}

mod derives {
    use serde::{Deserialize, Serialize};

    use super::*;

    redefined_remote!(#[derive(Serialize, Deserialize)] BasicStruct : "redefined-test-types");

    #[test]
    fn test_derive() {
        let val: BasicStructRedefined = BasicStruct::default().into();

        // using serde
        let ser = serde_json::to_value(val);
        assert!(ser.is_ok())
    }
}

mod lol {
    use malachite::{platform_64::Limb, Natural, Rational};
    use rkyv::{Archive as rkyvArchive, Deserialize as rkyvDeserialize, Serialize as rkyvSerialize};

    use crate::{redefined_remote, Redefined, RedefinedConvert};

    // Rational
    redefined_remote!(
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
        )]
        Rational : "malachite-q"
    );

    // Natural
    redefined_remote!(
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            rkyvSerialize,
            rkyvDeserialize,
            rkyvArchive,
        )]
        Natural : "malachite-nz"
    );

    // InnerNatural
    redefined_remote!(
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            rkyvSerialize,
            rkyvDeserialize,
            rkyvArchive,
        )]
        InnerNatural : "malachite-nz" : no_impl
    );

    /*
    #[derive(Clone, Eq, Hash, PartialEq)]
    pub(crate) enum InnerNatural {
        Small(LimbRedefined),
        Large(Vec<LimbRedefined>),
    }
    */
    pub type LimbRedefined = u64;

    fn t() {
        let t: RationalRedefined;
        let n: NaturalRedefined;
    }
}
