use std::{cmp::Ordering, mem::{swap, take, replace}};

use crate::arithmetic::UpperBound;

const NIL: usize = 0;

struct Row<K> {
    left: usize,
    right: usize,
    dist: isize,
    key: K,
}

pub struct LeftistTree<K: Ord + UpperBound> {
    nodes: Vec<Row<K>>,
}

impl<K: Ord + UpperBound> LeftistTree<K> {
    pub fn new(n: usize) -> Self {
        let mut res = Self {
            nodes: Vec::with_capacity(n + 1),
        };
        res.nodes.push(Row {
            left: NIL,
            right: NIL,
            dist: -1,
            key: K::max_element(),
        });
        res
    }

    pub fn new_heap(&mut self, k: K) -> usize {
        self.nodes.push(Row { left: NIL, right: NIL, dist: 0, key: k });
        self.nodes.len() - 1
    }   

    pub fn merge(&mut self, mut a: usize, mut b: usize) -> usize {
        if a == NIL || b == NIL {
            return a | b;
        }
        if self.nodes[a].key.partial_cmp(&self.nodes[b].key).unwrap() == Ordering::Greater {
            swap(&mut a, &mut b);
        } 
        self.nodes[a].right = self.merge(self.nodes[a].right, b);
        if self.nodes[self.nodes[a].left].dist < self.nodes[self.nodes[a].right].dist {
            let tmp = self.nodes[a].left;
            self.nodes[a].left = self.nodes[a].right;
            self.nodes[a].right = tmp;
        }
        self.nodes[a].dist = self.nodes[self.nodes[a].right].dist + 1;
        a
    }

    pub fn peek<'a>(&'a self, root: usize) -> &'a K {
        &self.nodes[root].key
    }

    pub fn pop(&mut self, root: usize) -> (K, usize) {
        let heap_top = self.merge(self.nodes[root].left, self.nodes[root].right);
        self.nodes[root].left = NIL;
        self.nodes[root].right = NIL;
        (replace(&mut self.nodes[root].key, K::max_element()), heap_top)
    }
}