use std::cmp::{min, max};

use crate::{segment::{estimate_tree_size, have_intersection, cover}, arithmetic::{LowerBound, UpperBound}, algebraic_structure::Ring};

#[derive(Debug, Clone)]
struct Row<T> {
    first_large: T,
    second_large: T,
    first_large_cnt: T,
    first_small: T,
    second_small: T,
    first_small_cnt: T,
    size: T,
    right: usize,
    dirty: T,
    sum: T,
}

///
/// segtree beat O((\log_2 n)^2) each operation
/// 
/// # Example
/// 
/// ```rust
/// use template::segtree_beat_ext::*;
/// //1 2 3 4
/// let mut st = SegtreeBeatExt::new(1, 4, |x| x);
/// assert_eq!(10, st.query_sum(1, 4));
/// //2 3 4 5
/// st.update_value(1, 4, 1);
/// assert_eq!(14, st.query_sum(1, 4));
/// assert_eq!(3, st.query_min(2, 4));
/// assert_eq!(4, st.query_max(1, 3));
/// //2 3 3 3
/// st.update_min(1, 4, 3);
/// assert_eq!(8, st.query_sum(1, 3));
/// assert_eq!(3, st.query_min(2, 4));
/// assert_eq!(3, st.query_max(1, 3));
/// //2 3 3 3
/// st.update_max(1, 4, 2);
/// assert_eq!(11, st.query_sum(1, 4));
/// assert_eq!(3, st.query_min(2, 4));
/// assert_eq!(3, st.query_max(1, 3));
/// ```
/// 
#[derive(Debug)]
pub struct SegtreeBeatExt<T> where T: Ring + Ord + LowerBound + UpperBound {
    nodes: Vec<Row<T>>,
    L: usize,
    R: usize,
}

impl<T> SegtreeBeatExt<T> where T: Ring + Ord + LowerBound + UpperBound {
    pub fn new(l: usize, r: usize, f: impl Fn(usize) -> T) -> Self {
        let cap = estimate_tree_size(r - l + 1);
        let mut res = Self {
            nodes: Vec::with_capacity(cap),
            L: l, 
            R: r,
        };
        res.initialize(l, r, &f);
        res
    }

    fn initialize(&mut self, l: usize, r: usize, f: &impl Fn(usize) -> T) -> usize {
        let root = self.nodes.len();
        self.nodes.push(
            Row { first_large: T::zero(), 
                second_large: T::zero(), 
                first_large_cnt: T::zero(), 
                first_small: T::zero(), 
                second_small: T::zero(), 
                first_small_cnt: T::zero(), 
                size: T::zero(), 
                right: usize::MAX, 
                dirty: T::zero(), 
                sum: T::zero()
             }
        );
        if l == r {
            let node = &mut self.nodes[root];
            node.sum = f(l);
            node.first_small = node.sum;
            node.first_large = node.sum;
            node.first_small_cnt = T::one();
            node.first_large_cnt = node.first_small_cnt;
            node.second_large = T::min_element();
            node.second_small = T::max_element();
            node.size = T::one();
            return root;
        }
        let m = (l + r) >> 1;
        self.initialize(l, m, f);
        self.nodes[root].right = self.initialize(m + 1, r, f);
        self.push_up(root);
        root
    }

    fn modify_min(self: &mut Self, root: usize, u: T) {
        let node = &mut self.nodes[root];
        if node.first_large <= u {
            return;
        }
        node.sum = node.sum - (node.first_large - u) * node.first_large_cnt;
        node.first_large = u;

        if node.first_small >= u {
            node.first_small = u;
        }
        node.second_small = min(node.second_small, u);
        if node.first_small == node.second_small {
            node.second_small = T::max_element();
        }
    }

    fn modify_max(self: &mut Self, root: usize, u: T) {
        let node = &mut self.nodes[root];
        if node.first_small >= u {
            return;
        }
        node.sum = node.sum + (u - node.first_small) * node.first_small_cnt;
        node.first_small = u;

        if node.first_large <= u {
            node.first_large = u;
        }
        node.second_large = max(node.second_large, u);
        if node.first_large == node.second_large {
            node.second_large = T::min_element();
        }
    }

