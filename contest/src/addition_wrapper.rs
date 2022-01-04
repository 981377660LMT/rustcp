use std::cmp::{max, min};
use std::fmt::Debug;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct MaxAdd<T: Ord + Copy + Debug>(T);

#[derive(Clone, Copy, Debug)]
pub struct MinAdd<T: Ord + Copy + Debug>(pub T);

impl<T: Ord + Copy + Debug> Add<MaxAdd<T>> for MaxAdd<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(max(self.0, rhs.0))
    }
}

impl<T: Ord + Copy + Debug> Add<MinAdd<T>> for MinAdd<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(min(self.0, rhs.0))
    }
}