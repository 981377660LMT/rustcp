use std::{
    cmp::{max, min},
    fmt::Debug,
    marker::PhantomData,
    ops::Add,
};

use crate::arithmetic::{AssociativeAdd, CommutativeAdd, IdempotentAdd};

pub trait Addition<T>: Copy + Debug
where
    T: Copy + Debug,
{
    fn add(a: T, b: T) -> T;
}
#[derive(Clone, Copy, Debug)]
pub struct Wrapper<T, F>
where
    T: Copy + Debug,
    F: Addition<T>,
{
    v: T,
    f: PhantomData<F>,
}
impl<T, F> Wrapper<T, F>
where
    T: Copy + Debug,
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

impl<T, F> Add for Wrapper<T, F>
where
    T: Copy + Debug,
    F: Addition<T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Wrapper::new(F::add(self.v, rhs.v))
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
