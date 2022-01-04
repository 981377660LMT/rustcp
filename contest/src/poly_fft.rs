use crate::algebraic_structure::Field;
use crate::num_float::float;
use crate::modint::ModInt;
use crate::num_integer_reverse::BitReverse;
use crate::num_number::FromNumber;
use crate::poly::{Convolution, Inverse};
use crate::poly_common::{poly_extend, poly_length};
use crate::{
    collection::swap_element, complex::Complex, math::log2_ceil, num_integer::Integer,
    num_real::Real,
};
use std::cmp::max;
use std::marker::PhantomData;

fn calculate_level<T: Real>(i: usize) -> Vec<Complex<T>> {
    let s = 1 << i;
    let angle = T::PI / <T as FromNumber>::from(s);
    (0..s)
        .map(|j| {
            let angle = angle * <T as FromNumber>::from(j);
            Complex::new(angle.cos(), angle.sin())
        })
        .collect()
}
fn fft<T: Real>(mut p: Vec<Complex<T>>, inv: bool) -> Vec<Complex<T>> {
    let n = p.len();
    let m = log2_ceil(n) as usize;
    let shift = <usize as Integer>::BITS - n.count_trailing_zero();
    for i in 1..n {
        let j = (i << shift).reverse();
        if i < j {
            swap_element(&mut p, i, j);
        }
    }
    for d in 0..m {
        let s = 1 << d;
        let s2 = s << 1;
        let level = calculate_level::<T>(d);
        for i in (0..n).step_by(s2) {
            for j in 0..s {
                let a = i + j;
                let b = a + s;
                let t = level[j] * p[b];
                p[b] = p[a] - t;
                p[a] = p[a] + t;
            }
        }
    }

    if inv {
        let mut i = 0;
        let mut j = 0;
        let mut tn = <T as FromNumber>::from(n);
        while i <= j {
            let pj = p[j];
            p[j] = p[i] / tn;
            if i != j {
                p[i] = pj / tn;
            }
            i += 1;
            j = n - i;
        }
    }

    p
}

#[derive(Clone, Copy)]
pub struct ConvolutionFFT<I: Integer, T: ModInt<I>>(PhantomData<(I, T)>);
impl<I: Integer, T: ModInt<I>> ConvolutionFFT<I, T> {}
impl<I: Integer, T: ModInt<I>> Convolution<T> for ConvolutionFFT<I, T> {
    fn convolution(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        let rank_a = a.len() - 1;
        let rank_b = b.len() - 1;

        if max(rank_a, rank_b) >= 100000 {
            panic!("??");
        }

        let step = I::from(15);
        let mask = (I::ONE << step) - I::ONE;
        let n = poly_length(rank_a + rank_b + 1);
        let a = a
            .iter()
            .map(ModInt::value)
            .map(|x| {
                let real = FromNumber::from(x & mask);
                let image = FromNumber::from(x >> step);
                Complex(real, image)
            })
            .collect();
        let a = poly_extend(a, n);
        let mut a: Vec<Complex<float>> = fft(a, false);
        let b: Vec<Complex<float>> = b
            .iter()
            .map(ModInt::value)
            .map(|x| {
                let real = FromNumber::from(x & mask);
                let image = FromNumber::from(x >> step);
                Complex(real, image)
            })
            .collect();
        let b = poly_extend(b, n);
        let mut b = fft(b, false);
        let mut i = 0;
        let mut j = 0;
        let two = FromNumber::from(2);
        while i <= j {
            let ari = a[i].0;
            let aii = a[i].1;
            let bri = b[i].0;
            let bii = b[i].1;
            let arj = a[j].0;
            let aij = a[j].1;
            let brj = b[j].0;
            let bij = b[j].1;

            let a1r = (ari + arj) / two;
            let a1i = (aii - aij) / two;
            let a2r = (aii + aij) / two;
            let a2i = (arj - ari) / two;

            let b1r = (bri + brj) / two;
            let b1i = (bii - bij) / two;
            let b2r = (bii + bij) / two;
            let b2i = (brj - bri) / two;

            a[i] = Complex(
                a1r * b1r - a1i * b1i - a2r * b2i - a2i * b2r,
                a1r * b1i + a1i * b1r + a2r * b2r - a2i * b2i,
            );
            b[i] = Complex(
                a1r * b2r - a1i * b2i + a2r * b1r - a2i * b1i,
                a1r * b2i + a1i * b2r + a2r * b1i + a2i * b1r,
            );

            if i != j {
                let a1r = (arj + ari) / two;
                let a1i = (aij - aii) / two;
                let a2r = (aij + aii) / two;
                let a2i = (ari - arj) / two;

                let b1r = (brj + bri) / two;
                let b1i = (bij - bii) / two;
                let b2r = (bij + bii) / two;
                let b2i = (bri - brj) / two;

                a[j] = Complex(
                    a1r * b1r - a1i * b1i - a2r * b2i - a2i * b2r,
                    a1r * b1i + a1i * b1r + a2r * b2r - a2i * b2i,
                );
                b[j] = Complex(
                    a1r * b2r - a1i * b2i + a2r * b1r - a2i * b1i,
                    a1r * b2i + a1i * b2r + a2r * b1i + a2i * b1r,
                );
            }

            i += 1;
            j = n - i;
        }

        let a = fft(a, true);
        let b = fft(b, true);
        let modulus: u64 = FromNumber::from(T::modulus());
        let ans = (0..n)
            .map(|i| {
                let aa: u64 = FromNumber::from(a[i].0.round());
                let bb: u64 = FromNumber::from(b[i].0.round());
                let cc: u64 = FromNumber::from(a[i].1.round());
                (aa + (bb << 15) + (cc << 30)) % modulus
            })
            .map(FromNumber::from)
            .collect();

        ans
    }

