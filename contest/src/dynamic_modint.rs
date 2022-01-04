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

pub struct Modulus<T>
where
    T: Integer,
{
    pub modulus: T,
    pub zero: T,
    pub one: T,
    pub primitive_root: T,
}

impl<T> Modulus<T>
where
    T: Integer,
{
    pub fn set(&mut self, modulus: T, primitive_root: T) {
        self.modulus = modulus;
        self.primitive_root = primitive_root;
        self.zero = T::ZERO;
        self.one = T::ONE % modulus;
    }
    #[inline(always)]
    pub fn add(&self, a: T, b: T) -> T {
        let x = a + b;
        if x >= self.modulus || x < a {
            a + b - self.modulus
        } else {
            a + b
        }
    }
    #[inline(always)]
    pub fn sub(&self, a: T, b: T) -> T {
        if a < b {
            a + b - self.modulus
        } else {
            a - b
        }
    }
    #[inline(always)]
    pub fn mul(&self, a: T, b: T) -> T {
        T::mul_mod(a, b, self.modulus)
    }
    #[inline(always)]
    pub fn div(&self, a: T, b: T) -> T {
        self.mul(a, self.inv(b).unwrap())
    }
    #[inline(always)]
    pub fn inv(&self, a: T) -> Option<T> {
        inv_mod(a, self.modulus)
    }
}

pub trait DynamicModulusFactory<T>: Copy
where
    T: 'static + Integer,
{
    fn modulus() -> &'static mut Modulus<T>;
}

macro_rules! DynamicModulusFactoryImpl {
    ($name: ident, $T: ty) => {
        #[derive(Clone, Copy)]
        pub struct $name;
        impl DynamicModulusFactory<$T> for $name {
            #[inline(always)]
            fn modulus() -> &'static mut Modulus<$T> {
                static mut singleton: Modulus<$T> = Modulus {
                    modulus: <$T as Number>::ZERO,
                    zero: <$T as Number>::ZERO,
                    one: <$T as Number>::ZERO,
                    primitive_root: <$T as Number>::ZERO,
                };
                unsafe { &mut singleton }
            }
        }
    };
}
pub(crate) use DynamicModulusFactoryImpl;
DynamicModulusFactoryImpl!(MF32, u32);
DynamicModulusFactoryImpl!(MF64, u64);

pub struct DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    v: T,
    phantom: PhantomData<F>,
}

impl<T, F> Clone for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T, F> Copy for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
}

impl<T, F> PartialEq for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl<T, F> Eq for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
}

impl<T, F> DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    pub fn new(v: T) -> Self {
        Self {
            v,
            phantom: PhantomData,
        }
    }
}

impl<T, F> Display for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.v, f)
    }
}
impl<T, F> Debug for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.v, f)
    }
}

impl<T, F> Div for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(F::modulus().div(self.v, rhs.v))
    }
}

impl<T, F> Mul for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(T::mul_mod(self.v, rhs.v, F::modulus().modulus))
    }
}

impl<T, F> Sub for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(F::modulus().sub(self.v, rhs.v))
    }
}

impl<T, F> Add for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(F::modulus().add(self.v, rhs.v))
    }
}
impl<T: Integer + 'static, F: DynamicModulusFactory<T>> ModInt<T> for DynamicModInt<T, F> {
    #[inline(always)]
    fn modulus() -> T {
        F::modulus().modulus
    }
    #[inline(always)]
    fn primitive_root() -> Option<Self> {
        Some(Self::new(F::modulus().primitive_root))
    }
    #[inline(always)]
    fn value(&self) -> T {
        self.v
    }

}
impl<T, F> FromNumber for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    fn from(num: impl Number) -> Self {
        Self::new(T::modular(FromNumber::from(num), F::modulus().modulus))
    }
}

impl<T, F> DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    fn mul_inv(&self) -> Self {
        Self::new(F::modulus().inv(self.v).unwrap())
    }
}

impl<T, F> CommutativeAdd for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
}

impl<T, F> AssociativeAdd for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
}

impl<T, F> IdentityAdd for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    fn zero() -> Self {
        Self::new(F::modulus().zero)
    }
}

impl<T, F> CommutativeMul for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
}

impl<T, F> AssociativeMul for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
}

impl<T, F> IdentityMul for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    fn one() -> Self {
        Self::new(F::modulus().one)
    }
}
impl<T: 'static + Integer, F: DynamicModulusFactory<T>> FromStr for DynamicModInt<T, F> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match T::from_str(s) {
            Ok(x) => Ok(FromNumber::from(x)),
            Err(_) => Err(()),
        }
    }
}
impl<T, F> MulInv for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    #[inline(always)]
    fn possible_inv(&self) -> Option<Self> {
        F::modulus().inv(self.v).map(Self::new)
    }
}
impl<T, F> Hash for DynamicModInt<T, F>
where
    T: 'static + Integer,
    F: DynamicModulusFactory<T>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.v.hash(state);
    }
}