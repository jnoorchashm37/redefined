use clickhouse::{fixed_string::FixedString, sql::Identifier};
use redefined_derive::{redefined_remote, Redefined};
use ruint::Uint;

use crate::RedefinedConvert;

/*
pub struct FixedString {
    pub string: String,
}
*/
redefined_remote!(FixedString : "clickhouse");

#[test]
fn test_basic_named_github_remote_type() {
    let fixed_string = FixedString { string: "HI".to_string() };
    let redefined_fixed_string: FixedStringRedefined = fixed_string.clone().into();
    let redefined_fixed_string_to_fixed_string: FixedString = redefined_fixed_string.into();

    assert_eq!(redefined_fixed_string_to_fixed_string, fixed_string);
}

// pub struct Identifier<'a>(pub &'a str);
redefined_remote!(Identifier : "clickhouse");

#[test]
fn test_basic_unnamed_github_remote_type() {
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
fn test_basic_unnamed_crates_io_remote_type() {
    let uint: Uint<256, 4> = Uint::default();
    let redefined_uint: UintRedefined<256, 4> = uint.clone().into();
    let redefined_uint_to_uint = redefined_uint.into();
    assert_eq!(uint, redefined_uint_to_uint);
}
