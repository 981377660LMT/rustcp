use crate::{algebraic_structure::Monoid, rand::rng};
use std::{
    fmt::{Debug, Display},
    ops::Add,
    usize,
};
const NIL: usize = 0;

pub struct SplitData<T> {
    pub size: usize,
    pub sum: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Row<S: Debug + Copy, U: Debug + Copy + Eq> {
    pub sum: S,
    pub weight: S,
    pub upd: U,
    pub size: usize,
    pub left: usize,
    pub right: usize,
    pub id: usize,
    pub reverse: bool,
    pub sum_rev: S,
}

///
/// Treap implementation, all operation can be done in O(log n)
///
/// # Example
///
/// ```
/// use template::treap::*;
/// let mut t = Treap::<i32, i32>::new(4, 0, 0,
///   |a, b| *a + *b,
///   |a, b| *a,
///   |a, b| *a);
///
/// t.set(0, 0);
/// t.set(1, 1);
/// t.set(2, 2);
/// t.set(3, 3);
/// let mut root = 0;
/// for i in 1..4 {
///     root = t.merge(root, i);
/// }
/// assert_eq!(t.node(root).size, 4);
/// assert_eq!(t.node(root).sum, 6);
/// let (r1, r2) = t.split_by_rank(root, 2);
/// assert_eq!(t.node(r1).sum, 1);
/// assert_eq!(t.node(r2).sum, 5);
/// ```
///
pub struct Treap<S: Debug + Copy + Monoid + Add<U, Output = S>, U: Debug + Copy + Eq + Monoid> {
    nodes: Vec<Row<S, U>>,
}

impl<S: Debug + Copy + Monoid + Add<U, Output = S>, U: Debug + Copy + Eq + Monoid> Treap<S, U> {
    pub fn new(n: usize) -> Self {
        let mut res = Self {
            nodes: vec![
                Row {
                    sum: S::zero(),
                    weight: S::zero(),
                    upd: U::zero(),
                    size: 0,
                    left: 0,
                    right: 0,
                    id: 0,
                    reverse: false,
                    sum_rev: S::zero(),
                };
                n + 1
            ],
        };
        for i in 0..n {
            res.nodes[i + 1].id = i + 1;
            res.nodes[i + 1].size = 1;
        }
        res
    }
    pub fn reverse(&mut self, id: usize) {
        self.reverse_internal(id + 1);
    }
    fn reverse_internal(&mut self, id: usize) {
        self.nodes[id].reverse = !self.nodes[id].reverse;
        {
            let temp = self.nodes[id].sum;
            self.nodes[id].sum = self.nodes[id].sum_rev;
            self.nodes[id].sum_rev = temp;
        }
    }

    fn push_up(&mut self, id: usize) {
        if id == NIL {
            return;
        }

        let left = self.nodes[id].left;
        let right = self.nodes[id].right;
        self.nodes[id].sum = self.nodes[left].sum + self.nodes[id].weight + self.nodes[right].sum;
        self.nodes[id].sum_rev =
            self.nodes[right].sum_rev + self.nodes[id].weight + self.nodes[left].sum_rev;
        self.nodes[id].size = self.nodes[left].size + 1 + self.nodes[right].size;
    }
    fn push_down(&mut self, id: usize) {
        if id == NIL {
            return;
        }

        if self.nodes[id].reverse {
            let tmp = self.nodes[id].left;
            self.nodes[id].left = self.nodes[id].right;
            self.nodes[id].right = tmp;

            self.reverse_internal(self.nodes[id].left);
            self.reverse_internal(self.nodes[id].right);

            self.nodes[id].reverse = false;
        }

        let left = self.nodes[id].left;
        let right = self.nodes[id].right;

        if self.nodes[id].upd != U::zero() {
            let upd = self.nodes[id].upd;
            self.modify_internal(left, upd);
            self.modify_internal(right, upd);
            self.nodes[id].upd = U::zero();
        }
    }
    pub fn set(&mut self, mut id: usize, data: S) {
        id += 1;
        self.nodes[id].weight = data;
        self.push_up(id);
    }
    pub fn modify(&mut self, mut id: usize, upd: U) {
        self.modify_internal(id + 1, upd);
    }
    fn modify_internal(&mut self, id: usize, upd: U) {
        if id == NIL {
            return;
        }
        self.nodes[id].upd = self.nodes[id].upd + upd;
        self.nodes[id].weight = self.nodes[id].weight + upd;
        self.nodes[id].sum = self.nodes[id].sum + upd;
        self.nodes[id].sum_rev = self.nodes[id].sum_rev + upd;
        return;
    }
    ///
    /// return the kth node in the preorder travel sequence, denote it as res, ensure node(res) return the newest value
    ///
    pub fn the_kth_node(&mut self, root: usize, k: usize) -> usize {
        self.the_kth_node_internal(root + 1, k) - 1
    }

    fn the_kth_node_internal(&mut self, root: usize, k: usize) -> usize {
        self.push_down(root);
        let left = self.nodes[root].left;
        let right = self.nodes[root].right;
        if self.nodes[left].size >= k {
            self.the_kth_node_internal(left, k)
        } else if self.nodes[left].size + 1 == k {
            root
        } else {
            self.the_kth_node_internal(right, k - 1 - self.nodes[left].size)
        }
    }

