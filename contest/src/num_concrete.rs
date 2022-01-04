use std::hash::Hash;

use crate::num_number::Number;

pub trait Concrete: Number + Eq + Ord + Hash {
}

macro_rules! Concrete {
    ($name: ident) => {
        impl Concrete for $name {}
    };
}

Concrete!(i8);
Concrete!(u8);
Concrete!(i16);
Concrete!(u16);
Concrete!(i32);
Concrete!(u32);
Concrete!(i64);
Concrete!(u64);
Concrete!(i128);
Concrete!(u128);
Concrete!(isize);
Concrete!(usize);
