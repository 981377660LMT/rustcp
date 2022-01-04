use crate::{algebraic_structure::Monoid, macros::should_eq};
const NIL: usize = 0;
const ROOT: usize = 1;

pub trait Charset {
    const CHARSET: usize;
}

pub struct Row<S: Monoid, const SIZE: usize> {
    pub sum: S,
    pub adj: [usize; SIZE],
}

impl<S: Monoid, const C: usize> Row<S, C> {
    pub fn new(weight: S) -> Self {
        Self {
            sum: weight,
            adj: [NIL; C],
        }
    }
}

pub struct Trie<S: Monoid, const C: usize> {
    nodes: Vec<Row<S, C>>,
}

impl<S: Monoid, const C: usize> Trie<S, C> {
    pub fn new(estimate_cap: usize) -> Self {
        let mut ans = Self {
            nodes: Vec::with_capacity(2 + estimate_cap),
        };
        ans.new_node();
        ans.new_node();
        ans
    }

    fn new_node(&mut self) -> usize {
        self.nodes.push(Row::new(S::zero()));
        self.nodes.len() - 1
    }

    pub fn update(&mut self, road: &mut impl Iterator<Item = usize>, u: S) {
        self.update_rec(ROOT, road, u)
    }

    pub fn query(&self, road: &mut impl Iterator<Item = usize>) -> usize {
        self.query_internal(ROOT, road)
    }
    fn query_internal(&self, root: usize, road: &mut impl Iterator<Item = usize>) -> usize {
        match road.next() {
            None => root,
            Some(x) => self.query_internal(self.nodes[root].adj[x], road),
        }
    }

    fn update_node(&mut self, root: usize, u: S) {
        if root == NIL {
            return;
        }
        self.nodes[root].sum = self.nodes[root].sum + u;
    }
    fn update_rec(&mut self, root: usize, road: &mut impl Iterator<Item = usize>, u: S) {
        self.update_node(root, u);
        match road.next() {
            None => {}
            Some(index) => {
                if self.nodes[root].adj[index] == NIL {
                    self.nodes[root].adj[index] = self.new_node();
                }
                self.update_rec(self.nodes[root].adj[index], road, u);
            }
        }
    }

    pub fn largest_lexicographi(&self, path_consumer: &mut impl FnMut(usize)) {
        self.largest_lexicographi_rec(ROOT, path_consumer);
    }

    fn largest_lexicographi_rec(&self, root: usize, path_consumer: &mut impl FnMut(usize)) {
        for (index, node) in self.nodes[root].adj.iter().rev().enumerate() {
            if self.nodes[*node].sum != S::zero() {
                path_consumer(index);
                self.largest_lexicographi_rec(root, path_consumer);
                return;
            }
        }
    }
    pub fn smallest_lexicographi(&self, path_consumer: &mut impl FnMut(usize)) {
        self.smallest_lexicographi_rec(ROOT, path_consumer);
    }
    fn smallest_lexicographi_rec(&self, root: usize, path_consumer: &mut impl FnMut(usize)) {
        for (index, node) in self.nodes[root].adj.iter().enumerate() {
            if self.nodes[*node].sum != S::zero() {
                path_consumer(index);
                self.smallest_lexicographi_rec(root, path_consumer);
                return;
            }
        }
    }

    pub fn max_xor(
        &self,
        road: &mut impl Iterator<Item = usize>,
        path_consumer: &mut impl FnMut(usize),
    ) {
        self.binary_prefer(ROOT, &mut road.map(|x| x ^ 1).into_iter(), path_consumer);
    }
    pub fn min_xor(
        &self,
        road: &mut impl Iterator<Item = usize>,
        path_consumer: &mut impl FnMut(usize),
    ) {
        self.binary_prefer(ROOT, road, path_consumer);
    }
    fn binary_prefer(
        &self,
        root: usize,
        road: &mut impl Iterator<Item = usize>,
        path_consumer: &mut impl FnMut(usize),
    ) {
        should_eq!(C, 2);
        match road.next() {
            None => {}
            Some(x) => {
                if self.nodes[self.nodes[root].adj[x]].sum != S::zero() {
                    path_consumer(x);
                    self.binary_prefer(self.nodes[root].adj[x], road, path_consumer);
                } else {
                    path_consumer(x ^ 1);
                    self.binary_prefer(self.nodes[root].adj[x ^ 1], road, path_consumer);
                }
            }
        }
    }

    pub fn node(&mut self, root: usize) -> &mut Row<S, C> {
        &mut self.nodes[root]
    }
}
