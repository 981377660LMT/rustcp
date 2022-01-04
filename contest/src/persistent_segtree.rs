use std::{fmt::Debug, ops::Add};

use crate::{segment::{cover, have_intersection}, algebraic_structure::Monoid};
#[derive(Debug, Clone, Copy)]
struct Row<S: Monoid> {
    sum: S,
    right: usize,
    left: usize,
}

struct DeltaBinarySearch<S, F> where F: Fn(usize, S, S) -> bool {
    pub L: usize,
    pub R: usize,
    pub f: F,
    pub s1: S,
    pub s2: S,
}
/// segtree with history which allow go back to any time point
///
/// # Example
///
/// ```
/// use template::persistent_segtree::*;
/// let mut st = PersistentSegtree::new(0, 3, 0, |a, b| a + b);
/// assert_eq!(st.query(st.latest_version(), 0, 3), 0);
/// let cur = st.update(st.latest_version(), 2, &1);
/// assert_eq!(st.query(st.latest_version(), 0, 3), 1);
/// st.update(st.latest_version(), 0, &3);
/// assert_eq!(st.query(st.latest_version(), 0, 3), 4);
/// assert_eq!(st.query(cur, 0, 3), 1);
/// ```
///
pub struct PersistentSegtree<S: Monoid>
{
    nodes: Vec<Row<S>>,
    L: usize,
    R: usize,
    nil_sum: S,
    latest_version: usize,
}

impl<T: Monoid> Debug for PersistentSegtree<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PersistentSegtree")
            .field("nodes", &self.nodes)
            .field("L", &self.L)
            .field("R", &self.R)
            .field("latest_version", &self.latest_version)
            .finish()
    }
}

impl<S: Monoid> PersistentSegtree<S>
{
    pub fn new(L: usize, R: usize, nil_sum: S) -> Self {
        Self {
            nodes: vec![Row {
                sum: nil_sum,
                left: 0,
                right: 0,
            }],
            L,
            R,
            nil_sum,
            latest_version: 0,
        }
    }

    pub fn latest_version(&self) -> usize {
        self.latest_version
    }

    pub fn update(&mut self, root: usize, pos: usize, x: S) -> usize {
        self.latest_version = self.update_rec(root, pos, self.L, self.R, x);
        self.latest_version
    }

    fn clone(&mut self, root: usize) -> usize {
        self.nodes.push(self.nodes[root]);
        self.nodes.len() - 1
    }

    fn push_up(&mut self, root: usize) {
        self.nodes[root].sum = 
            self.nodes[self.nodes[root].left].sum +
            self.nodes[self.nodes[root].right].sum;
    }

    fn update_rec(&mut self, mut root: usize, pos: usize, l: usize, r: usize, x: S) -> usize {
        if !have_intersection!(pos, pos, l, r) {
            return root;
        }
        root = self.clone(root);
        if cover!(pos, pos, l, r) {
            self.nodes[root].sum = self.nodes[root].sum + x;
            return root;
        }
        let m = (l + r) >> 1;
        self.nodes[root].left = self.update_rec(self.nodes[root].left, pos, l, m, x);
        self.nodes[root].right = self.update_rec(self.nodes[root].right, pos, m + 1, r, x);
        self.push_up(root);
        root
    }

    pub fn query(&self, root: usize, l: usize, r: usize) -> S {
        self.query_rec(root, l, r, self.L, self.R)
    }

    fn query_rec(&self, root: usize, L: usize, R: usize, l: usize, r: usize) -> S {
        if !have_intersection!(L, R, l, r) {
            return S::zero();
        }
        if cover!(L, R, l, r) {
            return self.nodes[root].sum;
        }
        let m = (l + r) >> 1;
        let lson = self.query_rec(self.nodes[root].left, L, R, l, m);
        let rson = self.query_rec(self.nodes[root].right, L, R, m + 1, r);
        lson + rson
    }

    ///O(\log_2 n)
    pub fn first_true(&self, l: usize, r: usize, f: impl Fn(usize, S) -> bool) -> (usize, S) {
        let mut s = S::zero();
        let res = self.first_true_rec(0, l, r, self.L, self.R, &f, &mut s);
        (res, s)
    }

