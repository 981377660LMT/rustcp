use crate::{
    algebraic_structure::Monoid,
    macros::should_eq,
    segment::{cover, estimate_tree_size, have_intersection},
};
use std::{fmt::Debug, ops::Add};

#[derive(Clone, Copy, Debug)]
struct NodeRow<S: Monoid, U: Monoid> {
    right: usize,
    sum: S,
    update: U,
}

const ROOT: usize = 0;
///
/// Segtree, O(n) build, O(\log_2 n) per update and per query
///
/// # Example
///
/// ```
/// use template::segtree::*;
/// //0, 1, 2
/// let mut st = Segtree::new(0, 3, &|x: usize| x, 0, 0, |x, y| x + y, |x, y| x + y, |x, y| x + y);
/// assert_eq!(st.query(0, 2), 3);
/// assert_eq!(st.query(0, 1), 1);
/// assert_eq!(st.query(0, 0), 0);
/// assert_eq!(st.query(1, 2), 3);
/// st.update(0, 0, &10);
/// assert_eq!(st.query(0, 0), 10);
/// assert_eq!(st.query(0, 2), 13);
/// ```
///
pub struct Segtree<S: Copy + Debug + Monoid + Add<U, Output = S>, U: Copy + Debug + Eq + Monoid> {
    nodes: Vec<NodeRow<S, U>>,
    L: usize,
    R: usize,
}
impl<S: Monoid + Add<U, Output = S>, U: Monoid> Debug for Segtree<S, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Segtree")
            .field("nodes", &self.nodes)
            .field("L", &self.L)
            .field("R", &self.R)
            .finish()
    }
}
impl<S: Monoid + Add<U, Output = S>, U: Monoid> Segtree<S, U> {
    fn push_up(&mut self, root: usize) {
        self.nodes[root].sum = self.nodes[root + 1].sum + self.nodes[self.nodes[root].right].sum;
    }
    fn modify(&mut self, root: usize, upd: U) {
        self.nodes[root].sum = self.nodes[root].sum + upd;
        self.nodes[root].update = self.nodes[root].update + upd;
    }
    fn push_down(&mut self, root: usize) {
        if self.nodes[root].update != U::zero() {
            let u = self.nodes[root].update;
            self.modify(root + 1, u);
            self.modify(self.nodes[root].right, u);
            self.nodes[root].update = U::zero();
        }
    }

    ///
    /// O(n)
    ///
    pub fn new(L: usize, R: usize, f: impl Fn(usize) -> S) -> Self {
        let capacity = estimate_tree_size(R - L + 1);
        let mut res = Self {
            nodes: Vec::with_capacity(capacity),
            L,
            R,
        };
        res.initialize(L, R, &f);
        should_eq!(res.nodes.len(), capacity);
        res
    }

    fn initialize(&mut self, l: usize, r: usize, f: &impl Fn(usize) -> S) -> usize {
        let root = self.nodes.len();
        self.nodes.push(NodeRow {
            right: usize::MAX,
            sum: S::zero(),
            update: U::zero(),
        });
        if l == r {
            self.nodes[root].sum = f(l);
            return root;
        }
        let m = (l + r) >> 1;
        self.initialize(l, m, f);
        self.nodes[root].right = self.initialize(m + 1, r, f);
        self.push_up(root);
        root
    }
    ///O(\log_2 n)
    pub fn update(&mut self, l: usize, r: usize, u: U) {
        self.update_rec(ROOT, l, r, self.L, self.R, u)
    }

    ///O(\log_2 n)
    pub fn query(&mut self, l: usize, r: usize) -> S {
        self.query_rec(ROOT, l, r, self.L, self.R)
    }
    fn update_rec(&mut self, root: usize, L: usize, R: usize, l: usize, r: usize, u: U) {
        if !have_intersection!(L, R, l, r) {
            return;
        }
        if cover!(L, R, l, r) {
            self.modify(root, u);
            return;
        }
        let m = (l + r) >> 1;
        self.push_down(root);
        self.update_rec(root + 1, L, R, l, m, u);
        self.update_rec(self.nodes[root].right, L, R, m + 1, r, u);
        self.push_up(root);
    }

    fn query_rec(&mut self, root: usize, L: usize, R: usize, l: usize, r: usize) -> S {
        if !have_intersection!(L, R, l, r) {
            return S::zero();
        }
        if cover!(L, R, l, r) {
            return self.nodes[root].sum;
        }
        let m = (l + r) >> 1;
        self.push_down(root);
        let lson = self.query_rec(root + 1, L, R, l, m);
        let rson = self.query_rec(self.nodes[root].right, L, R, m + 1, r);
        lson + rson
    }
    //O(\log_2 n)
    // pub fn first_true(&self, l: usize, r: usize, f: impl Fn(usize, S) -> bool) -> (usize, S) {
    //     let mut s = S::zero();
    //     let res = self.first_true_rec(0, l, r, self.L, self.R, &f, &mut s);
    //     (res, s)
    // }
    // fn first_true_rec(
    //     &self,
    //     root: usize,
    //     L: usize,
    //     R: usize,
    //     l: usize,
    //     r: usize,
    //     f: &impl Fn(usize, S) -> bool,
    //     s: &mut S,
    // ) -> usize {
    //     if !have_intersection!(L, R, l, r) {
    //         return usize::MAX;
    //     }
    //     if cover!(L, R, l, r) && !f(r, SS::merge(s, self.nodes[root].sum)) {
    //         *s = SS::merge(s, self.nodes[root].sum);
    //         return usize::MAX;
    //     }
    //     if l == r {
    //         *s = SS::merge(&s, &self.nodes[root].sum);
    //         return r;
    //     }
    //     let m = (l + r) >> 1;
    //     let mut res = self.first_true_rec(root + 1, L, R, l, m, f, s);
    //     if res == usize::MAX {
    //         res = self.first_true_rec(self.nodes[root].right, L, R, m + 1, r, f, s);
    //     }
    //     res
    // }
    // ///O(\log_2 n)
    // pub fn last_true(&self, l: usize, r: usize, f: impl Fn(usize, S) -> bool) -> (usize, S) {
    //     let mut s = S::zero();
    //     let res = self.last_true_rec(0, l, r, self.L, self.R, &f, &mut s);
    //     (res, s)
    // }
    // fn last_true_rec(
    //     &self,
    //     root: usize,
    //     L: usize,
    //     R: usize,
    //     l: usize,
    //     r: usize,
    //     f: &impl Fn(usize, S) -> bool,
    //     s: &mut S,
    // ) -> usize {
    //     if !have_intersection!(L, R, l, r) {
    //         return usize::MAX;
    //     }
    //     if cover!(L, R, l, r) && f(r, SS::merge(&s, &self.nodes[root].sum)) {
    //         *s = SS::merge(&s, &self.nodes[root].sum);
    //         return usize::MAX;
    //     }
    //     if l == r {
    //         return r - 1;
    //     }
    //     let m = (l + r) >> 1;
    //     let mut res = self.last_true_rec(root + 1, L, R, l, m, f, s);
    //     if res == usize::MAX {
    //         res = self.last_true_rec(self.nodes[root].right, L, R, m + 1, r, f, s);
    //     }
    //     res
    // }
}
