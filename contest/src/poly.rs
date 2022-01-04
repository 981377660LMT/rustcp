use std::{
    cmp::{max},
    marker::PhantomData,
    mem::take,
    ops::{Add, Div, Mul, Rem, Sub, Index}, fmt::Debug,
};

use crate::{
    algebraic_structure::{Ring, Field}, macros::should_eq, math::{inverse_batch},
    num_number::FromNumber, poly_common::{poly_trim, poly_extend, poly_length, poly_evaluate},
};



pub trait Convolution<T: Ring> {
    fn convolution(a: Vec<T>, b: Vec<T>) -> Vec<T>;
    
    fn pow2(a: Vec<T>) -> Vec<T> {
        Self::convolution(a.clone(), a)
    }
}

pub trait Inverse<T: Field + FromNumber>: Convolution<T>{
    fn inverse(a: Vec<T>, n: usize) -> Vec<T> {
        poly_trim(Self::inverse_internal(&poly_extend(a, n)[..]))
    }
    fn inverse_internal(p: &[T]) -> Vec<T> {
        if p.len() == 1 {
            return vec![T::one() / p[0]];
        }
        let m = p.len();
        let prev_mod = (m + 1) / 2;
        let proper_len = poly_length(m);
        let C = Self::inverse_internal(p.split_at(prev_mod).0);
        let C = poly_extend(C, proper_len);
        let A = p.to_owned();
        let A = poly_extend(A, proper_len);
    
        let mut AC = poly_extend(Self::convolution(A, C.clone()), m);
        let zero = T::zero();
        for i in 0..m {
            AC[i] = zero - AC[i];
        }
        AC[0] = AC[0] + <T as FromNumber>::from(2);
        poly_extend(Self::convolution(C, AC), m)
    }
}

#[derive(Eq)]
pub struct Poly<T: Ring, C: Convolution<T>>(Vec<T>, PhantomData<(T, C)>);
impl<T: Ring, C: Convolution<T>> PartialEq for Poly<T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Ring, C: Convolution<T>> Clone for Poly<T, C> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}
impl<T: Ring, C: Convolution<T>> Debug for Poly<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Poly").field(&self.0).finish()
    }
}

impl<T: Field + FromNumber, C: Inverse<T>> Poly<T, C> 
{
    pub fn divider_and_remainder(self, rhs: Self) -> (Self, Self) {
        let a = self.clone() / rhs.clone();
        (a.clone(), self - a * rhs)
    }

    pub fn integral(&self) -> Self {
        let rank = self.rank();
        let p = &self.0;
        let range: Vec<T> = (1..rank + 2).into_iter().map(T::from).collect();
        let inv = inverse_batch(&range[..]);
        let mut ans = vec![T::zero(); rank + 2];
        for i in 0..= rank {
            ans[i + 1] = inv[i] * p[i];
        }
        Self::new(ans)
    }

    pub fn ln(self, n: usize) -> Self {
        should_eq!(self.0[0], T::one());
        let diff = self.differential().modular(n);
        let inv = self.inverse(n);
        let prod = (diff * inv).modular(n);
        let ans = prod.integral();
        ans.modular(n)
    }

    pub fn exp(self, n: usize) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            self.modular(n).exp0(n)
        }
    }

    fn exp0(&self, n: usize) -> Self {
        if n == 1 {
            Self::one()
        } else {
            let mut ans = self.exp0((n + 1) / 2);
            let mut ln = ans.clone().ln(n);
            let mut ln = self.modular(n) - ln;
            ln.0[0] = ln.0[0] + T::one();
            (ans * ln).modular(n)
        }
    }
}

impl<T: Field + FromNumber, C: Inverse<T>> Poly<T, C> {
    pub fn inverse(self, n: usize) -> Self  {
        if n == 0 {
            Self::zero()
        } else {
            Self::new(C::inverse(self.0, n))
        }
    }
}

impl<T: Ring + FromNumber, C: Convolution<T>> Poly<T, C> {
    pub fn new(p: Vec<T>) -> Self {
        let mut res = Self(p, PhantomData);
        res.trim();
        res
    }

    pub fn to_vec(self) -> Vec<T> {
        self.0
    }