    fn pow2(a: Vec<T>) -> Vec<T> {
        let rank_a = a.len() - 1;
        let rank_b = rank_a;
        let step = I::from(15);
        let mask = (I::ONE << step) - I::ONE;
        let n = poly_length(rank_a + rank_b + 1);
        let a = a
            .iter()
            .map(ModInt::value)
            .map(|x| {
                let real = FromNumber::from(x & mask);
                let image = FromNumber::from(x >> step);
                Complex(real, image)
            })
            .collect();
        let a = poly_extend(a, n);
        let mut a: Vec<Complex<float>> = fft(a, false);
        let mut b = a.clone();
        let mut i = 0;
        let mut j = 0;
        let two = FromNumber::from(2);
        while i <= j {
            let ari = a[i].0;
            let aii = a[i].1;
            let bri = b[i].0;
            let bii = b[i].1;
            let arj = a[j].0;
            let aij = a[j].1;
            let brj = b[j].0;
            let bij = b[j].1;

            let a1r = (ari + arj) / two;
            let a1i = (aii - aij) / two;
            let a2r = (aii + aij) / two;
            let a2i = (arj - ari) / two;

            let b1r = (bri + brj) / two;
            let b1i = (bii - bij) / two;
            let b2r = (bii + bij) / two;
            let b2i = (brj - bri) / two;

            a[i] = Complex(
                a1r * b1r - a1i * b1i - a2r * b2i - a2i * b2r,
                a1r * b1i + a1i * b1r + a2r * b2r - a2i * b2i,
            );
            b[i] = Complex(
                a1r * b2r - a1i * b2i + a2r * b1r - a2i * b1i,
                a1r * b2i + a1i * b2r + a2r * b1i + a2i * b1r,
            );

            if i != j {
                let a1r = (arj + ari) / two;
                let a1i = (aij - aii) / two;
                let a2r = (aij + aii) / two;
                let a2i = (ari - arj) / two;

                let b1r = (brj + bri) / two;
                let b1i = (bij - bii) / two;
                let b2r = (bij + bii) / two;
                let b2i = (bri - brj) / two;

                a[j] = Complex(
                    a1r * b1r - a1i * b1i - a2r * b2i - a2i * b2r,
                    a1r * b1i + a1i * b1r + a2r * b2r - a2i * b2i,
                );
                b[j] = Complex(
                    a1r * b2r - a1i * b2i + a2r * b1r - a2i * b1i,
                    a1r * b2i + a1i * b2r + a2r * b1i + a2i * b1r,
                );
            }

            i += 1;
            j = n - i;
        }

        let a = fft(a, true);
        let b = fft(b, true);
        let modulus: u64 = FromNumber::from(T::modulus());
        let ans = (0..n)
            .map(|i| {
                let aa: u64 = FromNumber::from(a[i].0.round());
                let bb: u64 = FromNumber::from(b[i].0.round());
                let cc: u64 = FromNumber::from(a[i].1.round());
                (aa + (bb << 15) + (cc << 30)) % modulus
            })
            .map(FromNumber::from)
            .collect();

        ans
    }

    
}
impl<I: Integer, T: ModInt<I> + Field> Inverse<T> for ConvolutionFFT<I, T> {
}