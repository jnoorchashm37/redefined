use clickhouse::{sql::Identifier, Compression};
use redefined_derive::{redefined_remote, Redefined};
use ruint::Uint;

use crate::RedefinedConvert;

/*
pub enum Compression {
    None,
    Lz4,
    Lz4Hc(i32),
}
*/
redefined_remote!(Compression : "clickhouse");

#[test]
fn test_basic_named_github_remote_enum() {
    let compression = Compression::Lz4Hc(100);
    let redefined_compression: CompressionRedefined = compression.clone().into();
    let redefined_compression_to_compression: Compression = redefined_compression.into();

    assert_eq!(redefined_compression_to_compression, compression);
}

// pub struct Identifier<'a>(pub &'a str);
redefined_remote!(Identifier : "clickhouse");

#[test]
fn test_basic_unnamed_github_remote_enum() {
    let identifier = Identifier("HI");
    let initial = identifier.0;

    let redefined_identifier: IdentifierRedefined = identifier.into();
    let redefined_identifier_to_identifier: Identifier = redefined_identifier.into();
    let converted = redefined_identifier_to_identifier.0;

    assert_eq!(initial, converted);
}

/*
pub struct Uint<const BITS: usize, const LIMBS: usize> {
    limbs: [u64; LIMBS],
}
*/
redefined_remote!(Uint : "ruint");

#[test]
fn test_basic_unnamed_crates_io_remote_enum() {
    let uint: Uint<256, 4> = Uint::default();
    let redefined_uint: UintRedefined<256, 4> = uint.clone().into();
    let redefined_uint_to_uint = redefined_uint.into();
    assert_eq!(uint, redefined_uint_to_uint);
}
