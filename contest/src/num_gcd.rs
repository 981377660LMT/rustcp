use crate::num_number::{Number, FromNumber};
use crate::{
    macros::{should, should_eq},
    num_integer::Integer,
};

///
///
///
/// Get the greatest common divisor of a and b where a,b >= 0
///
/// # Example
///
/// ```
/// use template::num_gcd::gcd;
/// assert_eq!(gcd(2, 3), 1);
/// assert_eq!(gcd(6, 9), 3);
/// ```
///
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Integer,
{
    should! {a.is_non_negative(), b.is_non_negative()};
    let mut s = (a, b);
    while s.1 > T::ZERO {
        s = (s.1, s.0 % s.1);
    }
    s.0
}

///
///
///
/// find a equation for ax+by=gcd(a,b) where a,b >= 0
///
/// Result: (x, y, gcd(a, b))
///
pub fn extgcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Integer,
{
    should!(a.is_non_negative(), b.is_non_negative());
    if b == T::ZERO {
        (T::ONE, T::ZERO, a)
    } else {
        let div_and_rem = T::div_and_remainder(a, b);
        let ans = extgcd(b, div_and_rem.1);
        (ans.1, ans.0 - div_and_rem.0 * ans.1, ans.2)
    }
}

///
///
///
/// Find ax=1(mod m)
///
/// Result: x
///
/// # Example
///
/// ```
/// use template::num_gcd::inv_mod;
/// assert_eq!(inv_mod(1, 2).unwrap(), 1);
/// assert_eq!(inv_mod(2, 3).unwrap(), 2);
/// assert!(inv_mod(2, 4).is_none());
/// ```
///
pub fn inv_mod<T>(a: T, m: T) -> Option<T>
where
    T: Integer,
{
    let a1: T::SignedIntegerType = FromNumber::from(a);
    let m1: T::SignedIntegerType = FromNumber::from(m);
    let res = extgcd(a1, m1);
    if res.2 == T::SignedIntegerType::ONE {
        let res = T::from(T::SignedIntegerType::modular(
            res.0,
            m1,
        ));
        should_eq!(T::mul_mod(res, a, m), T::ONE);
        Some(res)
    } else {
        None
    }
}
