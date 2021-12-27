use std::ops::{Add, Div, Mul, Sub};

use crate::{
    arithmetic::*
};
///
/// for any a, b in S, a + b in S
///
/// # Reference
///
/// [magma](https://en.wikipedia.org/wiki/Magma_(algebra))
///
pub trait Magma: Add<Output = Self> + Copy + Eq {}
impl<T> Magma for T 
where T: Add<Output = Self> + Copy + Eq {}
///
/// for any a, b, c in S, (a+b)+c=a+(b+c)
///
/// # Reference
///
/// [semigroup](https://en.wikipedia.org/wiki/Semigroup)
///
///
pub trait Semigroup: Magma + AssociativeAdd {}
impl<T> Semigroup for T 
where T: Magma + AssociativeAdd{}
///
///
/// there is a identity element 0 in S that for any x in S, x+0=0+x=x
///
/// # Reference
///
/// [monoid](https://en.wikipedia.org/wiki/Monoid)
///
pub trait Monoid: Semigroup + IdentityAdd {}
impl<T> Monoid for T 
where T: Semigroup + IdentityAdd{}
///
/// for any x in S, -x in S is always satified, that x+(-x)=(-x)+x=0
///
/// # Reference
///
/// [group](https://en.wikipedia.org/wiki/Group_(mathematics)#Definition)
pub trait Group: Monoid + Sub<Output = Self> {}
impl<T> Group for T 
where T: Monoid + Sub<Output = Self>{}
///
///
/// for any a, b in S, a+b=b+a
///
/// # Reference
///
/// [abelian group](https://en.wikipedia.org/wiki/Abelian_group)
///
pub trait AbelianGroup: Group + CommutativeAdd {}
impl<T> AbelianGroup for T 
where T: Group + CommutativeAdd {}
///
/// (S,+) is semigroup and (S,\*) is monoid, and
///
/// - a \* (b + c) = (a \* b) + (a \* c) (left distributivity)
/// - (a + b) \* c = (a \* c) + (b \* c) (right distributivity)
///
/// # Reference
///
/// [ring](https://en.wikipedia.org/wiki/Ring_(mathematics)#Some_properties)
///
pub trait Ring: AbelianGroup + Mul<Output = Self> + IdentityMul {}
impl<T> Ring for T 
where T: AbelianGroup + Mul<Output = Self> + IdentityMul {}
///
/// for any a, b in S, a \* b = b \* a
///
/// # Reference
///
/// [commutative ring](https://en.wikipedia.org/wiki/Commutative_ring)
///  
pub trait CommutativeRing: Ring + CommutativeMul {}
impl<T> CommutativeRing for T 
where T: Ring + CommutativeMul {}
///
/// for any nonzero value a, b in S, a \* b != 0
///
/// # Reference
///
/// [integral domain](https://en.wikipedia.org/wiki/Commutative_ring)
///
pub trait IntegralDomain: CommutativeRing + IntegralMul {}
impl<T> IntegralDomain for T 
where T: CommutativeRing + IntegralMul {}
///
/// for any nonzero value x in S, there is a number y that x * y = 1 and y * x = 1
///
/// # Reference
///
/// [field](https://en.wikipedia.org/wiki/Field_(mathematics))
///
pub trait Field: IntegralDomain + Div<Output = Self> {}
impl<T> Field for T 
where T: IntegralDomain + Div<Output = Self> {}