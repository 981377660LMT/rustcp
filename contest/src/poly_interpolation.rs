use std::{collections::HashMap, hash::Hash, mem::take};

use crate::{
    algebraic_structure::Field,
    poly::{Convolution, Poly},
    poly_common::{convolution_brute_force, poly_div_and_rem, poly_evaluate, poly_trim},
};

pub struct GravityLargrangeInterpolation<T: Field + Hash> {
    points: HashMap<T, T>,
    xs: Vec<T>,
    ys: Vec<T>,
    lx: Vec<T>,
    inv_w: Vec<T>,
}

impl<T: Field + Hash> GravityLargrangeInterpolation<T> {
    pub fn new(cap: usize) -> Self {
        let mut res = Self {
            xs: Vec::with_capacity(cap),
            ys: Vec::with_capacity(cap),
            lx: vec![T::one()],
            inv_w: Vec::with_capacity(cap),
            points: HashMap::with_capacity(cap),
        };
        res
    }
    pub fn add(&mut self, x: T, y: T) {
        if self.points.contains_key(&x) {
            return;
        }
        let n = self.xs.len();
        self.points.insert(x, y);
        self.xs.push(x);
        self.ys.push(y);
        self.lx = convolution_brute_force(vec![T::zero() - x, T::one()], take(&mut self.lx));
        self.inv_w.push(T::one());
        for i in 0..n {
            self.inv_w[i] = self.inv_w[i] * (self.xs[i] - x);
            self.inv_w[n] = self.inv_w[n] * (x - self.xs[i]);
        }
    }

    pub fn estimate_point(&self, x: T) -> T {
        if let Some(y) = self.points.get(&x) {
            return *y;
        }
        let y = poly_evaluate(&self.lx, x);
        let mut sum = T::zero();
        for i in 0..self.xs.len() {
            let val = self.inv_w[i] * (x - self.xs[i]);
            let val = self.ys[i] / val;
            sum = sum + val;
        }
        y * sum
    }

    pub fn interpolate(&self) -> Vec<T> {
        let n = self.xs.len();
        let mut ans = vec![T::zero(); n];
        for i in 0..n {
            let c = self.ys[i] / self.inv_w[i];
            let div = poly_div_and_rem(self.lx.clone(), vec![T::zero() - self.xs[i], T::one()]).0;
            for (i, x) in div.iter().enumerate() {
                ans[i] = ans[i] + *x * c;
            }
        }
        let ans = poly_trim(ans);
        ans
    }
}
