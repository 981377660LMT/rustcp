use std::{ops::{Add, Index}, cmp::max, mem::swap, collections::VecDeque};

use crate::{algebraic_structure::Field, macros::SwapAttribute};

pub struct LinearFeedbackShiftRegister<T: Field> {
    cm: Vec<T>,
    m: usize,
    dm: T,
    cn: Vec<T>,
    seq: Vec<T>,
}

pub struct LinearFeedbackShiftRegisterElementIter<'a, T: Field> {
    cn: &'a [T],
    seq: VecDeque<T>,
}
impl<'a, T: Field> Iterator for LinearFeedbackShiftRegisterElementIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ans = T::zero();
        for (cn_e, seq_e) in self.cn.iter().zip(self.seq.iter().rev()) {
            ans = ans - *cn_e * *seq_e;
        }
        let head = self.seq.pop_front();
        self.seq.push_back(ans);
        head
    }
}

impl<T: Field> LinearFeedbackShiftRegister<T> {
    pub fn new(cap: usize) -> Self {
        let mut res = Self {
            cm: Vec::new(),
            cn: Vec::new(),
            seq: Vec::with_capacity(cap + 1),
            dm: T::zero(),
            m: usize::MAX,
        };
        res.cn.push(T::one());
        res
    }

    fn estimate_delta(&self) -> T {
        let mut ans = T::zero();
        let n = self.seq.len() - 1;
        for i in 0..self.cn.len() {
            ans = ans + self.cn[i] * self.seq[n - i];
        }
        ans
    } 

    pub fn add(&mut self, x: T) {
        let n = self.seq.len();
        self.seq.push(x);
        let dn = self.estimate_delta();
        if dn == T::zero() {
            return;
        }
        if self.m == usize::MAX {
            self.cm = self.cn.clone();
            self.dm = dn;
            self.m = n;
            self.cn.resize(n + 2, T::zero());
            return;
        }
        let ln = self.cn.len() - 1;
        let len = max(ln, n + 1 - ln);
        let mut buf = vec![T::zero(); len + 1];
        for i in 0..self.cn.len() {
            buf[i] = self.cn[i];
        }
        let factor = dn / self.dm;
        for i in n - self.m..n - self.m + self.cm.len() {
            buf[i] = buf[i] - factor * self.cm[i - (n - self.m)];
        }
        if self.cn.len() < buf.len() {
            SwapAttribute!(self.cn, self.cm);
            self.m = n;
            self.dm = dn;
        }
        self.cn = buf;
    }

    pub fn len(&self) -> usize {
        self.cn.len() - 1
    }

    pub fn code_iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.cn[1..self.cn.len()].iter()
    }

    pub fn element_iter<'a>(&'a self) -> LinearFeedbackShiftRegisterElementIter<'a, T> {
        LinearFeedbackShiftRegisterElementIter { cn: &self.cn[1..], seq: VecDeque::from(self.seq.clone())}
    }
}

impl<T: Field> Index<usize> for LinearFeedbackShiftRegister<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cn[index]
    }
}