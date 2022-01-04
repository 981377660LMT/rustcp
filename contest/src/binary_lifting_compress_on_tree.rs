use std::fmt::Debug;

use crate::{algebraic_structure::Monoid, macros::{should_eq, debug}, arithmetic::{IdentityAdd, Nil}};

///
/// Binary lifting with O(n) memory and time complexity for preprocessing
/// 
/// # Example
/// 
/// 0
/// |   \
/// 1   2
/// |\  |
/// 3 4 5
/// 
/// ```
/// use template::root_tree::*;
/// use template::{algebraic_structure::Monoid, arithmetic::{IdentityAdd, Nil}};
/// use template::binary_lifting_compress_on_tree::*;
/// let p = vec![usize::MAX, 0, 0, 1, 1, 2];
/// let depth = depth_on_tree(&p);
/// let weight = vec![Nil;p.len()];
/// let bl = BinaryLiftingCompress::new(&p[..], &depth[..], &weight);
/// assert_eq!(bl.kth_ancestor(1, 1), 0);
/// assert_eq!(bl.kth_ancestor(5, 1), 2);
/// assert_eq!(bl.kth_ancestor(5, 2), 0);
/// assert_eq!(bl.kth_ancestor(3, 2), 0);
/// 
/// assert_eq!(bl.lca(3, 4), 1);
/// assert_eq!(bl.lca(1, 2), 0);
/// assert_eq!(bl.lca(1, 3), 1);
/// assert_eq!(bl.lca(1, 1), 1);
/// assert_eq!(bl.lca(3, 5), 0);
/// ```
/// 
pub struct BinaryLiftingCompress<'a, T = Nil>
where T: Monoid {
    pub p: &'a [usize],
    pub depth: &'a [usize],
    pub jump: Vec<usize>,
    pub sum: Vec<T>,
    pub weight: &'a [T],
}

impl<'a, T> BinaryLiftingCompress<'a, T> 
where T: Monoid{
    pub fn new(p: &'a [usize], depth: &'a [usize], weight: &'a [T]) -> Self {
        should_eq!(p.len(), depth.len());
        should_eq!(p.len(), weight.len());
        
        let n = p.len();
        let mut me = Self{
            p, depth, weight,
            jump: vec![usize::MAX; n],
            sum: vec![<T as IdentityAdd>::zero(); n]
        };

        for i in 0..p.len() {
            me.consider(i);
        }

        me
    }

    fn consider(&mut self, root: usize) {
        if root == usize::MAX || self.jump[root] != usize::MAX {
            return;
        }
        let p = self.p[root];
        self.consider(p);
        self.add_leaf(root, p);
    }
    
    fn add_leaf(&mut self, leaf: usize, pid: usize) {
        if pid == usize::MAX {
            self.jump[leaf] = leaf;
        } else if self.depth[pid] - self.depth[self.jump[pid]] == self.depth[self.jump[pid]] - self.depth[self.jump[self.jump[pid]]] {
            self.jump[leaf] = self.jump[self.jump[pid]];
            self.sum[leaf] = self.weight[leaf] + self.sum[pid] + self.sum[self.jump[pid]];
        } else {
            self.jump[leaf] = pid;
            self.sum[leaf] = self.weight[leaf];
        }
    }

    fn first_true_raw<F>(&self, mut node: usize, pred: F) -> usize 
    where F: Fn(usize) -> bool{
        while !pred(node) {
            let p = self.jump[node];
            if pred(p) {
                node = self.p[node];
            } else {
                if node == p {
                    return usize::MAX;
                }
                node = p;
            }
        }
        node
    }

    

    pub fn first_true<F>(&self, mut node: usize, pred: F) -> (usize, T) 
    where F: Fn(usize, T) -> bool{
        let mut s = <T as IdentityAdd>::zero();
        while !pred(node, s) {
            let cand = s + self.sum[node];
            let p = self.jump[node];
            if pred(p, cand + self.weight[node]) {
                s = s + self.weight[node];
                node = self.p[node];
            } else {
                s = cand;
                if node == p {
                    return (usize::MAX, s + self.weight[node]);
                }
                node = p;
            }
        }
        (node, s + self.weight[node])
    }
    
    pub fn last_true<F>(&self, mut node: usize, pred: F) -> (usize, T) 
    where F: Fn(usize, T) -> bool {
        let mut s = <T as IdentityAdd>::zero();
        if !pred(node, s) {
            return (usize::MAX, s);
        }
        loop {
            let cand = s + self.sum[node];
            let p = self.jump[node];
            if pred(p, cand + self.weight[p]) {
                if node == self.jump[node] {
                    return (node, cand + self.weight[node]);
                }
                s = cand;
                node = self.jump[node];
            } else {
                let cand = s + self.weight[node];
                let p = self.p[node];
                if pred(p, cand + self.weight[node]) {
                    s = cand;
                    node = p;
                } else {
                    return (node, cand);
                }
            }
        }
    }

    pub fn last_true_raw<F>(&self, mut node: usize, pred: F) -> usize 
    where F: Fn(usize) -> bool {
        if !pred(node) {
            return usize::MAX;
        }
        loop {
            let p = self.jump[node];
            if pred(p) {
                if node == self.jump[node] {
                    return node;
                }
                node = self.jump[node];
            } else {
                let p = self.p[node];
                if pred(p) {
                    node = p;
                } else {
                    return node;
                }
            }
        }
    }

    pub fn kth_ancestor(&self, node: usize, k: usize) -> usize {
        let target = self.depth[node] - k;
        self.first_true_raw(node, |i| -> bool {
            self.depth[i] <= target
        })
    }

    ///
    /// lowest common ancestor in O(log_2 n)
    /// 
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] > self.depth[b] {
            a = self.kth_ancestor(a, self.depth[a] - self.depth[b]);
        } else {
            b = self.kth_ancestor(b, self.depth[b] - self.depth[a]);
        }
        while a != b {
            if self.jump[a] == self.jump[b] {
                a = self.p[a];
                b = self.p[b];
            } else {
                a = self.jump[a];
                b = self.jump[b];
            }
        }
        return a;
    }
}

impl<'a, T> Debug for BinaryLiftingCompress<'a, T>
where T: Monoid + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryLiftingCompress").field("p", &self.p).field("depth", &self.depth).field("jump", &self.jump).field("sum", &self.sum).field("weight", &self.weight).finish()
    }
}