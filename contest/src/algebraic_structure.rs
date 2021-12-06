use std::ops::{Add, Div, Mul, Sub};

use crate::{num_integer::Integer, num_number::Number, num_real::Real};

///
/// # Reference
///
/// [magma](https://en.wikipedia.org/wiki/Magma_(algebra))
///
/// 
///
/// for any a, b in S, a + b in S
///
pub trait Magma: Add<Output = Self> + Copy + PartialEq {}
///
/// # Reference
///
/// [semigroup](https://en.wikipedia.org/wiki/Semigroup)
///
/// 
///
/// for any a, b, c in S, (a+b)+c=a+(b+c)
///
pub trait Semigroup: Magma {}

///
/// # Reference
///
/// [monoid](https://en.wikipedia.org/wiki/Monoid)
///
/// 
///
/// there is a identity element 0 in S that for any x in S, x+0=0+x=x
///
pub trait Monoid: Semigroup {
    fn add_identity() -> Self;
}

///
/// # Reference
///
/// [group](https://en.wikipedia.org/wiki/Group_(mathematics)#Definition)
///
/// 
///
/// for any x in S, -x in S is always satified, that x+(-x)=(-x)+x=0
///  
pub trait Group: Monoid + Sub<Output = Self> {
    fn add_inv(&self) -> Self;
}

///
/// # Reference
///
/// [abelian group](https://en.wikipedia.org/wiki/Abelian_group)
///
/// 
///
/// for any a, b in S, a+b=b+a
///  
pub trait AbelianGroup: Group {}

///
/// # Reference
///
/// [ring](https://en.wikipedia.org/wiki/Ring_(mathematics)#Some_properties)
///
/// 
///
/// (S,+) is semigroup and (S,\*) is monoid, and
///
/// - a \* (b + c) = (a \* b) + (a \* c) (left distributivity)
/// - (a + b) \* c = (a \* c) + (b \* c) (right distributivity)
///  
pub trait Ring: AbelianGroup + Mul<Output = Self> {
    fn mul_identity() -> Self;
}
///
/// # Reference
///
/// [commutative ring](https://en.wikipedia.org/wiki/Commutative_ring)
///
/// 
///
/// for any a, b in S, a \* b = b \* a
///  
pub trait CommutativeRing: Ring {}
///
/// # Reference
///
/// [integral domain](https://en.wikipedia.org/wiki/Commutative_ring)
///
/// 
///
/// for any nonzero value a, b in S, a \* b != 0
///
pub trait IntegralDomain: CommutativeRing {}
///
/// # Reference
///
/// [field](https://en.wikipedia.org/wiki/Field_(mathematics))
///
/// 
///
/// for any nonzero value a, b in S, a \* b != 0
///
pub trait Field: IntegralDomain + Div<Output = Self> {
    fn mul_inv(&self) -> Self;
}

/**
 * Implement CommutativeRing for number type
 */
impl<T> CommutativeRing for T where T: Number {}
impl<T> Ring for T
where
    T: Number,
{
    fn mul_identity() -> Self {
        T::ONE
    }
}
impl<T> AbelianGroup for T where T: Number {}
impl<T> Group for T
where
    T: Number,
{
    fn add_inv(&self) -> Self {
        self.negative()
    }
}
impl<T> Monoid for T
where
    T: Number,
{
    fn add_identity() -> Self {
        Self::ZERO
    }
}
impl<T> Semigroup for T where T: Number {}
impl<T> Magma for T where T: Number {}


/**
 * Implement field for real type
 */
impl<T> Field for T 
where T: Real {
    fn mul_inv(&self) -> Self {
        Self::ONE / *self
    }
}
impl<T> IntegralDomain for T 
where T: Real {
}

