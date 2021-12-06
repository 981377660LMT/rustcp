use std::ops::{Add, Div, Mul, Sub};

use crate::algebraic_structure::*;

#[derive(Clone, Copy)]
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

impl Field for Nil {
    fn mul_inv(&self) -> Nil {
        Nil
    }
}
impl IntegralDomain for Nil {}
impl CommutativeRing for Nil {}
impl Ring for Nil {
    fn mul_identity() -> Nil {
        Nil
    }
}
impl AbelianGroup for Nil {}
impl Group for Nil {
    fn add_inv(&self) -> Nil {
        Nil
    }
}
impl Monoid for Nil {
    fn add_identity() -> Nil {
        Nil
    }
}
impl Semigroup for Nil {}
impl Magma for Nil {}
impl PartialEq for Nil {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl Eq for Nil {}