    pub fn with_constant(v: T) -> Self {
        Self::new(vec![v])
    }

    pub fn zero() -> Self {
        Self::new(vec![T::zero()])
    }

    pub fn one() -> Self {
        Self::new(vec![T::one()])
    }

    pub fn convolution_delta(mut a: Self, mut b: Self) -> Self {
        let a_rank = a.rank();
        a.0.reverse();
        let mut res = C::convolution(a.0, b.0);
        let mut res = poly_extend(res, a_rank + 1);
        res.reverse();
        Self::new(res)
    }

    pub fn apply(&self, x: T) -> T {
        poly_evaluate(&self.0, x)
    }

    pub fn differential(&self) -> Self {
        let p = &self.0;
        let mut ans = vec![T::zero(); self.rank()];
        for i in 1..=ans.len() {
            ans[i - 1] = p[i] * T::from(i);
        }
        Self::new(ans)
    }



    pub fn dot(&self, rhs: &Self) -> Self {
        Self::new(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| *a * *b)
                .collect(),
        )
    }

    fn extend(&mut self, n: usize) {
        if n <= self.0.len() {
            return;
        }
        self.0.resize_with(n, T::zero);
    }

    fn trim(&mut self) {
        self.0 = poly_trim(take(&mut self.0));
    }

    pub fn rank(&self) -> usize {
        self.0.len() - 1
    }

    /// self % x^n
    pub fn modular(&self, n: usize) -> Self {
        if self.rank() < n {
            self.clone()
        } else {
            Self::new(self.0[0..n].iter().map(|x| *x).collect())
        }
    }

    pub fn iter(&'_ self) -> core::slice::Iter<'_, T> {
        return self.0.iter();
    }


    
    pub fn batch_mul(mut polys: &mut [Self]) -> Self{
        if polys.len() == 1 {
            return polys[0].clone();
        }
        let mid = polys.len() >> 1;
        let (a, b) = polys.split_at_mut(mid);
        Self::batch_mul(a) * Self::batch_mul(b)
    }
}

impl<T: Ring + FromNumber, C: Convolution<T>> IntoIterator for Poly<T, C> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(mut self) -> std::vec::IntoIter<T> {
        return self.0.into_iter();
    }
}

impl<T: Ring + FromNumber, C: Convolution<T>> Add for Poly<T, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let n = self.0.len();
        let m = rhs.0.len();
        let mut res = poly_extend(self.0, max(n, m));
        for (index, x) in rhs.0.iter().enumerate() {
            res[index] = res[index] + *x;
        }
        Self::new(res)
    }
}

impl<T: Ring + FromNumber, C: Convolution<T>> Sub for Poly<T, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let n = self.0.len();
        let m = rhs.0.len();
        let mut res = poly_extend(self.0, max(n, m));
        for (index, x) in rhs.0.iter().enumerate() {
            res[index] = res[index] - *x;
        }
        Self::new(res)
    }
}

impl<T: Ring + FromNumber, C: Convolution<T>> Default for Poly<T, C> {
    fn default() -> Self {
        Self::new(vec![T::zero()])
    }
}

impl<T: Ring + FromNumber, C: Convolution<T>> Mul for Poly<T, C> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let prod = C::convolution(self.0, rhs.0);
        Self::new(prod)
    }
}

impl<T: Field + FromNumber, C: Inverse<T>> Div for Poly<T, C> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut a = self.0;
        let mut b = rhs.0;
        if a.len() < b.len() {
            return Self::default();
        }
        a.reverse();
        b.reverse();
        let c_rank = a.len() - b.len();
        let proper_len = poly_length(c_rank * 2 + 1);
        let a = poly_extend(a, proper_len);
        let b = poly_extend(b, proper_len);
        let c = C::inverse(b, c_rank + 1);
        let mut prod = poly_extend(C::convolution(a, c), c_rank + 1);
        prod.reverse();
        Self::new(prod)
    }
}

impl<T: Field + FromNumber, C: Inverse<T>> Rem for Poly<T, C> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.divider_and_remainder(rhs).1
    }
}

impl<T: Ring + FromNumber, C: Convolution<T>> Index<usize> for Poly<T, C> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