    fn modify_val(self: &mut Self, root: usize, u: T) {
        let node = &mut self.nodes[root];
        node.dirty = node.dirty + u;
        node.sum = node.sum + u * node.size;
        node.first_small = node.first_small + u;
        node.first_large = node.first_large + u;
        if node.second_small != T::max_element() {
            node.second_small = node.second_small + u;
        }
        if node.second_large != T::min_element() {
            node.second_large = node.second_large + u;
        }
    }

    fn push_up(self: &mut Self, root: usize) {
        let l = root + 1;
            let r = self.nodes[root].right;
            self.nodes[root].sum = self.nodes[l].sum + self.nodes[r].sum;
            self.nodes[root].size = self.nodes[l].size + self.nodes[r].size;

            self.nodes[root].first_large = max(self.nodes[l].first_large, self.nodes[r].first_large);
            self.nodes[root].second_large = T::min_element();
            self.nodes[root].first_large_cnt = T::zero();
            if self.nodes[l].first_large != self.nodes[root].first_large {
                self.nodes[root].second_large = max(self.nodes[root].second_large, self.nodes[l].first_large);
            } else {
                self.nodes[root].first_large_cnt = self.nodes[root].first_large_cnt + self.nodes[l].first_large_cnt;
                self.nodes[root].second_large = max(self.nodes[root].second_large, self.nodes[l].second_large);
            }
            if self.nodes[r].first_large != self.nodes[root].first_large {
                self.nodes[root].second_large = max(self.nodes[root].second_large, self.nodes[r].first_large);
            } else {
                self.nodes[root].first_large_cnt = self.nodes[root].first_large_cnt + self.nodes[r].first_large_cnt;
                self.nodes[root].second_large = max(self.nodes[root].second_large, self.nodes[r].second_large);
            }

            self.nodes[root].first_small = min(self.nodes[l].first_small, self.nodes[r].first_small);
            self.nodes[root].second_small = T::max_element();
            self.nodes[root].first_small_cnt = T::zero();
            if self.nodes[l].first_small != self.nodes[root].first_small {
                self.nodes[root].second_small = min(self.nodes[root].second_small, self.nodes[l].first_small);
            } else {
                self.nodes[root].first_small_cnt = self.nodes[root].first_small_cnt + self.nodes[l].first_small_cnt;
                self.nodes[root].second_small = min(self.nodes[root].second_small, self.nodes[l].second_small);
            }
            if self.nodes[r].first_small != self.nodes[root].first_small {
                self.nodes[root].second_small = min(self.nodes[root].second_small, self.nodes[r].first_small);
            } else {
                self.nodes[root].first_small_cnt = self.nodes[root].first_small_cnt + self.nodes[r].first_small_cnt;
                self.nodes[root].second_small = min(self.nodes[root].second_small, self.nodes[r].second_small);
            }
    }

    fn push_down(self: &mut Self, root: usize) {
        let l = root + 1;
        let r = self.nodes[root].right;
        if self.nodes[root].dirty != T::zero() {
            self.modify_val(l, self.nodes[root].dirty);
            self.modify_val(r, self.nodes[root].dirty);
            self.nodes[root].dirty = T::zero();
        }
        self.modify_min(l, self.nodes[root].first_large);
        self.modify_min(r, self.nodes[root].first_large);
        self.modify_max(l, self.nodes[root].first_small);
        self.modify_max(r, self.nodes[root].first_small);
    }

    pub fn query_sum(self: &mut Self, L: usize, R: usize) -> T {
        self.query_sum_rec(0, L, R, self.L, self.R)
    }

    fn query_sum_rec(self: &mut Self, root: usize, L: usize, R: usize, l: usize, r: usize) -> T {
        if !have_intersection!(L, R, l, r) {
            return T::zero();
        }
        if cover!(L, R, l, r) {
            return self.nodes[root].sum;
        }
        self.push_down(root);
        let mid = (l + r) >> 1;
        let lson = self.query_sum_rec(root + 1, L, R, l, mid);
        let rson = self.query_sum_rec(self.nodes[root].right, L, R, mid + 1, r);
        lson + rson
    }
    pub fn query_max(self: &mut Self, L: usize, R: usize) -> T {
        self.query_max_rec(0, L, R, self.L, self.R)
    }

