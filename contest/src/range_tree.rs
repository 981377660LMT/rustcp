use std::fmt::{Display, Debug};

use crate::{macros::should, num_integer::Integer, binary::{lowest_k_one, highest_k_one}};

pub struct RangeTreeIter<'a>(&'a RangeTree, usize);
impl<'a> Iterator for RangeTreeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.ceil(self.1) {
            Some(x) => {
                self.1 = x + 1;
                Some(x)
            },
            None => {
                None
            }
        }
    }
}

pub struct RangeTree(Vec<Vec<u64>>, usize, usize);
const BIT_SHIFT: usize = 6;
const BIT_SHIFT_VALUE: usize = 1 << BIT_SHIFT;
const BIT_SHIFT_VALUE_MASK: usize = BIT_SHIFT_VALUE - 1;

///
/// # Example
/// 
/// ```
/// use template::range_tree::*;
/// let mut rt = RangeTree::new(100);
/// 
/// assert_eq!(None, rt.first());
/// assert_eq!(None, rt.last());
/// 
/// rt.add(30);
/// rt.add(70);
/// 
/// assert!(rt.contain(30));
/// assert!(rt.contain(70));
/// assert!(!rt.contain(0));
/// assert!(!rt.contain(80));
/// 
/// assert_eq!(Some(30), rt.floor(50));
/// assert_eq!(Some(70), rt.floor(70));
/// assert_eq!(None, rt.floor(20));
/// 
/// assert_eq!(Some(30), rt.ceil(0));
/// assert_eq!(Some(70), rt.ceil(60));
/// assert_eq!(None, rt.ceil(90));
/// 
/// assert_eq!(Some(30), rt.first());
/// assert_eq!(Some(70), rt.last());
/// 
/// let mut iter = rt.iter();
/// assert_eq!(Some(30), iter.next());
/// assert_eq!(Some(70), iter.next());
/// assert_eq!(None, iter.next());
/// 
/// rt.remove(30);
/// 
/// assert!(!rt.contain(30));
/// assert_eq!(Some(70), rt.ceil(20));
/// ```
/// 
/// ```
/// use template::range_tree::*;
/// let mut rt = RangeTree::new(82);
/// rt.add(73);
/// rt.add(74);
/// assert_eq!(Some(73), rt.ceil(37));
/// ```
/// 
/// ```
/// use template::range_tree::*;
/// let mut rt = RangeTree::new(101);
/// rt.add(74);
/// rt.add(80);
/// rt.remove(80);
/// assert_eq!(Some(74), rt.ceil(7));
/// ```
/// 
impl RangeTree {
    pub fn new(n: usize) -> Self {
        let level = Self::calc_level(n);
        let mut res = RangeTree(Vec::with_capacity(level), n, 0);
        let mut size = (n + 63) / 64;
        for i in 0..level {
            res.0.push(vec![0; size]);
            size = (n + 63) / 64;
        }
        res
    }
    fn calc_level(mut n: usize) -> usize {
        let mut res = 0;
        while n > 64 {
            res += 1;
            n = (n + 63) / 64;
        }
        res += 1;
        res
    }

    pub fn contain(&self, x: usize) -> bool {
        ((self.0[0][x >> BIT_SHIFT] >> (x & BIT_SHIFT_VALUE_MASK)) & 1) == 1
    }

    pub fn len(&self) -> usize {
        self.2
    }

    pub fn add(&mut self, mut x: usize) {
        if self.contain(x) {
            return;
        }
        self.2 += 1;
        for i in 0..self.0.len() {
            let offset = x & BIT_SHIFT_VALUE_MASK;
            x >>= BIT_SHIFT;
            self.0[i][x] |= 1u64 << offset;
        }
    }
    pub fn remove(&mut self, mut x: usize) {
        if !self.contain(x) {
            return;
        }
        self.2 -= 1;
        let mut last_value = 0;
        for i in 0..self.0.len() {
            if last_value != 0 {
                break;
            }
            let offset = x & BIT_SHIFT_VALUE_MASK;
            x >>= BIT_SHIFT;
            last_value = self.0[i][x] & !(1u64 << offset);
            self.0[i][x] = last_value;
        }
    }
    fn last_set(&self, mut i: usize, mut x: usize, mask: u64) -> usize {
        should!(i == usize::MAX || (self.0[i][x] ^ mask) != 0);
        while i != usize::MAX {
            let offset = (self.0[i][x] ^ mask).higest_set_bit_offset();
            x = (x << BIT_SHIFT) | offset as usize;
            i -= 1;
        }
        x
    }
    fn first_set(&self, mut i: usize, mut x : usize, mask: u64) -> usize {
        should!(i == usize::MAX || (self.0[i][x] ^ mask) != 0);
        while i != usize::MAX {
            let offset = (self.0[i][x] ^ mask).lowest_set_bit().higest_set_bit_offset();
            x = (x << BIT_SHIFT) | offset as usize;
            i -= 1;
        }
        return x;
    }

    pub fn floor(&self, x: usize) -> Option<usize> {
        if self.contain(x) {
            return Some(x);
        }
        let mut y = x;
        for i in 0..self.0.len() {
            let offset = y & BIT_SHIFT_VALUE_MASK;
            y = y >> BIT_SHIFT;
            let head_mask = lowest_k_one(offset as u64);
            if (self.0[i][y] & head_mask) != 0 {
                return Some(self.last_set(i - 1, (y << BIT_SHIFT) | (self.0[i][y] & head_mask).higest_set_bit_offset() as usize, 0));
            }
        }
        return None;
    }

    pub fn ceil(&self, x: usize) -> Option<usize> {
        if x >= self.1 {
            return None;
        }
        if self.contain(x) {
            return Some(x);
        }
        let mut y = x;
        for i in 0..self.0.len() {
            let offset = y & BIT_SHIFT_VALUE_MASK;
            y = y >> BIT_SHIFT;
            let tail_mask = highest_k_one((63 - offset) as u64);
            if (self.0[i][y] & tail_mask) != 0 {
                return Some(self.first_set(i - 1, (y << BIT_SHIFT) | (self.0[i][y] & tail_mask).lowest_set_bit().higest_set_bit_offset() as usize, 0));
            }
        }
        return None;
    }

    pub fn first(&self) -> Option<usize> {
        if self.2 == 0 {
            None
        } else {
            Some(self.first_set(self.0.len() - 1, 0, 0))
        }
    }
    
    pub fn last(&self) -> Option<usize> {
        if self.2 == 0 {
            None
        } else {
            Some(self.last_set(self.0.len() - 1, 0, 0))
        }
    }

    pub fn iter_with_start_offset<'a>(&'a self, start_offset: usize) -> RangeTreeIter<'a> {
        RangeTreeIter(self, start_offset)
    }

    pub fn iter<'a>(&'a self) -> RangeTreeIter<'a> {
        self.iter_with_start_offset(0)
    }
}

impl Display for RangeTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: Vec<usize> = self.iter().collect();
        res.fmt(f)
    }
}