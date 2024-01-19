use redefined_derive::Redefined;
use redefined_outside_crate_tests::NonPubFieldStructA;

use crate::RedefinedConvert;

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(NonPubFieldStructA)]
#[redefined_attr(to_source = "NonPubFieldStructA::new(self.p, self.d, self.vals)")]
pub struct NonPubFieldStructB {
    #[redefined_attr(func = "get_p")]
    pub p:    u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

#[test]
fn test_struct_non_pub_and_new_source_fn_field() {
    let struct_a = NonPubFieldStructA::default();
    let struct_b = NonPubFieldStructB::from_source(struct_a.clone());
    let struct_b_to_a: NonPubFieldStructA = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}
