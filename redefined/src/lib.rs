#![feature(trivial_bounds)]
#![allow(trivial_bounds)]

use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
};

pub use redefined_derive::{redefined_remote, Redefined};

pub trait RedefinedConvert<O>
where
    O: ?Sized,
{
    fn from_source(src: O) -> Self;

    fn to_source(self) -> O;
}

impl<'a, T> RedefinedConvert<&'a T> for &'a T {
    fn from_source(src: &'a T) -> Self {
        src
    }

    fn to_source(self) -> &'a T {
        self
    }
}

impl<T, F> RedefinedConvert<Option<T>> for Option<F>
where
    F: RedefinedConvert<T>,
{
    fn from_source(item: Option<T>) -> Self {
        item.map(|val| F::from_source(val))
    }

    fn to_source(self) -> Option<T> {
        self.map(|val| val.to_source())
    }
}

impl<T, F> RedefinedConvert<Vec<T>> for Vec<F>
where
    F: RedefinedConvert<T>,
{
    fn from_source(item: Vec<T>) -> Vec<F> {
        item.into_iter()
            .map(|val| F::from_source(val))
            .collect::<Vec<F>>()
    }

    fn to_source(self) -> Vec<T> {
        self.into_iter()
            .map(|val| val.to_source())
            .collect::<Vec<T>>()
    }
}

impl<X, Y, W, Z, S> RedefinedConvert<HashMap<W, Z, S>> for HashMap<X, Y, S>
where
    X: RedefinedConvert<W>,
    Y: RedefinedConvert<Z>,
    X: Hash + Eq,
    W: Hash + Eq,
    S: BuildHasher + Default,
{
    fn from_source(item: HashMap<W, Z, S>) -> Self {
        item.into_iter()
            .map(|(a, b)| (X::from_source(a), Y::from_source(b)))
            .collect()
    }

    fn to_source(self) -> HashMap<W, Z, S> {
        self.into_iter()
            .map(|(a, b)| (a.to_source(), b.to_source()))
            .collect()
    }
}

#[macro_export]
macro_rules! self_convert_redefined_with_fixed_size_array {
    ($($val:ident),*) => {
        $(
            impl RedefinedConvert<$val> for $val {
                fn from_source(item: $val) -> Self {
                    item
                }

                fn to_source(self) -> $val {
                    self
                }
            }

            self_convert_redefined_as_generic_slice!($val);
        )*
    };
}

#[macro_export]
macro_rules! self_convert_redefined {
    ($($val:ident),*) => {
        $(
            impl redefined::RedefinedConvert<$val> for $val {
                fn from_source(item: $val) -> Self {
                    item
                }

                fn to_source(self) -> $val {
                    self
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! self_convert_redefined_as_generic_slice {
    ($val:ident) => {
        impl<const N: usize> RedefinedConvert<[$val; N]> for [$val; N] {
            fn from_source(item: [$val; N]) -> Self {
                item
            }

            fn to_source(self) -> [$val; N] {
                self
            }
        }
    };
}

#[macro_export]
macro_rules! self_convert_redefined_sized {
    ($($val:ident),*) => {
        $(
            impl RedefinedConvert<$val> for $val
            where Self: Sized {
                fn from_source(item: $val) -> Self {
                    item
                }

                fn to_source(self) -> $val {
                    self
                }
            }
        )*
    };
}

self_convert_redefined_with_fixed_size_array!(usize, u8, u16, u32, u64, u128);
self_convert_redefined_with_fixed_size_array!(i8, i16, i32, i64, i128);
self_convert_redefined_with_fixed_size_array!(f32, f64);
self_convert_redefined_with_fixed_size_array!(String, char);
self_convert_redefined_with_fixed_size_array!(bool);

impl<'a> RedefinedConvert<&'a str> for &str {
    fn from_source(src: &'a str) -> Self {
        let ptr = src.as_ptr();
        let len = src.len();

        unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);

            std::str::from_utf8(slice).unwrap()
        }
    }

    fn to_source(self) -> &'a str {
        let ptr = self.as_ptr();
        let len = self.len();

        unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);

            std::str::from_utf8(slice).unwrap()
        }
    }
}

