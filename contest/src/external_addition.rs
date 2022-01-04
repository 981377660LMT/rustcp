use std::{
    cmp::{max, min},
    fmt::Debug,
    marker::PhantomData,
    ops::Add,
};

use crate::arithmetic::{AssociativeAdd, CommutativeAdd, IdempotentAdd, IdentityAdd};

pub trait Addition<T>: Copy + Debug
where
    T: Copy + Debug,
{
    fn add(a: T, b: T) -> T;
}
pub trait IdentityAddition<T>: Addition<T>
where
    T: Copy + Debug,
{
    const ZERO: T;
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wrapper<T, F>
where
    T: Copy + Debug + PartialEq + Eq,
    F: Addition<T>,
{
    v: T,
    f: PhantomData<F>,
}
impl<T, F> Wrapper<T, F>
where
    T: Copy + Debug + PartialEq + Eq,
    F: Addition<T>,
{
    pub fn new(v: T) -> Self {
        Self { v, f: PhantomData }
    }
    pub fn value_ref<'a>(&'a self) -> &'a T {
        &self.v
    }
    pub fn value_mut_ref<'a>(&'a mut self) -> &'a mut T {
        &mut self.v
    }
    pub fn value(&self) -> T {
        self.v
    }
}

impl<T, F> Debug for Wrapper<T, F>
where
    T: Copy + Debug + PartialEq + Eq,
    F: Addition<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}
impl<T, F> Add for Wrapper<T, F>
where
    T: Copy + Debug + PartialEq + Eq,
    F: Addition<T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Wrapper::new(F::add(self.v, rhs.v))
    }
}

impl<T, F> IdentityAdd for Wrapper<T, F>
where
    T: Copy + Debug + PartialEq + Eq,
    F: IdentityAddition<T>,
{
    fn zero() -> Self {
        Self {
            v: <F as IdentityAddition<T>>::ZERO,
            f: PhantomData,
        };
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MinimumAddition<T>
where
    T: Ord + Copy + Debug,
{
    phantom: PhantomData<T>,
}

impl<T> Addition<T> for MinimumAddition<T>
where
    T: Ord + Copy + Debug,
{
    fn add(a: T, b: T) -> T {
        min(a, b)
    }
}
impl<T> IdempotentAdd for Wrapper<T, MinimumAddition<T>> where T: Ord + Copy + Debug {}
impl<T> CommutativeAdd for Wrapper<T, MinimumAddition<T>> where T: Ord + Copy + Debug {}
impl<T> AssociativeAdd for Wrapper<T, MinimumAddition<T>> where T: Ord + Copy + Debug {}

#[derive(Clone, Copy, Debug)]
pub struct MaximumAddition<T>
where
    T: Ord + Copy + Debug,
{
    phantom: PhantomData<T>,
}
impl<T> Addition<T> for MaximumAddition<T>
where
    T: Ord + Copy + Debug,
{
    fn add(a: T, b: T) -> T {
        max(a, b)
    }
}
impl<T> IdempotentAdd for Wrapper<T, MaximumAddition<T>> where T: Ord + Copy + Debug {}
impl<T> CommutativeAdd for Wrapper<T, MaximumAddition<T>> where T: Ord + Copy + Debug {}
impl<T> AssociativeAdd for Wrapper<T, MaximumAddition<T>> where T: Ord + Copy + Debug {}
