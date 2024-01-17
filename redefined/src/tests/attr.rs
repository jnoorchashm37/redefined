use redefined_derive::Redefined;
use redefined_outside_crate_tests::NonPubFieldStructA;

use crate::RedefinedConvert;

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(NonPubFieldStructA, to_source = "NonPubFieldStructA::new(self.p, self.d, self.vals)")]
pub struct NonPubFieldStructB {
    #[redefined(func = "get_p")]
    pub p:    u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

#[test]
fn test_struct() {
    let struct_a = NonPubFieldStructA::default();
    let struct_b = NonPubFieldStructB::from_source(struct_a.clone());
    let struct_b_to_a: NonPubFieldStructA = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}