    pub fn first_true_delta<F>(
        &self,
        root1: usize,
        root2: usize,
        l: usize,
        r: usize,
        f: F,
    ) -> (usize, S, S) 
    where F: Fn(usize, S, S) -> bool {
        let mut sum = &mut DeltaBinarySearch {
            L: l,
            R: r,
            f,
            s1: S::zero(),
            s2: S::zero(),
        };
        let res = self.first_true_delta_rec(root1, root2, self.L, self.R, &mut sum);
        (res, sum.s1, sum.s2)
    }
    fn first_true_delta_rec<F>(
        &self,
        root1: usize,
        root2: usize,
        l: usize,
        r: usize,
        qs: &mut DeltaBinarySearch<S, F>,
    ) -> usize where F:Fn(usize, S, S) -> bool{
        if !have_intersection!(qs.L, qs.R, l, r) {
            return usize::MAX;
        }
        if cover!(qs.L, qs.R, l, r)
            && !(qs.f)(
                r,
                qs.s1 + self.nodes[root1].sum,
                qs.s2 + self.nodes[root2].sum,
            )
        {
            qs.s1 = qs.s1 + self.nodes[root1].sum;
            qs.s2 = qs.s2 + self.nodes[root2].sum;
            return usize::MAX;
        }
        if l == r {
            qs.s1 = qs.s1 + self.nodes[root1].sum;
            qs.s2 = qs.s2 + self.nodes[root2].sum;
            return r;
        }
        let m = (l + r) >> 1;
        let mut res = self.first_true_delta_rec(self.nodes[root1].left, self.nodes[root2].left, l, m, qs);
        if res == usize::MAX {
            res = self.first_true_delta_rec(
                self.nodes[root1].right,
                self.nodes[root2].right,
                m + 1,
                r,
                qs,
            );
        }
        res
    }

    pub fn last_true_delta(
        &self,
        root1: usize,
        root2: usize,
        l: usize,
        r: usize,
        f: impl Fn(usize, S, S) -> bool + 'static,
    ) -> (usize, S, S) {
        let mut sum = &mut DeltaBinarySearch {
            L: l,
            R: r,
            f: Box::new(f),
            s1: S::zero(),
            s2: S::zero(),
        };
        let res = self.last_true_delta_rec(root1, root2, self.L, self.R, &mut sum);
        (res, sum.s1, sum.s2)
    }
    fn last_true_delta_rec<F>(
        &self,
        root1: usize,
        root2: usize,
        l: usize,
        r: usize,
        qs: &mut DeltaBinarySearch<S, F>,
    ) -> usize where F: Fn(usize, S, S) -> bool{
        if !have_intersection!(qs.L, qs.R, l, r) {
            return usize::MAX;
        }
        if cover!(qs.L, qs.R, l, r)
            && (qs.f)(
                r,
                qs.s1 + self.nodes[root1].sum,
                qs.s2 + self.nodes[root2].sum,
            )
        {
            qs.s1 = qs.s1 + self.nodes[root1].sum;
            qs.s2 = qs.s2 + self.nodes[root2].sum;
            return usize::MAX;
        }
        if l == r {
            return r - 1;
        }
        let m = (l + r) >> 1;
        let mut res = self.last_true_delta_rec(self.nodes[root1].left, self.nodes[root2].left, l, m, qs);
        if res == usize::MAX {
            res = self.last_true_delta_rec(
                self.nodes[root1].right,
                self.nodes[root2].right,
                m + 1,
                r,
                qs,
            );
        }
        res
    }
    pub fn first_true_rec(
        &self,
        root: usize,
        L: usize,
        R: usize,
        l: usize,
        r: usize,
        f: &impl Fn(usize, S) -> bool,
        s: &mut S,
    ) -> usize {
        if !have_intersection!(L, R, l, r) {
            return usize::MAX;
        }
        if cover!(L, R, l, r) && !f(r, *s + self.nodes[root].sum) {
            *s = *s + self.nodes[root].sum;
            return usize::MAX;
        }
        if l == r {
            *s = *s + self.nodes[root].sum;
            return r;
        }
        let m = (l + r) >> 1;
        let mut res = self.first_true_rec(root + 1, L, R, l, m, f, s);
        if res == usize::MAX {
            res = self.first_true_rec(self.nodes[root].right, L, R, m + 1, r, f, s);
        }
        res
    }
    ///O(\log_2 n)
    pub fn last_true(&self, l: usize, r: usize, f: impl Fn(usize, S) -> bool) -> (usize, S) {
        let mut s = S::zero();
        let res = self.last_true_rec(0, l, r, self.L, self.R, &f, &mut s);
        (res, s)
    }
    pub fn last_true_rec(
        &self,
        root: usize,
        L: usize,
        R: usize,
        l: usize,
        r: usize,
        f: &impl Fn(usize, S) -> bool,
        s: &mut S,
    ) -> usize {
        if !have_intersection!(L, R, l, r) {
            return usize::MAX;
        }
        if cover!(L, R, l, r) && f(r, *s + self.nodes[root].sum) {
            *s = *s + self.nodes[root].sum;
            return usize::MAX;
        }
        if l == r {
            return r - 1;
        }
        let m = (l + r) >> 1;
        let mut res = self.last_true_rec(root + 1, L, R, l, m, f, s);
        if res == usize::MAX {
            res = self.last_true_rec(self.nodes[root].right, L, R, m + 1, r, f, s);
        }
        res
    }
}
