use redefined_derive::Redefined;
use redefined_outside_crate_tests::{GenericStructA, NonPubFieldStructA};

use crate::RedefinedConvert;

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(NonPubFieldStructA, to_source = "NonPubFieldStructA::new(self.p, self.d, self.vals)")]
pub struct NonPubFieldStructB {
    #[redefined(func = "get_p")]
    pub p:    u64,
    pub d:    u64,
    pub vals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Redefined)]
#[redefined(GenericStructA, generics(K, Z))]
pub struct GenericStructB<X, Y>
where
    X: RedefinedConvert<X>,
    Y: RedefinedConvert<Y>,
{
    pub p:    u64,
    pub d:    X,
    pub vals: Vec<Y>,
}

#[test]
fn test_generics() {
    let struct_a = GenericStructA::default();
    let struct_b = GenericStructB::from_source(struct_a.clone());
    let struct_b_to_a: GenericStructA<String, u64> = struct_b.to_source();
    assert_eq!(struct_b_to_a, struct_a);
}
