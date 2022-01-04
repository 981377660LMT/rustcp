use crate::{
    algebraic_structure::{CommutativeRing, Field},
    arithmetic::{AssociativeAdd, AssociativeMul, CommutativeAdd, MulInv},
    collection::swap_element,
    macros::should_eq,
    modint::{modint_sum_batch, ModInt},
    num_integer::Integer,
};
use std::ops::{Add, Index, IndexMut, Mul, Sub, Div};

#[derive(Clone, Debug)]
pub struct Matrix<T: CommutativeRing> {
    data: Vec<T>,
    n: usize,
    m: usize,
}
impl<T: CommutativeRing> AssociativeAdd for Matrix<T> {}
impl<T: CommutativeRing> CommutativeAdd for Matrix<T> {}
impl<T: CommutativeRing> AssociativeMul for Matrix<T> {}

impl<T: CommutativeRing> Matrix<T> {
    #[inline]
    pub fn row_num(&self) -> usize {
        self.n
    }
    #[inline]
    pub fn col_num(&self) -> usize {
        self.m
    }
    pub fn is_square(&self) -> bool {
        self.n == self.m
    }

    pub fn zero(n: usize, m: usize) -> Self {
        let zero = T::zero();
        Self {
            data: vec![zero; n * m],
            n,
            m,
        }
    }
    pub fn square_one(n: usize) -> Self {
        let mut res = Self::square_zero(n);
        for i in 0..n {
            res[(i, i)] = T::one();
        }
        res
    }
    pub fn square_zero(n: usize) -> Self {
        Self::zero(n, n)
    }
    pub fn with_initial_value(data: Vec<T>, n: usize, m: usize) -> Self {
        should_eq!(data.len(), n * m);
        Self { data, n, m }
    }
    pub fn transpose(&self) -> Self {
        let mut res = Self::zero(self.m, self.n);
        for i in 0..self.n {
            for j in 0..self.m {
                res[(j, i)] = self[(i, j)];
            }
        }
        res
    }
}

pub fn matrix_mul_mod<I: Integer, T: ModInt<I>>(lhs: &Matrix<T>, rhs: &Matrix<T>) -> Matrix<T> {
    let right = rhs.transpose();
    let n = lhs.n;
    let m = right.n;
    let k = lhs.m;
    let mut res = Matrix::zero(n, m);
    for i in 0..n {
        for j in 0..m {
            res[(i, j)] = modint_sum_batch(&lhs[i], &right[j]);
        }
    }
    res
}

impl<T: CommutativeRing> Add for Matrix<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        should_eq!(self.n, rhs.n);
        should_eq!(self.m, rhs.m);
        Self::with_initial_value(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a + b)
                .collect(),
            self.n,
            self.m,
        )
    }
}
impl<T: CommutativeRing> Sub for Matrix<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        should_eq!(self.n, rhs.n);
        should_eq!(self.m, rhs.m);
        Self::with_initial_value(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| a - b)
                .collect(),
            self.n,
            self.m,
        )
    }
}
impl<T: CommutativeRing> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        should_eq!(self.m, rhs.n);
        let n = self.n;
        let k = self.m;
        let m = rhs.m;
        let mut res = Self::zero(n, m);
        for i in 0..n {
            for t in 0..k {
                for j in 0..m {
                    res[(i, j)] = res[(i, j)] + self[(i, t)] * rhs[(t, j)];
                }
            }
        }
        res
    }
}

impl<T: CommutativeRing> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let begin = index * self.m;
        &self.data[begin..begin + self.m]
    }
}

impl<T: CommutativeRing> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let begin = index * self.m;
        &mut self.data[begin..begin + self.m]
    }
}

impl<T: CommutativeRing> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.m * index.0 + index.1]
    }
}

impl<T: CommutativeRing> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[self.m * index.0 + index.1]
    }
}

impl<T: CommutativeRing> Matrix<T> {
    pub fn row_swap(&mut self, a: usize, b: usize) {
        if a != b {
            let offset0 = a * self.m;
            let offset1 = b * self.m;
            for i in 0..self.m {
                swap_element(&mut self.data, i + offset0, i + offset1);
            }
        }
    }

    fn row_add(&mut self, a: usize, b: usize, f: T) {
        if a != b {
            let offset0 = a * self.m;
            let offset1 = b * self.m;
            for i in 0..self.m {
                self.data[offset0 + i] = self.data[offset0 + i] + self.data[offset1 + i] * f;
            }
        }
    }

    fn row_mul(&mut self, a: usize, f: T) {
        self[a].iter_mut().for_each(|x| *x = *x * f);
    }
}

impl<T: Field + MulInv> Matrix<T> {
    pub fn determinant(mut self) -> T {
        should_eq!(self.n, self.m);
        let n = self.m;
        let mut ans = T::one();
        for i in 0..n {
            let mut max_row = i;
            for j in i..n {
                if self[(j, i)] != T::zero() {
                    max_row = j;
                    break;
                }
            }
            if self[(max_row, i)] == T::zero() {
                return T::zero();
            }
            if i != max_row {
                self.row_swap(i, max_row);
                ans = T::zero() - ans;
            }
            ans = ans * self[(i, i)];
            self.row_mul(i, self[(i, i)].possible_inv().unwrap());
            for j in i + 1..n {
                if self[(j, i)] == T::zero() {
                    continue;
                }
                let f = T::zero() - self[(j, i)];
                self.row_add(j, i, f);
            }
        }

        ans
    }
}

impl<T: Field + MulInv> MulInv for Matrix<T> {
    fn possible_inv(&self) -> Option<Self> {
        if self.n != self.m {
            return None;
        }
        let n = self.n;
        let mut l = self.clone();
        let mut r = Self::square_one(n);
        for i in 0..n {
            let mut max_row = i;
            for j in i..n {
                if l[(j, i)] != T::zero() {
                    max_row = j;
                    break;
                }
            }
            if l[(max_row, i)] == T::zero() {
                return None;
            }
            l.row_swap(i, max_row);
            r.row_swap(i, max_row);
            let inv = l[(i, i)].possible_inv().unwrap();
            r.row_mul(i, inv);
            l.row_mul(i, inv);
            for j in 0..n {
                if i == j {
                    continue;
                }
                if l[(j, i)] == T::zero() {
                    continue;
                }
                r.row_add(j, i, T::zero() - l[(j, i)]);
                l.row_add(j, i, T::zero() - l[(j, i)]);
            }
        }
        Some(r)
    }
}
impl<T: Field + MulInv> Div for Matrix<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.possible_inv().unwrap()
    }
}