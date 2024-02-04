use redefined::{redefined_remote, Redefined, RedefinedConvert};
use redefined_test_types::structs::*;

use crate::struct_test;

mod crates_io {
    use ruint::Uint;

    use super::*;

    redefined_remote!([Uint] : "ruint");

    struct_test!((UintRedefined, 256, 4), Uint, { Uint::from_limbs([100; 4]) });
}

mod github {
    use super::*;

    redefined_remote!([BasicStruct] : "redefined-test-types");

    redefined_remote!([GenericTypeStruct] : "redefined-test-types");

    redefined_remote!([GenericConstantStruct] : "redefined-test-types");

    redefined_remote!([GenericLifetimeStruct] : "redefined-test-types");

    redefined_remote!([ComplexStructA] : "redefined-test-types");

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

    redefined_remote!(#[derive(Serialize, Deserialize)] [BasicStruct] : "redefined-test-types");

    #[test]
    fn test_derive() {
        let val: BasicStructRedefined = BasicStruct::default().into();

        // using serde
        let ser = serde_json::to_value(val);
        assert!(ser.is_ok())
    }
}

mod lol {
    use std::{fmt, hash::Hash, str::FromStr};

    use alloy_primitives::{hex, Address, Bytes as AlloyBytes, FixedBytes, Uint};
    use derive_more::{Deref, DerefMut, From, Index, IndexMut, IntoIterator};
    use malachite::{platform_64::Limb, Natural, Rational};
    use redefined::{redefined_remote, Redefined, RedefinedConvert};
    use rkyv::{Archive as rkyvArchive, Deserialize as rkyvDeserialize, Serialize as rkyvSerialize};
    use serde::{Deserialize as serdeDeserialize, Deserializer, Serialize as serdeSerialize, Serializer};

    use super::*;

    // Uint
    redefined_remote!(
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, rkyvSerialize, rkyvDeserialize, rkyvArchive)]
        [Uint] : "ruint"
    );

    impl<const BITS: usize, const LIMBS: usize> Default for UintRedefined<BITS, LIMBS> {
        fn default() -> Self {
            Self { limbs: [0; LIMBS] }
        }
    }

    pub type U256Redefined = UintRedefined<256, 4>;
    pub type U64Redefined = UintRedefined<64, 1>;

    //FixedBytes
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
            rkyvSerialize,
            rkyvDeserialize,
            rkyvArchive,
        )]
        [FixedBytes] : "alloy-primitives"
    );

    pub type TxHashRedefined = FixedBytesRedefined<32>;

    impl<const N: usize> serdeSerialize for FixedBytesRedefined<N> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let this = self.to_source();
            serdeSerialize::serialize(&this, serializer)
        }
    }

    impl<const N: usize> Hash for ArchivedFixedBytesRedefined<N> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl<const N: usize> Eq for ArchivedFixedBytesRedefined<N> {}

    impl<const N: usize> PartialEq for ArchivedFixedBytesRedefined<N> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    /// Address
    /// Haven't implemented macro stuff yet
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
        serde::Serialize,
        Index,
        IndexMut,
        IntoIterator,
        Redefined,
        rkyv::Serialize,
        rkyv::Deserialize,
        rkyv::Archive,
    )]
    #[redefined(Address)]
    #[archive_attr(derive(Hash, PartialEq, Eq))]
    pub struct AddressRedefined(FixedBytesRedefined<20>);

    impl FromStr for AddressRedefined {
        type Err = hex::FromHexError;

        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(AddressRedefined::from_source(Address::from_str(s)?))
        }
    }

    /// Bytes
    /// Have not implements parsing 'Bytes::bytes' yet
    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, rkyv::Serialize, rkyv::Deserialize, rkyv::Archive, Redefined)]
    #[redefined(AlloyBytes)]
    #[redefined_attr(to_source = "self.0.into()", from_source = "Self(src.to_vec())")]
    pub struct AlloyBytesRedefined(pub Vec<u8>);

    #[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, Redefined)]
    #[redefined_attr(
        derive(Debug, Clone, PartialEq, Eq, Hash, rkyvArchive, rkyvDeserialize, rkyvSerialize)
    )]
    #[redefined_attr(
        other(#[archive_attr(derive(Hash, PartialEq, Eq))]), 
    )]
    pub struct Pair(#[redefined(field((Address, default)))] pub Address, #[redefined(field((Address, default)))] pub Address);

    impl Pair {
        pub fn ordered(&self) -> Self {
            if self.0 <= self.1 {
                Pair(self.0, self.1)
            } else {
                Pair(self.1, self.0)
            }
        }
    }

    impl Eq for Pair {}

    impl PartialEq for Pair {
        fn eq(&self, other: &Self) -> bool {
            self.ordered().0 == other.ordered().0 && self.ordered().1 == other.ordered().1
        }
    }

    fn t() {
       // let t: ArchivedPairRedefined = ArchivedPairRedefined::hash();
    }
}
