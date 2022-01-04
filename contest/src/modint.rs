use std::{str::FromStr, string::ParseError, hash::Hash};

use crate::{num_integer::Integer, algebraic_structure::CommutativeRing, num_number::{Number, FromNumber}, arithmetic::MulInv};

pub trait ModInt<T: Integer>: CommutativeRing + FromNumber + MulInv + Hash {
    fn modulus() -> T;
    fn primitive_root() -> Option<Self>;
    fn value(&self) -> T;
}
pub fn modint_sum_batch<I: Integer, T: ModInt<I>>(a: &[T], b: &[T]) -> T {
    let modulus = FromNumber::from(T::modulus());
    let max_allow = I::HighPrecisionIntegerType::MAX - (modulus - I::HighPrecisionIntegerType::ONE) * (modulus - I::HighPrecisionIntegerType::ONE);
    let mut sum = I::HighPrecisionIntegerType::ZERO;
    a.iter().zip(b.iter()).for_each(|(x, y)| {
        let prod = <I::HighPrecisionIntegerType as FromNumber>::from(x.value()) * 
            <I::HighPrecisionIntegerType as FromNumber>::from(y.value());
        sum = sum + prod;
        if sum > max_allow {
            sum = sum % modulus;
        }
    });
    sum = sum % modulus;
    FromNumber::from(sum)
}