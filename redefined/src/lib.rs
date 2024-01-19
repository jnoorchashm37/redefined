#![feature(trivial_bounds)]
#![allow(trivial_bounds)]

#[cfg(test)]
mod tests;

pub use redefined_derive::*;

pub trait RedefinedConvert<O> {
    fn from_source(src: O) -> Self;

    fn to_source(self) -> O;
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
    fn from_source(item: Vec<T>) -> Self {
        item.into_iter().map(|val| F::from_source(val)).collect()
    }

    fn to_source(self) -> Vec<T> {
        self.into_iter().map(|val| val.to_source()).collect()
    }
}

#[macro_export]
macro_rules! self_convert {
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

            self_convert_as_generic_slice!($val);
        )*
    };
}

#[macro_export]
macro_rules! self_convert_as_generic_slice {
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
macro_rules! self_convert_sized {
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

#[macro_export]
macro_rules! self_convert_tuples {
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

self_convert_tuples!();
self_convert_tuples!(T1);
self_convert_tuples!(T1, T2);
self_convert_tuples!(T1, T2, T3);
self_convert_tuples!(T1, T2, T3, T4);
self_convert_tuples!(T1, T2, T3, T4, T5);
self_convert_tuples!(T1, T2, T3, T4, T5, T6);
self_convert_tuples!(T1, T2, T3, T4, T5, T6, T7);
self_convert_tuples!(T1, T2, T3, T4, T5, T6, T7, T8);
self_convert_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
self_convert_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

self_convert!(u8, u16, u32, u64, u128);
self_convert!(i8, i16, i32, i64, i128);
self_convert!(f32, f64);
self_convert!(String, char);
self_convert!(bool);
self_convert_sized!(str);
