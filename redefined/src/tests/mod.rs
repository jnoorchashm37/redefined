mod derive;
#[cfg(feature = "remote")]
mod remote;

#[macro_export]
macro_rules! struct_test {
    ($target_struct:ident, $source_struct:ident) => {
        paste::paste! {
            #[test]
            fn [<test_struct $target_struct:lower>]() {
                let struct_a = $source_struct::default();
                let struct_b: $target_struct = struct_a.clone().into();
                let struct_b_to_a: $source_struct = struct_b.into();
                assert_eq!(struct_b_to_a, struct_a);
            }
        }
    };

    ($tag:ident, $target_struct:ident, $source_struct:ident) => {
        paste::paste! {
            #[test]
            fn [<test_struct $target_struct:lower _ $tag>]() {
                let struct_a = $source_struct::default();
                let struct_b: $target_struct = struct_a.clone().into();
                let struct_b_to_a: $source_struct = struct_b.into();
                assert_eq!(struct_b_to_a, struct_a);
            }
        }
    };

    ($tag:ident, $target_struct:ident, $source_struct:ident, $fn:block) => {
        paste::paste! {
            #[test]
            fn [<test_struct $target_struct:lower _ $tag>]() {
                let struct_a = $fn;
                let struct_b: $target_struct = struct_a.clone().into();
                let struct_b_to_a: $source_struct = struct_b.into();
                assert_eq!(struct_b_to_a, struct_a);
            }
        }
    };

    ($target_struct:ident, $source_struct:ident, $fn:block) => {
        paste::paste! {
            #[test]
            fn [<test_struct $target_struct:lower>]() {
                let struct_a = $fn;
                let struct_b: $target_struct = struct_a.clone().into();
                let struct_b_to_a: $source_struct = struct_b.into();
                assert_eq!(struct_b_to_a, struct_a);
            }
        }
    };
}
