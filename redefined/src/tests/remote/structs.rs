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
    use alloy_primitives::FixedBytes;
    use derive_more::{Deref, DerefMut, From, Index, IndexMut, IntoIterator};
    use reth_primitives::alloy_primitives;

    use crate::{redefined_remote, Redefined, RedefinedConvert};

    redefined_remote!(
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Deref,
            DerefMut,
            From,
            Index,
            IndexMut,
            IntoIterator,
        )]
        FixedBytes : "alloy-primitives" : no_impl
    );
}
