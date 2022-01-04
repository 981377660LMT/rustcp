use std::{
    fmt::{self, Debug, Display, Error},
    marker::PhantomData,
    num::ParseIntError,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
    string::ParseError, hash::Hash,
};

use crate::{
    algebraic_structure::*,
    arithmetic::*,
    modint::ModInt,
    num_gcd::inv_mod,
    num_integer::Integer,
    num_number::{FromNumber, Number},
};

pub trait StaticModulusFactory<T> {
    const M: T;
    const ZERO: T;
    const ONE: T;
    const ROOT: T;
}

macro_rules! StaticModulusFactoryImpl {
    ($name: ident, $T: ty, $M: expr, $R: expr) => {
        pub struct $name;
        impl StaticModulusFactory<$T> for $name {
            const M: $T = ($M) as $T;

            const ZERO: $T = <$T>::ZERO;

            const ONE: $T = <$T>::ONE % ($M) as $T;

            const ROOT: $T = ($R) as $T;
        }
    };
}
pub(crate) use StaticModulusFactoryImpl;
//2^26 * x + 1
StaticModulusFactoryImpl!(MF469762049, i32, 469762049, 3);
//2^25 * x + 1
StaticModulusFactoryImpl!(MF167772161, i32, 167772161, 3);
//2^23 * x + 1
StaticModulusFactoryImpl!(MF998244353, i32, 998_244_353, 3);
StaticModulusFactoryImpl!(MF1000000007, i32, 1_000_000_007, 5);
StaticModulusFactoryImpl!(MF1000000009, i32, 1_000_000_009, 13);
//54975513881*2^24+1
StaticModulusFactoryImpl!(MF9223372036737335297, i64, 9223372036737335297, 3);

pub struct StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    v: T,
    phantom: PhantomData<F>,
}

impl<T, F> FromStr for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    type Err = ();
    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = T::from_str(s) {
            Ok(FromNumber::from(x))
        } else {
            Result::Err(())
        }
    }
}

impl<T, F> Clone for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T, F> Copy for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
}

impl<T, F> PartialEq for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl<T, F> Eq for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
}

impl<T: Integer, F: StaticModulusFactory<T>> ModInt<T> for StaticModInt<T, F> {
    #[inline(always)]
    fn modulus() -> T {
        F::M
    }
    #[inline(always)]
    fn primitive_root() -> Option<Self> {
        Some(Self::new(F::ROOT))
    }
    #[inline(always)]
    fn value(&self) -> T {
        self.v
    }

}

impl<T, F> FromNumber for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    fn from(num: impl Number) -> Self {
        Self::new(T::modular(FromNumber::from(num), F::M))
    }
}

impl<T, F> StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    #[inline(always)]
    pub fn new(v: T) -> Self {
        Self {
            v,
            phantom: PhantomData,
        }
    }
}

impl<T, F> Display for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.v, f)
    }
}
impl<T, F> Debug for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.v, f)
    }
}

impl<T, F> Div for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.possible_inv().unwrap()
    }
}

impl<T, F> Mul for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(T::mul_mod(self.v, rhs.v, F::M))
    }
}

impl<T, F> Sub for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(if self.v >= rhs.v {
            self.v - rhs.v
        } else {
            self.v - rhs.v + F::M
        })
    }
}

impl<T, F> Add for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new({
            let res = self.v + rhs.v;
            if res < self.v || res >= F::M {
                res - F::M
            } else {
                res
            }
        })
    }
}

impl<T, F> StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    #[inline(always)]
    fn mul_inv(&self) -> Self {
        self.possible_inv().unwrap()
    }
}

impl<T, F> CommutativeAdd for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
}

impl<T, F> AssociativeAdd for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
}

impl<T, F> IdentityAdd for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    #[inline(always)]
    fn zero() -> Self {
        Self::new(F::ZERO)
    }
}

impl<T, F> CommutativeMul for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
}

impl<T, F> AssociativeMul for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
}

impl<T, F> IdentityMul for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    #[inline(always)]
    fn one() -> Self {
        Self::new(F::ONE)
    }
}
impl<T, F> MulInv for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    #[inline(always)]
    fn possible_inv(&self) -> Option<Self> {
        inv_mod(self.v, F::M).map(Self::new)
    }
}
impl<T, F> Hash for StaticModInt<T, F>
where
    T: 'static + Integer,
    F: StaticModulusFactory<T>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.v.hash(state);
    }
}