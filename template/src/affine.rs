use std::ops::{Add, Sub, Mul};

use crate::{algebraic_structure::CommutativeRing, arithmetic::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Affine<T>(pub T, pub T) where T: CommutativeRing;



impl<T> Affine<T> where T: CommutativeRing {
    pub fn f(&self, x: &T) -> T {
        self.0 * *x + self.1
    }
}

impl<T> Add for Affine<T>where T: CommutativeRing {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Affine(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Sub for Affine<T>where T: CommutativeRing {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Affine(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Mul for Affine<T> where T: CommutativeRing {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Affine(self.0 * rhs.0, self.0 * rhs.1 + self.1)
    }
}

impl<T> AssociativeAdd for Affine<T> where T: CommutativeRing {}
impl<T> CommutativeAdd for Affine<T> where T: CommutativeRing {}
impl<T> IdentityAdd for Affine<T> where T: CommutativeRing {
    fn zero() -> Self {
        Self(<T as IdentityAdd>::zero(), <T as IdentityAdd>::zero())
    }
}
impl<T> AssociativeMul for Affine<T> where T: CommutativeRing {}
impl<T> IdentityMul for Affine<T> where T: CommutativeRing {
    fn one() -> Self {
        Self(<T as IdentityMul>::one(), <T as IdentityAdd>::zero())
    }
}