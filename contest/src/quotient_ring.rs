use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::{algebraic_structure::*, arithmetic::*, num_gcd::inv_mod, num_integer::Integer};

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
pub(crate) use ModulusGenerator;

pub trait Modulus<T>: Clone + Copy
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

#[derive(Clone, Copy)]
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
    pub fn value(&self) -> T {
        self.v
    }
    pub fn possible_inv(&self) -> Option<Quotient<T, M>> {
        match inv_mod(self.v, M::MOD) {
            Some(x) => Some(Self::new(x)),
            None => None,
        }
    }
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

impl<T, M> Quotient<T, M>
where
    T: Integer,
    M: PrimeModulus<T>,
{
    fn mul_inv(&self) -> Self {
        Self::new(inv_mod(self.v, M::MOD).unwrap())
    }
}

impl<T, M> CommutativeAdd for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> AssociativeAdd for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> IdentityAdd for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    const ZERO: Self = Self{v: M::ZERO, phantom: PhantomData};
}

impl<T, M> CommutativeMul for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> AssociativeMul for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
}

impl<T, M> IdentityMul for Quotient<T, M>
where
    T: Integer,
    M: Modulus<T>,
{
    const ONE: Self = Self{v: M::ONE, phantom: PhantomData};
}

impl<T, M> IntegralMul for Quotient<T, M>
where
    T: Integer,
    M: PrimeModulus<T>,
{
}
