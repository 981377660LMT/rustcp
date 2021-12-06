use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::{
    algebraic_structure::*,
    num_gcd::inv_mod,
    num_integer::Integer,
    num_number::Number,
};

macro_rules! ModulusGenerator {
    ($n: ident, $t: ty, $m: expr, $r: expr) => {
        ModulusGenerator!($t, $m);
        impl PrimeModulus for $n {
            const PRIME_ROOT: $t = $r as $t;
        }
    };
    ($n: ident, $t: ty, $m: expr) => {
        struct $n;
        impl Modulus<$t> for $n {
            const MOD: $t = $m as $t;
            const ZERO: $t = <$t as Number>::ZERO;
            const ONE: $t = <$t as Number>::ONE % $m as $t;
        }
    };
}

pub trait Modulus<T>
where
    T: Integer,
{
    const MOD: T;
    const ZERO: T;
    const ONE: T;
}

pub trait PrimeModulus<T>: Modulus<T>
where
    T: Integer,
{
    const PRIME_ROOT: T;
}

struct Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    v: T,
    phantom: PhantomData<M>,
}

impl<T, M> Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn new(v: T) -> Self {
        Self {
            v,
            phantom: PhantomData,
        }
    }
    fn value(&self) -> T {
        self.v
    }
    fn possible_inv(&self) -> Option<Quotient<T, M>> {
        match inv_mod(self.v, M::MOD) {
            Some(x) => Some(Self::new(x)),
            None => None,
        }
    }
}

impl<T, M> Clone for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            phantom: self.phantom.clone(),
        }
    }
}

impl<T, M> Copy for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> Display for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.v, f)
    }
}
impl<T, M> Debug for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.v, f)
    }
}
impl<T, M> PartialEq for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl<T, M> Div for Quotient<T, M>
where
    T: Integer,
    M: PrimeModulus<T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(T::mul_mod(self.v, rhs.mul_inv().v, M::MOD))
    }
}

impl<T, M> Mul for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(T::mul_mod(self.v, rhs.v, M::MOD))
    }
}

impl<T, M> Sub for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.v - rhs.v;
        if r < M::MOD {
            Self::new(r - M::MOD)
        } else {
            Self::new(r)
        }
    }
}

impl<T, M> Add for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.v + rhs.v;
        if r >= M::MOD {
            Self::new(r - M::MOD)
        } else {
            Self::new(r)
        }
    }
}

impl<T, M> Magma for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}
impl<T, M> Semigroup for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> Monoid for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn add_identity() -> Self {
        Self::new(M::ZERO)
    }
}

impl<T, M> Group for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn add_inv(&self) -> Self {
        if (self.v == M::ZERO) {
            Self::new(M::ZERO)
        } else {
            Self::new(M::MOD - self.v)
        }
    }
}

impl<T, M> AbelianGroup for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> Ring for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    fn mul_identity() -> Self {
        Self::new(M::ONE)
    }
}

impl<T, M> CommutativeRing for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> IntegralDomain for Quotient<T, M>
where
    T: Integer,
    M: PrimeModulus<T>,
{
}

impl<T, M> Field for Quotient<T, M>
where
    T: Integer,
    M: PrimeModulus<T>,
{
    fn mul_inv(&self) -> Self {
        Self::new(inv_mod(self.v, M::MOD).unwrap())
    }
}
