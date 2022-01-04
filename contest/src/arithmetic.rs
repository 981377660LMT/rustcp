use crate::{num_float::float, num_number::FromNumber};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use crate::{num_integer::Integer, num_number::Number, num_real::Real, macros::should};

///
/// a + b = b + a
///
pub trait CommutativeAdd: Add<Output = Self> + Clone + Debug {}
///
/// (a + b) + c = a + (b + c)
///
pub trait AssociativeAdd: Add<Output = Self> + Clone + Debug {}
///
/// a + ZERO = a, ZERO + a = a
///
pub trait IdentityAdd: Add<Output = Self> + Clone + Debug {
    fn zero() -> Self;
}
///
/// a * b = b * a
///
pub trait CommutativeMul: Mul<Output = Self> + Clone + Debug {}
///
/// (a * b) * c = a * (b * c)
///
pub trait AssociativeMul: Mul<Output = Self> + Clone + Debug {}
///
/// a * ONE = a, ONE * a = a
///
pub trait IdentityMul: Mul<Output = Self> + Clone + Debug {
    fn one() -> Self;
}
///
/// a + a = a
///
pub trait IdempotentAdd: CommutativeAdd + AssociativeAdd {}
///
/// a * a = a
///
pub trait IdempotentMul: CommutativeMul + AssociativeMul {}
///
/// a * b = 0 => a == 0 || b == 0
///
pub trait IntegralMul: Mul<Output = Self> + Clone + Debug {}
impl<T> IntegralMul for T where T: Div<Output = Self> + Mul<Output = Self> + Clone + Debug {}

pub trait PreferDiv: Div<Output = Self> + Clone + Debug {
    fn div_ceil(a: Self, b: Self) -> Self {
        a / b
    }
    fn div_floor(a: Self, b: Self) -> Self {
        a / b
    }
}

impl PreferDiv for f32 {}
impl PreferDiv for f64 {}
impl<T: Integer> PreferDiv for T {
    fn div_ceil(a: Self, b: Self) -> Self {
        should!(b >= Self::ZERO);
        let res = a / b;
        if res * b > a {
            res - Self::ONE
        } else {
            res
        }
    }
    fn div_floor(a: Self, b: Self) -> Self {
        should!(b >= Self::ZERO);
        let res = a / b;
        if res * b < a {
            res + Self::ONE
        } else {
            res
        }
    }
}

pub trait LowerBound: PartialOrd {
    fn min_element() -> Self;
}

pub trait UpperBound: PartialOrd {
    fn max_element() -> Self;
}

pub trait MulInv: Mul + Clone + Debug {
    fn possible_inv(&self) -> Option<Self>;
}

macro_rules! AddTagImpl {
    ($t: ty, $zero: expr) => {
        impl CommutativeAdd for $t {}
        impl IdentityAdd for $t {
            fn zero() -> Self {
                $zero
            }
        }
        impl AssociativeAdd for $t {}
        impl IdempotentAdd for $t {}
    };
}
pub(crate) use AddTagImpl;


macro_rules! MulTagImpl {
    ($t: ty, $one: expr) => {
        impl CommutativeMul for $t {}
        impl IdentityMul for $t {
            fn one() -> Self {
                $one
            }
        }
        impl AssociativeMul for $t {}
        impl IdempotentMul for $t {}
    };
}
pub(crate) use MulTagImpl;
macro_rules! AddMulTagImpl {
    ($t: ty, $zero: expr, $one: expr) => {
        AddTagImpl!($t, $zero);
        MulTagImpl!($t, $one);
    };
}
pub(crate) use AddMulTagImpl;
/**
 * implementation for Number
 */
impl<T> CommutativeAdd for T where T: Number {}
impl<T> IdentityAdd for T
where
    T: Number,
{
    fn zero() -> Self {
        <T as Number>::ZERO
    }
}
impl<T> AssociativeAdd for T where T: Number {}
impl<T> CommutativeMul for T where T: Number {}
impl<T> IdentityMul for T
where
    T: Number,
{
    fn one() -> Self {
        <T as Number>::ONE
    }
}
impl MulInv for f32 {
    fn possible_inv(&self) -> Option<Self> {
        if *self == f32::ZERO {
            None
        } else {
            Some(1f32 / *self)
        }
    }
}
impl MulInv for f64 {
    fn possible_inv(&self) -> Option<Self> {
        if *self == f64::ZERO {
            None
        } else {
            Some(1f64 / *self)
        }
    }
}
impl<T: Real> MulInv for T {
    fn possible_inv(&self) -> Option<Self> {
        if *self == T::zero() {
            None
        } else {
            Some(<T as FromNumber>::from(1) / *self)
        }
    }
}
impl<T> AssociativeMul for T where T: Number {}
impl<T> LowerBound for T
where
    T: Number,
{
    fn min_element() -> Self {
        <T as Number>::MIN
    }
}
impl<T> UpperBound for T
where
    T: Number,
{
    fn max_element() -> Self {
        <T as Number>::MAX
    }
}


///
/// empty arithmetic type, no cost as placeholder
///
#[derive(Clone, Copy, Debug)]
pub struct Nil;
impl Mul for Nil {
    type Output = Nil;

    fn mul(self, rhs: Nil) -> Self::Output {
        Nil
    }
}
impl Add for Nil {
    type Output = Nil;

    fn add(self, rhs: Nil) -> Self::Output {
        Nil
    }
}
impl Sub for Nil {
    type Output = Nil;

    fn sub(self, rhs: Nil) -> Self::Output {
        Nil
    }
}

impl Div for Nil {
    type Output = Nil;

    fn div(self, rhs: Nil) -> Self::Output {
        Nil
    }
}
impl PartialEq for Nil {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl Eq for Nil {}
impl PartialOrd for Nil {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}
impl LowerBound for Nil {
    fn min_element() -> Self {
        Nil
    }
}
impl UpperBound for Nil {
    fn max_element() -> Self {
        Nil
    }
}
AddMulTagImpl!(Nil, Nil, Nil);
