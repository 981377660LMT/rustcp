use std::marker::PhantomData;

use crate::{
    collection::swap_element,
    macros::{should_eq},
    math::{log2_ceil, pow, dot_mul},
    modint::ModInt,
    num_integer::Integer,
    poly::{Convolution,Inverse}, num_number::FromNumber, num_integer_reverse::BitReverse, algebraic_structure::Field, poly_common::{poly_extend, poly_length, convolution_brute_force, poly_trim},
};

pub fn ntt<I: Integer, T: ModInt<I>>(mut p: Vec<T>, inv: bool) -> Vec<T> {
    let modulus = T::modulus();
    let g = T::primitive_root().unwrap();
    let m = log2_ceil(p.len()) as usize;
    let n = 1 << m;
    let shift = usize::BITS as usize - n.count_trailing_zero() as usize;
    for i in 1..n {
        let j = (i << shift).reverse();
        if i < j {
            swap_element(&mut p, i, j);
        }
    }
    let mut ws = vec![T::zero(); n / 2];
    should_eq!((modulus - I::ONE) % FromNumber::from(n), I::ZERO);
    let unit = pow(g, (modulus - I::ONE) / FromNumber::from(n));
    if ws.len() >= 1 {
        ws[0] = T::one();
    }
    for i in 1..ws.len() {
        ws[i] = ws[i - 1] * unit;
    }

    for d in 0..m {
        let s = 1usize << d;
        let s2 = s << 1;
        let right = n >> (1 + d);
        for i in (0..n).step_by(s2) {
            for j in 0..s {
                let a = i + j;
                let b = a + s;
                let t = ws[j * right] * p[b];
                p[b] = p[a] - t;
                p[a] = p[a] + t;
            }
        }
    }

    if inv {
        let inv_n = T::from(n).possible_inv().unwrap();
        let mut i = 0;
        let mut j = 0;
        while i <= j {
            let a = p[j];
            p[j] = p[i] * inv_n;

            if i != j {
                p[i] = a * inv_n;
            }

            i += 1;
            j = n - i;
        }
    }

    p
}

#[derive(Clone, Copy)]
pub struct ConvolutionNTT<I: Integer, T: ModInt<I>>(PhantomData<(I, T)>);
impl<I: Integer, T: ModInt<I>> ConvolutionNTT<I, T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}



impl<I: Integer, T: ModInt<I> + Field> Inverse<T> for ConvolutionNTT<I, T> {

    fn inverse_internal(data: &[T]) -> Vec<T> {
        if data.len() == 1 {
            return vec![data[0].possible_inv().unwrap()];
        }
        let m = data.len();
        let prev_len = (m + 1) / 2;
        let ans = Self::inverse_internal(&data[0..prev_len]);
        let n = (prev_len - 1) * 2 + m - 1  + 1;
        let proper_len = poly_length(n);
        let ans = poly_extend(ans, proper_len);
        let prefix: Vec<T> = poly_extend(data.to_owned(), proper_len);
        let prefix = ntt(prefix, false);
        let mut ans = ntt(ans, false);
        for i in 0..proper_len {
            ans[i] = ans[i] * (T::from(2) - prefix[i] * ans[i]);
        }
        let ans = ntt(ans, true);
        poly_extend(ans, m)
    }
}
impl<I: Integer, T: ModInt<I>> Convolution<T> for ConvolutionNTT<I, T> {
    fn convolution(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        if a.len() < 50 || b.len() < 50 {
            return convolution_brute_force(a, b);
        }

        let mut a = poly_trim(a);
        let mut b = poly_trim(b);
        let len = a.len() + b.len() - 1;
        let proper_len = 1 << log2_ceil(len);
        a.resize_with(proper_len, T::zero);
        b.resize_with(proper_len, T::zero);
        let a = ntt(a, false);
        let b = ntt(b, false);
        let prod = dot_mul(&a, &b);
        poly_trim(ntt(prod, true))
    }



    fn pow2(a: Vec<T>) -> Vec<T> {
        let mut a = poly_trim(a);
        let len = a.len() + a.len() - 1;
        let proper_len = 1 << log2_ceil(len);
        a.resize_with(proper_len, T::zero);
        let a = ntt(a, false);
        let prod = dot_mul(&a, &a);
        poly_trim(ntt(prod, true))
    }
}
