use crate::{algebraic_structure::{Monoid, Group}, arithmetic::IdentityAdd, num_integer::Integer};
use std::{fmt::Debug, cmp::min};

///
/// Maintain an array A[0..n), allow O(\log_2 n) prefix sum query and point update
/// 
/// # Example
/// 
/// ```
/// use contest::fenwick_tree::*;
/// let mut ft = FenwickTree::with_initial_value(&[1, 2, 3, 4]);
/// assert_eq!(ft.query(0), 1);
/// assert_eq!(ft.query(1), 3);
/// assert_eq!(ft.query(2), 6);
/// assert_eq!(ft.query(3), 10);
/// ft.update(0, 3);
/// assert_eq!(ft.query(2), 9);
/// ft.update(3, 1);
/// assert_eq!(ft.query(2), 9);
/// assert_eq!(ft.query(3), 14);
/// ```
/// 
#[derive(Debug, Clone)]
pub struct FenwickTree<T>
where T: Debug + Monoid {
    data: Vec<T>
}

impl<T> FenwickTree<T> 
where T: Debug + Monoid {
    pub fn new(n: usize) -> Self {
        FenwickTree{
            data: vec![<T as IdentityAdd>::ZERO; n + 1]
        }
    }
    ///
    /// create fenwick tree with initial value O(n)
    /// 
    pub fn with_initial_value(data: &[T]) -> Self {
        let n = data.len();
        let mut res = Self::new(n);
        for i in 0..n {
            res.data[i + 1] = data[i];
        }
        for i in 1..(n + 1) {
            let to = i + i.lowest_set_bit();
            if to <= n {
                res.data[to] = res.data[to] + res.data[i];
            }
        }
        res
    }

    pub fn clear(&mut self) {
        self.data.fill(<T as IdentityAdd>::ZERO);
    }

    ///
    /// A[i] += u
    /// 
    pub fn update(&mut self, i: usize, u: T) {
        let mut i = i + 1;
        if i <= 0 {
            return;
        }
        while i < self.data.len() {
            self.data[i] = self.data[i] + u;
            i = i + i.lowest_set_bit();
        }
    }

    ///
    /// A[0] + ... + A[i] in log_2 n
    /// 
    pub fn query(&self, i: usize) -> T {
        let mut i = min(i + 1, self.data.len() - 1);
        let mut res = <T as IdentityAdd>::ZERO;
        while i > 0 {
            res = res + self.data[i];
            i = i - i.lowest_set_bit();
        }
        res
    }
}

impl<T> FenwickTree<T>
where T: Debug + Group {
    pub fn query_range(&self, l: usize, r: usize) -> T {
        if l > r {
            return <T as IdentityAdd>::ZERO;
        }
        self.query(r) - self.query(l - 1)
     }
}