    pub fn split_by_rank(&mut self, root: usize, k: usize) -> (usize, usize) {
        let res = self.split_by_rank_internal(root + 1, k);
        (res.0 - 1, res.1 - 1)
    }
    fn split_by_rank_internal(&mut self, root: usize, k: usize) -> (usize, usize) {
        if k == 0 {
            return (NIL, root);
        }
        self.push_down(root);
        let left = self.nodes[root].left;
        let right = self.nodes[root].right;
        let mut res;
        if self.nodes[left].size >= k {
            res = self.split_by_rank_internal(left, k);
            self.nodes[root].left = res.1;
            res.1 = root;
        } else {
            res = self.split_by_rank_internal(right, k - self.nodes[left].size - 1);
            self.nodes[root].right = res.0;
            res.0 = root;
        }
        self.push_up(root);
        res
    }
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        self.merge_internal(a + 1, b + 1) - 1
    }
    fn merge_internal(&mut self, a: usize, b: usize) -> usize {
        if a == 0 || b == 0 {
            return a | b;
        }
        let root;
        if rng().limit_usize(self.nodes[a].size + self.nodes[b].size) < self.nodes[a].size {
            root = a;
            self.push_down(root);
            self.nodes[a].right = self.merge_internal(self.nodes[a].right, b);
        } else {
            root = b;
            self.push_down(root);
            self.nodes[b].left = self.merge_internal(a, self.nodes[b].left);
        }
        self.push_up(root);
        root
    }
    pub fn node(&mut self, id: usize) -> &mut Row<S, U> {
        &mut self.nodes[id + 1]
    }
    pub fn split_first_true(
        &mut self,
        root: usize,
        f: &impl Fn(usize, S) -> bool,
    ) -> (usize, usize) {
        let res = self.split_first_true_internal(
            root + 1,
            &mut SplitData {
                size: 0,
                sum: S::zero(),
            },
            f,
        );
        (res.0 - 1, res.1 - 1)
    }

    fn split_first_true_internal(
        &mut self,
        root: usize,
        sd: &mut SplitData<S>,
        f: &impl Fn(usize, S) -> bool,
    ) -> (usize, usize) {
        if root == NIL || (*f)(sd.size, sd.sum) {
            return (NIL, root);
        }
        self.push_down(root);
        let left = self.nodes[root].left;
        let right = self.nodes[root].right;
        let mut res;
        if (*f)(
            self.nodes[left].size + sd.size,
            sd.sum + self.nodes[left].sum,
        ) {
            res = self.split_first_true_internal(left, sd, f);
            self.nodes[root].left = res.1;
            res.1 = root;
        } else {
            sd.size += self.nodes[left].size + 1;
            sd.sum = sd.sum + self.nodes[left].sum;
            sd.sum = sd.sum + self.nodes[root].weight;
            res = self.split_first_true_internal(right, sd, f);
            self.nodes[root].right = res.0;
            res.0 = root;
        }
        self.push_up(root);
        res
    }

    pub fn split_last_true(
        &mut self,
        root: usize,
        f: &impl Fn(usize, S) -> bool,
    ) -> (usize, usize) {
        let res = self.split_last_true_internal(
            root + 1,
            &mut SplitData {
                size: 0,
                sum: S::zero(),
            },
            f,
        );
        (res.0 - 1, res.1 - 1)
    }

    fn split_last_true_internal(
        &mut self,
        root: usize,
        sd: &mut SplitData<S>,
        f: &impl Fn(usize, S) -> bool,
    ) -> (usize, usize) {
        if root == NIL {
            return (NIL, root);
        }
        self.push_down(root);

        let left = self.nodes[root].left;
        let right = self.nodes[root].right;
        let mut res;
        if (*f)(
            self.nodes[left].size + sd.size + 1,
            sd.sum + self.nodes[left].sum + self.nodes[root].weight,
        ) {
            sd.size += self.nodes[left].size + 1;
            sd.sum = sd.sum + self.nodes[left].sum;
            sd.sum = sd.sum + self.nodes[root].weight;
            res = self.split_last_true_internal(right, sd, f);
            self.nodes[root].right = res.0;
            res.0 = root;
        } else {
            res = self.split_last_true_internal(left, sd, f);
            self.nodes[root].left = res.1;
            res.1 = root;
        }
        self.push_up(root);
        res
    }
    pub fn dfs_internal(&mut self, root: usize, consumer: &mut impl FnMut(&Row<S, U>)) {
        if root == NIL {
            return;
        }
        self.push_down(root);
        self.dfs_internal(self.nodes[root].left, consumer);
        consumer(&self.nodes[root]);
        self.dfs_internal(self.nodes[root].right, consumer);
    }
}

impl<S: Debug + Copy + Monoid + Add<U, Output = S> + Display, U: Debug + Copy + Eq + Monoid>
    Treap<S, U>
{
    pub fn to_string_tree(&mut self, root: usize) -> String {
        let backup = self.nodes.clone();
        let mut ans = Vec::new();
        self.dfs_internal(root + 1, &mut |x| ans.push(x.weight.to_string()));
        self.nodes = backup;
        ans.join(", ")
    }

    pub fn to_string_tree_sum(&mut self, root: usize) -> String {
        let backup = self.nodes.clone();
        let mut ans = Vec::new();
        self.dfs_internal(root + 1, &mut |x| ans.push(x.sum.to_string()));
        self.nodes = backup;
        ans.join(", ")
    }
}

impl<S: Debug + Copy + Monoid + Add<U, Output = S>, U: Debug + Copy + Eq + Monoid> Debug
    for Treap<S, U>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Treap")
            .field("nodes", &self.nodes)
            .finish()
    }
}
