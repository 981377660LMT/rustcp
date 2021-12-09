use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use crate::{num_number::Number, num_real::Real};

///
/// a + b = b + a
///
pub trait CommutativeAdd: Add<Output = Self> + Copy + Debug {}
///
/// (a + b) + c = a + (b + c)
///
pub trait AssociativeAdd: Add<Output = Self> + Copy + Debug {}
///
/// a + ZERO = a, ZERO + a = a
///
pub trait IdentityAdd: Add<Output = Self> + Copy + Debug {
    const ZERO: Self;
}
///
/// a * b = b * a
///
pub trait CommutativeMul: Mul<Output = Self> + Copy + Debug {}
///
/// (a * b) * c = a * (b * c)
///
pub trait AssociativeMul: Mul<Output = Self> + Copy + Debug {}
///
/// a * ONE = a, ONE * a = a
///
pub trait IdentityMul: Mul<Output = Self> + Copy + Debug {
    const ONE: Self;
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
pub trait IntegralMul: Mul<Output = Self> + Copy + Debug {}
macro_rules! AddGenerator {
    ($t: ty, $zero: expr) => {
        impl CommutativeAdd for $t {}
        impl IdentityAdd for $t {
            const ZERO: Self = $zero;
        }
        impl IdempotentAdd for $t {}
        impl AssociativeAdd for $t {}
    };
}
pub(crate) use AddGenerator;
macro_rules! MulGenerator {
    ($t: ty, $one: expr) => {
        impl CommutativeMul for $t {}
        impl IdentityMul for $t {
            const ONE: Self = $one;
        }
        impl IdempotentMul for $t {}
        impl AssociativeMul for $t {}
        impl IntegralMul for $t {}
    };
}
pub(crate) use MulGenerator;
macro_rules! AllGenerator {
    ($t: ty, $zero: expr, $one: expr) => {
        AddGenerator!($t, $zero);
        MulGenerator!($t, $one);
    };
}
pub(crate) use AllGenerator;
/**
 * implementation for Number
 */
impl<T> CommutativeAdd for T where T: Number {}
impl<T> IdentityAdd for T
where
    T: Number,
{
    const ZERO: Self = <T as Number>::ZERO;
}
impl<T> AssociativeAdd for T where T: Number {}
impl<T> CommutativeMul for T where T: Number {}
impl<T> IdentityMul for T
where
    T: Number,
{
    const ONE: Self = <T as Number>::ONE;
}
impl<T> AssociativeMul for T where T: Number {}
impl<T> IntegralMul for T where T: Real {}

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
AllGenerator!(Nil, Nil, Nil);
