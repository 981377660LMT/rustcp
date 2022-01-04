use std::ops::{Add, Sub, Mul, Div};

use crate::{algebraic_structure::{CommutativeRing, Field}, num_real::Real, num_number::FromNumber, arithmetic::{AssociativeAdd, AssociativeMul, IdentityAdd, IdentityMul, CommutativeAdd, CommutativeMul, MulInv}};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Complex<T: CommutativeRing>(pub T, pub T);

impl<T: CommutativeRing> Eq for Complex<T> {
}
impl<T: CommutativeRing> AssociativeAdd for Complex<T> {
}
impl<T: CommutativeRing> AssociativeMul for Complex<T> {
}
impl<T: CommutativeRing> CommutativeAdd for Complex<T> {
}
impl<T: CommutativeRing> CommutativeMul for Complex<T> {
}
impl<T: CommutativeRing> IdentityAdd for Complex<T> {
    fn zero() -> Self {
        Self::with_real(T::zero())
    }
}
impl<T: CommutativeRing> IdentityMul for Complex<T> {
    fn one() -> Self {
        Self::with_real(T::one())
    }
}


impl<T: CommutativeRing> Complex<T> {
    pub fn with_real(real: T) -> Self {
        Self(real, T::zero())    
    }
    pub fn new(real: T, image: T) -> Self {
        Self(real, image)
    }
    pub fn real(&self) -> T{
        self.0
    }
    pub fn image(&self) -> T{
        self.1
    }
    pub fn conj(&self) -> Self {
        Self(self.0, T::zero() - self.1)
    }
    pub fn norm2(&self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl<T: Real> Complex<T> {
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }
}

impl<T: CommutativeRing> Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: CommutativeRing> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: CommutativeRing> Mul for Complex<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
    }
}
impl<T: CommutativeRing> Mul<T> for Complex<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
impl<T: Field> Div<T> for Complex<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: Field + FromNumber> Div for Complex<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.conj() / rhs.norm2()
    }
}

impl<T: CommutativeRing + FromNumber> FromNumber for Complex<T> {
    fn from(num: impl crate::num_number::Number) -> Self {
        Self::with_real(FromNumber::from(num))
    }
}


impl<T: Field + FromNumber> MulInv for Complex<T> {
    fn possible_inv(&self) -> Option<Self> {
        if *self == Self::zero() {
            None
        } else {
            Some(Self::with_real(FromNumber::from(1.0)) / *self)
        }
    }
} 