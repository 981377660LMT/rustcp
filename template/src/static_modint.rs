use std::{
    fmt::{self, Debug, Display, Error},
    marker::PhantomData,
    num::ParseIntError,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
    string::ParseError,
};

use crate::{
    algebraic_structure::*, arithmetic::*, num_gcd::inv_mod, num_integer::Integer,
    num_number::Number,
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
    }
}
pub (crate)use StaticModulusFactoryImpl;
StaticModulusFactoryImpl!(MF998_244_353, u32, 998_244_353, 3);
StaticModulusFactoryImpl!(MF1_000_000_007, u32, 1_000_000_007, 5);
StaticModulusFactoryImpl!(MF1_000_000_009, u32, 1_000_000_009, 13);


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
            Ok(Self::new(x))
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
    #[inline(always)]
    pub fn value(&self) -> T {
        self.v
    }
    #[inline(always)]
    pub fn possible_inv(&self) -> Option<StaticModInt<T, F>> {
        inv_mod(self.v, F::M).map(Self::new)
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
