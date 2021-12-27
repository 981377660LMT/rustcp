use std::mem::swap;

use crate::{algebraic_structure::{Group}, arithmetic::{CommutativeAdd, IdentityAdd, Nil}};

///
/// 
/// 
/// Disjoint Set 
/// 
/// # Example
/// 
/// ```
/// use template::{algebraic_structure::{Semigroup, Group, Monoid}, arithmetic::{CommutativeAdd, IdentityAdd, Nil}};
/// use template::dsu_compress_path::*;
/// let mut dsu = DSU::new(vec![Nil; 5]);
/// assert!(dsu.find(0usize) != dsu.find(1usize));
/// dsu.union(0usize, 1usize, Nil);
/// assert!(dsu.find(0usize) == dsu.find(1usize));
/// assert!(dsu.find(0usize) != dsu.find(2usize));
/// dsu.union(3usize, 4usize, Nil);
/// dsu.union(1usize, 4usize, Nil);
/// assert!(dsu.find(0usize) == dsu.find(3usize));
/// ```
/// 
pub struct DSU<T = Nil, R = Nil> 
where T: CommutativeAdd,
    R: Group
{
    p: Vec<usize>,
    size: Vec<usize>,
    sum: Vec<T>,
    path_to_root: Vec<R>,
}

impl<T, R> DSU<T, R>
where T: CommutativeAdd,
    R: Group, 
    {
    pub fn new(weight: Vec<T>) -> Self {
        let n = weight.len();
        DSU{
            p: (0..n).into_iter().collect(),
            size: vec![1; n],
            sum: weight,
            path_to_root: vec![<R as IdentityAdd>::zero(); n]
        }
    }
    pub fn find(&mut self, root: usize) -> usize {
        let mut p = self.p[root];
        if p != self.p[p] {
            self.p[root] = self.find(p);
            self.path_to_root[root] = self.path_to_root[root] + self.path_to_root[p];
            p = self.p[root];
        }
        p
    }
    pub fn path_to_root(&mut self, root: usize) -> R {
        self.find(root);
        self.path_to_root[root]
    }
    pub fn sum_of_set(&mut self, root: usize) -> T {
        let root = self.find(root);
        self.sum[root]
    }
    pub fn size_of_set(&mut self, root:usize) -> usize {
        let root = self.find(root);
        self.size[root]
    }
    ///
    /// 
    /// 
    /// Union set a and set b, and has a - b == delta
    /// 
    /// # Formula
    /// 
    /// ```ignore
    /// a - b = delta
    /// a - ra = x
    /// b - rb = y
    /// ra - rb = (a - x) - (b - y) = delta + y - x
    /// ```
    /// 
    pub fn union(&mut self, a: usize, b: usize, delta: R) {
        let mut delta = delta + self.path_to_root(b) - self.path_to_root(a);
        let mut c = self.find(a);
        let mut d = self.find(b);
        if c == d {
            return;
        }
        if self.size[c] < self.size[d] {
            swap(&mut c, &mut d);
        } else {
            delta = <R as IdentityAdd>::zero() - delta;
        }
        self.sum[c] = self.sum[c] + self.sum[d];
        self.size[c] = self.size[c] + self.size[d];
        self.p[d] = c;
        self.path_to_root[d] = delta;
    }
} 