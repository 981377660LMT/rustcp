
use std::{fmt::Debug, cmp::min, ops::Sub};

use crate::num_integer::Integer;

///
/// Maintain an array A[0..n), allow O(\log_2 n) prefix sum query and point update
/// 
/// # Example
/// 
/// ```
/// use template::fenwick_tree::*;
/// let mut ft = FenwickTree::with_initial_value(&[1, 2, 3, 4], 0, |x, y| x + y);
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
pub struct FenwickTree<T>
where T: Debug + Clone {
    data: Vec<T>,
    zero: T,
    f: Box<dyn Fn(T, T) -> T>,
}
impl<T> Debug for FenwickTree<T> where T: Debug + Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FenwickTree").field("data", &self.data).field("zero", &self.zero).finish()
    }
}

impl<T> FenwickTree<T> 
where T: Debug + Clone {
    pub fn new(n: usize, zero: T, f: impl Fn(T, T) -> T + 'static) -> Self {
        FenwickTree{
            data: vec![zero.clone(); n + 1],
            f: Box::new(f),
            zero
        }
    }
    ///
    /// create fenwick tree with initial value O(n)
    /// 
    pub fn with_initial_value(data: &[T], zero: T, f: impl Fn(T, T) -> T + 'static) -> Self {
        let n = data.len();
        let mut res = Self::new(n, zero, f);
        for i in 0..n {
            res.data[i + 1] = data[i].clone();
        }
        for i in 1..(n + 1) {
            let to = i + i.lowest_set_bit();
            if to <= n {
                res.data[to] = (res.f)(res.data[to].clone(), res.data[i].clone());
            }
        }
        res
    }

    pub fn clear(&mut self) {
        self.data.fill(self.zero.clone());
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
            self.data[i] = (self.f)(self.data[i].clone(), u.clone());
            i = i + i.lowest_set_bit();
        }
    }

    ///
    /// A[0] + ... + A[i] in log_2 n
    /// 
    pub fn query(&self, i: usize) -> T {
        let mut i = min(i + 1, self.data.len() - 1);
        let mut res = self.zero.clone();
        while i > 0 {
            res = (self.f)(res, self.data[i].clone());
            i = i - i.lowest_set_bit();
        }
        res
    }
}

impl<T> FenwickTree<T>
where T: Debug + Clone + Sub<Output = T> {
    pub fn query_range(&self, l: usize, r: usize) -> T {
        if l > r {
            return self.zero.clone();
        }
        self.query(r) - self.query(l - 1)
     }
}