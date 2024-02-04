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
    use std::{fmt, hash::Hash, str::FromStr, sync::atomic::AtomicPtr};

    use alloy_primitives::{hex, Address, Bytes as AlloyBytes, FixedBytes, Log, LogData, Uint};
    use bytes::Bytes;
    use derive_more::{Deref, DerefMut, From, Index, IndexMut, IntoIterator};
    use malachite::{platform_64::Limb, Natural, Rational};
    use redefined::{redefined_remote, Redefined, RedefinedConvert};
    use rkyv::{
        out_field,
        vec::{ArchivedVec, VecResolver},
        Archive, Archive as rkyvArchive, ArchiveUnsized, Archived, Deserialize as rDeserialize, Fallible, Infallible, MetadataResolver, RelPtr,
        Serialize as rSerialize, SerializeUnsized,
    };
    use serde::{Deserialize, Serialize};

    use super::*;

    // Uint
    redefined_remote!(
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, rSerialize, rDeserialize, Archive)]
        [Uint] : "ruint"
    );

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
            rSerialize,
            rDeserialize,
            Archive,
        )]
        [FixedBytes] : "alloy-primitives"
    );

    impl<const N: usize> Default for FixedBytesRedefined<N> {
        fn default() -> Self {
            FixedBytesRedefined([0; N])
        }
    }

    pub type TxHashRedefined = FixedBytesRedefined<32>;
    pub type B256Redefined = FixedBytesRedefined<32>;

    impl<const N: usize> Serialize for FixedBytesRedefined<N> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let this = self.to_source();
            Serialize::serialize(&this, serializer)
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
        Default,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Deref,
        DerefMut,
        From,
        Serialize,
        Index,
        IndexMut,
        IntoIterator,
        Redefined,
        rSerialize,
        rDeserialize,
        Archive,
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
    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, Deserialize, Default, Redefined)]
    #[redefined(AlloyBytes)]
    #[redefined_attr(transmute)]
    pub struct AlloyBytesRedefined(#[serde(with = "s")] pub Bytes);

    type ArchivedAlloyBytesRedefined = Vec<u8>;

    // Since AlloyBytesRedefined is essentially a wrapper around Bytes,
    // its Resolver can be unit because we'll directly convert Bytes to Vec<u8> and
    // back.
    pub struct ResolverForAlloyBytesRedefined;

    impl Archive for AlloyBytesRedefined {
        type Archived = ArchivedAlloyBytesRedefined;
        type Resolver = ResolverForAlloyBytesRedefined;

        unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
            *out = self.0.to_vec();
        }
    }

    impl<S: rkyv::ser::Serializer + ?Sized + rkyv::ser::ScratchSpace> rkyv::Serialize<S> for AlloyBytesRedefined {
        fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
            self.0.as_ref().serialize_unsized(serializer)?;
            Ok(ResolverForAlloyBytesRedefined)
        }
    }

    impl<D: Fallible + ?Sized> rDeserialize<AlloyBytesRedefined, D> for Archived<AlloyBytesRedefined> {
        fn deserialize(&self, _: &mut D) -> Result<AlloyBytesRedefined, D::Error> {
            Ok(AlloyBytesRedefined(Bytes::from(self.clone()))) // Convert Vec<u8> back to Bytes
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, Deserialize, rSerialize, rDeserialize, Default, Archive)]
    pub struct BB(Vec<u8>);

    #[derive(Debug, Default, PartialEq, Clone, Serialize, rSerialize, rDeserialize, Archive, Redefined)]
    #[redefined(Log)]
    // #[redefined_attr(
    //     to_source = "Log {address: self.address.into(), data: self.data.into()}",
    //     from_source = "LogRedefined {address: src.address.into(), data:
    // src.data.into()}" )]
    pub struct LogRedefined<T = LogDataRedefined> {
        pub address: AddressRedefined,
        #[serde(flatten)]
        pub data:    T,
    }

    #[derive(Debug, Default, PartialEq, Clone, Serialize, rSerialize, rDeserialize, Archive, Redefined)]
    #[redefined(LogData)]
    #[redefined_attr(to_source = "LogData::new_unchecked(self.topics.to_source(), self.data.to_source())")]
    pub struct LogDataRedefined {
        /// The indexed topic list.
        #[redefined(func = "src.topics().to_vec().into()")]
        topics:   Vec<B256Redefined>,
        /// The plain data.
        pub data: AlloyBytesRedefined,
    }

    #[test]
    fn t() {
        let value: LogRedefined = Default::default();

        // let k: Log = unsafe { std::mem::transmute(t) };

        // let kf = LogDataRedefined::default();
        // let p: LogData = unsafe { std::mem::transmute(kf) };

        let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

        let archived = unsafe { rkyv::archived_root::<LogRedefined>(&bytes[..]) };
        let deserialized: LogRedefined = archived.deserialize(&mut rkyv::Infallible).unwrap();
        assert_eq!(deserialized, value);

        // let k: Log = t.into();
    }

    mod s {

        use std::str::FromStr;

        use alloy_primitives::Address;
        use bytes::Bytes;
        use serde::{
            de::{Deserialize, Deserializer},
            ser::{Serialize, Serializer},
        };

        pub fn serialize<S: Serializer>(u: &Bytes, serializer: S) -> Result<S::Ok, S::Error> {
            format!("{:?}", u.to_vec()).serialize(serializer)
        }

        #[allow(dead_code)]
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
        where
            D: Deserializer<'de>,
        {
            let address: Vec<u8> = Deserialize::deserialize(deserializer)?;

            Ok(Bytes::from(address))
        }
    }
}

// pub struct ArchivedBytes {
//     ptr: RelPtr<[u8]>,
//     len: Archived<usize>,
// }

// pub struct AlloyBytesResolver {
//     pos:               usize,
//     metadata_resolver: MetadataResolver<str>,
// }

// impl Archive for AlloyBytesRedefined {
//     type Archived = ArchivedBytes;
//     type Resolver = AlloyBytesResolver;

//     unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut
// Self::Archived) {         let (fp, fo) = out_field!(out.ptr);
//         self.0
//             .resolve_unsized(pos + fp, resolver.pos,
// resolver.metadata_resolver, fo);     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, Deserialize,
// rSerialize, rDeserialize, Default, Archive, Redefined)]
// #[redefined(Bytes)]
// #[redefined_attr(to_source = "self.0.into()", from_source =
// "Self(src.to_vec())")] #[redefined_attr(transmute)]
// pub struct BytesRedefined {
//     ptr:    *const u8,
//     len:    usize,
//     data:   AtomicPtr<()>,
//     vtable: &'static Vtable,
// }

// pub(crate) struct Vtable {
//     pub clone:  unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
//     pub to_vec: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Vec<u8>,
//     pub drop:   unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
// }
// redefined_remote!(
//     #[derive(Debug, Default, PartialEq, Clone, Serialize, rSerialize,
// rDeserialize, Archive)]     [LogData] : "alloy-primitives"
// );
