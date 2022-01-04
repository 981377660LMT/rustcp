use std::{fmt::Debug, ops::*};

use crate::{algebraic_structure::CommutativeRing, segtree::Segtree, arithmetic::{IdentityAdd, AssociativeAdd}};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S<T: CommutativeRing>(T, T);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct U<T: CommutativeRing>(T, T);
impl<T: CommutativeRing> Add<S<T>> for S<T> {
    type Output = S<T>;

    fn add(self, y: S<T>) -> Self::Output {
        let x = self;
        S(x.0 + y.0, x.1 + y.1)
    }
}
impl<T: CommutativeRing> IdentityAdd for S<T> {
    fn zero() -> Self {
        S(T::zero(), T::zero())
    }
}
impl<T: CommutativeRing> Add<U<T>> for S<T> {
    type Output = S<T>;

    fn add(self, y: U<T>) -> Self::Output {
        let x = self;
        S(y.0 * x.0 + x.1 * y.1, x.1)
    }
}
impl<T: CommutativeRing> AssociativeAdd for S<T> {
}
impl<T: CommutativeRing> Add<U<T>> for U<T> {
    type Output = U<T>;

    fn add(self, y: U<T>) -> Self::Output {
        let x = self;
        U(y.0 * x.0, y.1 + x.1 * y.0)
    }
}
impl<T: CommutativeRing> IdentityAdd for U<T> {
    fn zero() -> Self {
        U(T::one(), T::zero())
    }
}
impl<T: CommutativeRing> AssociativeAdd for U<T> {
}
pub struct RangeAffineRangeSum<T>
where
    T: CommutativeRing,
{
    st: Segtree<S<T>, U<T>>,
}
impl<T> Debug for RangeAffineRangeSum<T>
where
    T: CommutativeRing,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.st.fmt(f)
    }
}

///
/// Range affine range sum
///
/// # Example
///
/// ```
/// use template::range_affine_range_sum::*;
/// let mut rars = RangeAffineRangeSum::new(1, 10, &|x| x);
/// assert_eq!(55, rars.query(1, 10));
/// assert_eq!(10, rars.query(1, 4));
/// rars.update(1, 5, 1, 1);
/// assert_eq!(60, rars.query(1, 10));
/// rars.update(6, 10, 2, 0);
/// assert_eq!(100, rars.query(1, 10));
/// ```
///
impl<T> RangeAffineRangeSum<T>
where
    T: CommutativeRing,
{
    pub fn new(l: usize, r: usize, f: impl Fn(usize) -> T) -> Self {
        Self {
            st: Segtree::new(
                l,
                r,
                |x| S(f(x), T::one()),
            ),
        }
    }
    pub fn query(&mut self, l: usize, r: usize) -> T {
        self.st.query(l, r).0
    }

    pub fn update(&mut self, l: usize, r: usize, a: T, b: T) {
        self.st.update(l, r, U(a, b))
    }
}
