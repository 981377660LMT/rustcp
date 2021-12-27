use std::{fmt::Debug, marker::PhantomData, cmp::max, ops::Index, cell::RefCell, path::Iter};

use crate::template_macro::should_eq;
const NIL: usize = 0;
const ROOT: usize = 1;

pub trait Charset {
    const CHARSET: usize;
}

pub struct Row<T, const SIZE: usize> where
T: Debug + Clone {
    pub sum: T,
    pub adj: [usize; SIZE],
}

impl<T, const C: usize> Row<T, C> where
T: Debug + Clone
{
    pub fn new(weight: T) -> Self{
        Self {
            sum: weight.clone(),
            adj: [NIL; C],
        }
    }
}

pub struct Trie<T, const C: usize>
where
    T: Debug + Clone + Eq
{
    nodes: Vec<Row<T, C>>,
    s_u_f: Box<dyn Fn(&T, &T) -> T>,
    zero_T: T,
}

impl<T, const C: usize> Trie<T, C>
where
    T: Debug + Clone + Eq
{
    pub fn new(estimate_cap: usize, zero_T: T,
        s_u_f: impl Fn(&T, &T) -> T + 'static,
    ) -> Self {
        let mut ans = Self {
            nodes: Vec::with_capacity(2 + estimate_cap),
            s_u_f: Box::new(s_u_f),
            zero_T,
        };
        ans.new_node();
        ans.new_node();
        ans
    }

    fn new_node(&mut self) -> usize {
        self.nodes.push(Row::new(self.zero_T.clone()));
        self.nodes.len() - 1
    }

    pub fn update(&mut self, road: &mut impl Iterator<Item = usize>, u: &T) {
        self.update_rec(ROOT, road, u)
    }

    pub fn query(&self, road: &mut impl Iterator<Item = usize>) -> usize {
        self.query_internal(ROOT, road)
    }
    fn query_internal(&self, root: usize, road: &mut impl Iterator<Item = usize>) -> usize {
        match road.next() {
            None => root,
            Some(x) => self.query_internal(self.nodes[root].adj[x], road)
        }
    }

    fn update_node(&mut self, root: usize, u: &T){
        if root == NIL {
            return;
        }
        self.nodes[root].sum = (self.s_u_f)(&self.nodes[root].sum, u);
    }
    fn update_rec(&mut self, root: usize, road: &mut impl Iterator<Item = usize>, u: &T) {
        self.update_node(root, u);
        match road.next() {
            None => {},
            Some(index) => {
                if self.nodes[root].adj[index] == NIL {
                    self.nodes[root].adj[index] = self.new_node();
                }
                self.update_rec(self.nodes[root].adj[index], road, u);
            },
        }
    }

    pub fn largest_lexicographi(&self, path_consumer: &mut impl FnMut(usize)) {
        self.largest_lexicographi_rec(ROOT, path_consumer);
    }

    fn largest_lexicographi_rec(&self, root: usize, path_consumer: &mut impl FnMut(usize)) {
        for (index, node) in self.nodes[root].adj.iter().rev().enumerate() {
            if self.nodes[*node].sum != self.zero_T {
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
            if self.nodes[*node].sum != self.zero_T {
                path_consumer(index);
                self.smallest_lexicographi_rec(root, path_consumer);
                return;
            }
        }
    }

    
    pub fn max_xor(&self,  road: &mut impl Iterator<Item = usize>, path_consumer: &mut impl FnMut(usize)) {
        self.binary_prefer(ROOT, &mut road.map(|x| x ^ 1).into_iter(), path_consumer);
    }
    pub fn min_xor(&self,  road: &mut impl Iterator<Item = usize>, path_consumer: &mut impl FnMut(usize)) {
        self.binary_prefer(ROOT, road, path_consumer);
    }
    fn binary_prefer(&self, root: usize, road: &mut impl Iterator<Item = usize>, path_consumer: &mut impl FnMut(usize)) {
        should_eq!(C, 2);
        match road.next() {
            None => {},
            Some(x) => {
                if self.nodes[self.nodes[root].adj[x]].sum != self.zero_T {
                    path_consumer(x);
                    self.binary_prefer(self.nodes[root].adj[x], road, path_consumer);
                } else {
                    path_consumer(x ^ 1);
                    self.binary_prefer(self.nodes[root].adj[x ^ 1], road, path_consumer);
                }
            }
        }
    }

    pub fn node(&mut self, root: usize) -> &mut Row<T, C> {
        &mut self.nodes[root]
    }
}