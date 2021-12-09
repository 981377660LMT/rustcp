use std::cmp::min;

use crate::{algebraic_structure::{Monoid, Group}, arithmetic::IdentityAdd};

///
/// prefix sum
/// 
/// # Example
/// 
/// ```
/// use contest::algebraic_structure::{Monoid, Group};
/// use contest::prefix_sum::*;
/// let ps = PrefixSum::new(vec![1, 2, 3, 4]);
/// assert_eq!(0, ps.prefix(-1));
/// assert_eq!(1, ps.prefix(0));
/// assert_eq!(3, ps.prefix(1));
/// assert_eq!(10, ps.prefix(10));
/// assert_eq!(10, ps.post(0));
/// assert_eq!(0, ps.post(4));
/// assert_eq!(10, ps.interval(0, 3));
/// assert_eq!(1, ps.interval(0, 0));
/// assert_eq!(3, ps.interval(2, 2));
/// assert_eq!(10, ps.interval(-1, 10));
/// assert_eq!(0, ps.interval(0, -1));
/// ```
/// 
#[derive(Debug)]
pub struct PrefixSum<T>
where T: Monoid {
    sum: Vec<T>,
    total: T
}

impl<T> PrefixSum<T>
where T: Monoid {
    pub fn new(mut sum: Vec<T>) -> Self {
        for i in 1..sum.len() {
            sum[i] = sum[i] + sum[i - 1];
        }
        let total = if sum.len() == 0 {
            <T as IdentityAdd>::ZERO
        } else {
            sum[sum.len() - 1]
        };
        Self {
            sum, total
        }
    }

    pub fn prefix(&self, n: isize) -> T {
        let n = min(n, self.sum.len() as isize - 1);
        if n < 0 {
            <T as IdentityAdd>::ZERO
        } else {
            self.sum[n as usize]
        }
    }
}

impl<T> PrefixSum<T>
where T: Group {
    pub fn post(&self, n: isize) -> T {
        self.total - self.prefix(n - 1) 
    }
    pub fn interval(&self, l: isize, r: isize) -> T {
        if l > r {
            <T as IdentityAdd>::ZERO
        } else {
            self.prefix(r) - self.prefix(l - 1) 
        }
    }
}