use crate::{algebraic_structure::{Ring, Field}, math::log2_ceil, num_number::FromNumber};

pub fn poly_extend<T: Ring>(mut p: Vec<T>, len: usize) -> Vec<T> {
    p.resize(len, T::zero());
    p
}

pub fn poly_modular<T: Ring>(mut p: Vec<T>, len: usize) -> Vec<T> {
    if p.len() <= len {
        p
    } else {
        poly_trim(poly_extend(p, len))
    }
}

pub fn poly_modular_ref<T: Ring>(p: &Vec<T>, len: usize) -> Vec<T> {
    if p.len() <= len {
        p.clone()
    } else {
        poly_trim((0..len).map(|i| p[i]).collect())
    }
}

pub fn poly_length(n: usize) -> usize {
    1 << log2_ceil(n)
}

pub fn poly_trim<T: Ring>(mut p: Vec<T>) -> Vec<T> {
    let zero = T::zero();
    while p.len() > 1 && *p.last().unwrap() == zero {
        p.pop();
    }
    if p.is_empty() {
        p.push(zero);
    }
    p
}

pub fn poly_evaluate<T: Ring>(p: &Vec<T>, x: T) -> T {
    let mut res = T::zero();
    for i in p.iter().rev() {
        res = res * x + *i;
    }
    res
}

pub fn poly_div_and_rem<T: Field>(mut a: Vec<T>, mut b: Vec<T>) -> (Vec<T>, Vec<T>) {
    b.reverse();
    let inv_first = b.first().unwrap().possible_inv().unwrap();
    let n = a.len();
    let m = b.len();
    let mut divisor = vec![T::zero(); n - m + 1];
    for i in (m - 1..n).into_iter().rev() {
        if a[i] == T::zero() {
            continue;
        }
        let factor = a[i] * inv_first;
        divisor[i - (m - 1)] = factor;
        for j in 0..m {
            a[i - j] = a[i - j] - b[j] * factor;
        }
    }
    (poly_trim(divisor), poly_trim(a))
}

pub fn convolution_brute_force<T: Ring>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let a = poly_trim(a);
    let b = poly_trim(b);
    let n = a.len();
    let m = b.len();
    let mut c = vec![T::zero(); n + m - 1];
    for i in 0..n {
        for j in 0..m {
            c[i + j] = c[i + j] + a[i] * b[j];
        }
    }
    c
}