/// 2 tuple
impl<A, B, C, D> RedefinedConvert<(A, B)> for (C, D)
where
    C: RedefinedConvert<A>,
    D: RedefinedConvert<B>,
{
    fn from_source(item: (A, B)) -> Self {
        (C::from_source(item.0), D::from_source(item.1))
    }

    fn to_source(self) -> (A, B) {
        (self.0.to_source(), self.1.to_source())
    }
}

/// all tuple stuff
#[macro_export]
macro_rules! self_convert_redefined_tuples {
    ($($T:ident),*) => {
        impl<$($T: RedefinedConvert<$T>),*> RedefinedConvert<($($T,)*)> for ($($T,)*) {
            fn from_source(item: ($($T,)*)) -> Self {
                item
            }

            fn to_source(self) -> ($($T,)*) {
                self
            }
        }
    };
}

self_convert_redefined_tuples!();
self_convert_redefined_tuples!(T1);
self_convert_redefined_tuples!(T1, T2, T3, T4, T5, T6, T7);
self_convert_redefined_tuples!(T1, T2, T3, T4, T5, T6, T7, T8);
self_convert_redefined_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
self_convert_redefined_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

/// 3 tuple
impl<A, B, C, D, E, F> RedefinedConvert<(A, B, C)> for (D, E, F)
where
    D: RedefinedConvert<A>,
    E: RedefinedConvert<B>,
    F: RedefinedConvert<C>,
{
    fn from_source(item: (A, B, C)) -> Self {
        (D::from_source(item.0), E::from_source(item.1), F::from_source(item.2))
    }

    fn to_source(self) -> (A, B, C) {
        (self.0.to_source(), self.1.to_source(), self.2.to_source())
    }
}

/// 4 tuple
impl<A, B, C, D, E, F, G, H> RedefinedConvert<(A, B, C, D)> for (E, F, G, H)
where
    E: RedefinedConvert<A>,
    F: RedefinedConvert<B>,
    G: RedefinedConvert<C>,
    H: RedefinedConvert<D>,
{
    fn from_source(item: (A, B, C, D)) -> Self {
        (E::from_source(item.0), F::from_source(item.1), G::from_source(item.2), H::from_source(item.3))
    }

    fn to_source(self) -> (A, B, C, D) {
        (self.0.to_source(), self.1.to_source(), self.2.to_source(), self.3.to_source())
    }
}

/// 5 tuple
impl<A, B, C, D, E, F, G, H, I, J> RedefinedConvert<(A, B, C, D, E)> for (F, G, H, I, J)
where
    F: RedefinedConvert<A>,
    G: RedefinedConvert<B>,
    H: RedefinedConvert<C>,
    I: RedefinedConvert<D>,
    J: RedefinedConvert<E>,
{
    fn from_source(item: (A, B, C, D, E)) -> Self {
        (F::from_source(item.0), G::from_source(item.1), H::from_source(item.2), I::from_source(item.3), J::from_source(item.4))
    }

    fn to_source(self) -> (A, B, C, D, E) {
        (self.0.to_source(), self.1.to_source(), self.2.to_source(), self.3.to_source(), self.4.to_source())
    }
}

/// 6 tuple
impl<A, B, C, D, E, F, G, H, I, J, K, L> RedefinedConvert<(A, B, C, D, E, F)> for (G, H, I, J, K, L)
where
    G: RedefinedConvert<A>,
    H: RedefinedConvert<B>,
    I: RedefinedConvert<C>,
    J: RedefinedConvert<D>,
    K: RedefinedConvert<E>,
    L: RedefinedConvert<F>,
{
    fn from_source(item: (A, B, C, D, E, F)) -> Self {
        (
            G::from_source(item.0),
            H::from_source(item.1),
            I::from_source(item.2),
            J::from_source(item.3),
            K::from_source(item.4),
            L::from_source(item.5),
        )
    }

    fn to_source(self) -> (A, B, C, D, E, F) {
        (self.0.to_source(), self.1.to_source(), self.2.to_source(), self.3.to_source(), self.4.to_source(), self.5.to_source())
    }
}
