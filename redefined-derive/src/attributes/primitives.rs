use proc_macro2::Ident;

pub struct Primitive(&'static str);

impl PartialEq<Primitive> for Ident {
    fn eq(&self, word: &Primitive) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Primitive> for &'a Ident {
    fn eq(&self, word: &Primitive) -> bool {
        *self == word.0
    }
}

impl<'a> From<Ident> for Primitive {
    fn from(value: Ident) -> Self {
        (value.to_string().as_str()).into()
    }
}

macro_rules! primitive {
    ($($v:ident),*) => {
        $(
            pub const $v: Primitive = Primitive(stringify!($v));
        )*

        impl From<&str> for Primitive {
            fn from(value: &str) -> Self {
                match value {
                    $(
                        stringify!($v) => $v,
                    ) *
                    _ => panic!("No primitive called {}", value),
                }
            }
        }

        impl Primitive {
            pub fn is_primitive(ident: &Ident) -> bool {
                match ident.to_string().as_str() {
                    $(
                        stringify!($v) => true,
                    ) *
                    _ => false
                }
            }
        }

    };
}

primitive!(Vec);

pub fn is_simple_primitive(val: &str) -> bool {
    match val {
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "f32" | "f64" | "str" | "bool"
        | "String" | "char" => true,
        _ => false,
    }
}
