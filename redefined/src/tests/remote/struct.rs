use clickhouse::{fixed_string::FixedString, sql::Identifier};
use redefined_derive::{redefined_remote, Redefined};

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
    let fixed_string_to_redefined_fixed_string: FixedString = redefined_fixed_string.into();

    assert_eq!(fixed_string_to_redefined_fixed_string, fixed_string);
}

// pub struct Identifier<'a>(pub &'a str);
redefined_remote!(Identifier : "clickhouse");

#[test]
fn test_basic_unnamed_github_remote_type() {
    let identifier = Identifier("HI");
    let initial = identifier.0;

    let redefined_identifier: IdentifierRedefined = identifier.into();
    let identifier_to_redefined_identifier: Identifier = redefined_identifier.into();
    let converted = identifier_to_redefined_identifier.0;

    assert_eq!(initial, converted);
}
