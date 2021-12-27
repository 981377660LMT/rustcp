use std::cmp::Ordering;

use crate::{algebraic_structure::Ring, num_integer::Integer, arithmetic::IdentityMul};

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
where T: Ring,
    E: Integer {
    if n == E::ZERO {
        return <T as IdentityMul>::one();
    }
    let ans = pow(x, n >> E::ONE);
    let ans = ans * ans;
    if (n & E::ONE) == E::ONE {
        ans * x
    }else{
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
where T: Integer {
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
where T: Integer{
    let res = log2_floor(x);
    if res < 0 || (T::ONE << T::from_i32(res)) < x {
        res + 1
    } else {
        res
    }
}

pub fn unsafe_min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}


pub fn unsafe_max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}