    fn query_max_rec(self: &mut Self, root: usize, L: usize, R: usize, l: usize, r: usize) -> T {
        if !have_intersection!(L, R, l, r) {
            return T::min_element();
        }
        if cover!(L, R, l, r) {
            return self.nodes[root].first_large;
        }
        self.push_down(root);
        let mid = (l + r) >> 1;
        let lson = self.query_max_rec(root + 1, L, R, l, mid);
        let rson = self.query_max_rec(self.nodes[root].right, L, R, mid + 1, r);
        max(lson, rson)
    }

    pub fn query_min(self: &mut Self, L: usize, R: usize) -> T {
        self.query_min_rec(0, L, R, self.L, self.R)
    }

    fn query_min_rec(self: &mut Self, root: usize, L: usize, R: usize, l: usize, r: usize) -> T {
        if !have_intersection!(L, R, l, r) {
            return T::max_element();
        }
        if cover!(L, R, l, r) {
            return self.nodes[root].first_small;
        }
        self.push_down(root);
        let mid = (l + r) >> 1;
        let lson = self.query_min_rec(root + 1, L, R, l, mid);
        let rson = self.query_min_rec(self.nodes[root].right, L, R, mid + 1, r);
        return min(lson, rson);
    }

    pub fn update_min(self: &mut Self, L: usize, R: usize, u: T) {
        self.update_min_rec(0, L, R, self.L, self.R, u)
    }

    ///x = min(x, u)
    fn update_min_rec(
        self: &mut Self,
        root: usize,
        L: usize,
        R: usize,
        l: usize,
        r: usize,
        u: T,
    ) {
        if !have_intersection!(L, R, l, r) {
            return;
        }
        if cover!(L, R, l, r) {
            if self.nodes[root].first_large <= u {
                return;
            }
            if self.nodes[root].second_large < u {
                self.modify_min(root, u);
                return;
            }
        }
        self.push_down(root);
        let mid = (l + r) / 2;
        self.update_min_rec(root + 1, L, R, l, mid, u);
        self.update_min_rec(self.nodes[root].right, L, R, mid + 1, r, u);
        self.push_up(root);
    }

    pub fn update_max(self: &mut Self, L: usize, R: usize, u: T) {
        self.update_max_rec(0, L, R, self.L, self.R, u)
    }

    ///x = max(x, u)
    fn update_max_rec(
        self: &mut Self,
        root: usize,
        L: usize,
        R: usize,
        l: usize,
        r: usize,
        u: T,
    ) {
        if !have_intersection!(L, R, l, r) {
            return;
        }
        if cover!(L, R, l, r) {
            if self.nodes[root].first_small >= u {
                return;
            }
            if self.nodes[root].second_small > u {
                self.modify_max(root, u);
                return;
            }
        }
        self.push_down(root);
        let mid = (l + r) / 2;
        self.update_max_rec(root + 1, L, R, l, mid, u);
        self.update_max_rec(self.nodes[root].right, L, R, mid + 1, r, u);
        self.push_up(root);
    }

    pub fn update_value(self: &mut Self, L: usize, R: usize, u: T) {
        self.update_value_rec(0, L, R, self.L, self.R, u)
    }

    ///x = min(x, u)
    fn update_value_rec(
        self: &mut Self,
        root: usize,
        L: usize,
        R: usize,
        l: usize,
        r: usize,
        u: T,
    ) {
        if !have_intersection!(L, R, l, r) {
            return;
        }
        if cover!(L, R, l, r) {
            self.modify_val(root, u);
            return;
        }
        self.push_down(root);
        let mid = (l + r) / 2;
        self.update_value_rec(root + 1, L, R, l, mid, u);
        self.update_value_rec(self.nodes[root].right, L, R, mid + 1, r, u);
        self.push_up(root);
    }
}