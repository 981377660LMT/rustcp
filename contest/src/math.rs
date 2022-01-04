use std::cmp::Ordering;

use crate::{
    algebraic_structure::{Field, Ring},
    arithmetic::IdentityMul,
    binary_search::{first_true, last_true},
    num_gcd::gcd,
    num_integer::Integer,
    num_number::FromNumber,
};

///
///
///
/// x^n
///
/// # Example
///
/// ```
/// use template::{algebraic_structure::Ring, num_integer::Integer, math::pow};
/// assert_eq!(pow(5, 2), 25);
/// assert_eq!(pow(0, 0), 1);
/// assert_eq!(pow(2, 5), 32);
/// ```
///
pub fn pow<T, E>(x: T, n: E) -> T
where
    T: Ring,
    E: Integer,
{
    if n == E::ZERO {
        return <T as IdentityMul>::one();
    }
    let ans = pow(x, n >> E::ONE);
    let ans = ans * ans;
    if (n & E::ONE) == E::ONE {
        ans * x
    } else {
        ans
    }
}

///
/// find maximum t that 2^t <= x, -1 if x == 0
///
/// # Example
///
/// ```
/// use template::math::*;
/// assert_eq!(0, log2_floor(1));
/// assert_eq!(-1, log2_floor(0));
/// assert_eq!(1, log2_floor(2));
/// assert_eq!(1, log2_floor(3));
/// ```
///
pub fn log2_floor<T>(x: T) -> i32
where
    T: Integer,
{
    let leading_zero = x.count_leading_zero();
    T::BITS - leading_zero - 1
}
///
/// find minimum t that 2^t >= x
///
/// # Example
///
/// ```
/// use template::math::*;
/// assert_eq!(0, log2_ceil(1));
/// assert_eq!(0, log2_ceil(0));
/// assert_eq!(1, log2_ceil(2));
/// assert_eq!(2, log2_ceil(3));
/// ```
///
pub fn log2_ceil<T>(x: T) -> i32
where
    T: Integer,
{
    let res = log2_floor(x);
    if res < 0 || (T::ONE << FromNumber::from(res)) < x {
        res + 1
    } else {
        res
    }
}

pub fn argmax<'a, T: Ord>(data: &'a [T]) -> Option<(usize, &'a T)> {
    data.iter().enumerate().max_by_key(|(_, x)| *x)
}

pub fn argmin<'a, T: Ord>(data: &'a [T]) -> Option<(usize, &'a T)> {
    data.iter().enumerate().min_by_key(|(_, x)| *x)
}

pub fn argmax_by<'a, T>(
    data: &'a [T],
    f: &mut impl FnMut(&T, &T) -> Ordering,
) -> Option<(usize, &'a T)> {
    data.iter().enumerate().max_by(|(_, x), (_, y)| f(x, y))
}

pub fn argmin_by<'a, T>(
    data: &'a [T],
    f: &mut impl FnMut(&T, &T) -> Ordering,
) -> Option<(usize, &'a T)> {
    data.iter().enumerate().min_by(|(_, x), (_, y)| f(x, y))
}

pub fn sqrt_floor<T: Integer>(x: T) -> Option<T> {
    if x < T::ZERO {
        None
    } else {
        let x = x.as_unsigned();
        let limit: T = <T as FromNumber>::from(1) << <T as FromNumber>::from(T::BITS / 2);
        last_true(T::ZERO, limit - T::ONE, |t| (*t * *t).as_unsigned() <= x)
    }
}

pub fn sqrt_ceil<T: Integer>(x: T) -> Option<T> {
    if x < T::ZERO {
        None
    } else {
        let x = x.as_unsigned();
        let limit: T = <T as FromNumber>::from(1) << FromNumber::from(T::BITS / 2);
        match first_true(T::ZERO, limit - T::ONE, |t| (*t * *t).as_unsigned() >= x) {
            None => Some(limit + T::ONE),
            Some(x) => Some(x),
        }
    }
}

pub fn inverse_batch<T: Field>(mut data: &[T]) -> Vec<T> {
    if data.is_empty() {
        return Vec::new();
    }
    let n = data.len();
    let mut res = data.to_owned();
    for i in 1..n {
        res[i] = res[i - 1] * res[i];
    }
    let mut inv = T::one() / res[n - 1];
    for i in (1..n).rev() {
        res[i] = inv * res[i - 1];
        inv = inv * data[i];
    }
    res[0] = inv;
    res
}

pub fn max_batch<'a, T: Ord>(a: &'a [T]) -> Option<&'a T> {
    a.iter().max()
}
pub fn min_batch<'a, T: Ord>(a: &'a [T]) -> Option<&'a T> {
    a.iter().min()
}
pub fn dot_mul<T: Ring>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    a.iter().zip(b.iter()).map(|(x, y)| *x * *y).collect()
}
pub fn dot_mul_plus<T: Ring>(a: &Vec<T>, b: &Vec<T>, dest: &mut Vec<T>) {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| *x * *y)
        .enumerate()
        .for_each(|(index, v)| dest[index] = dest[index] + v);
}
