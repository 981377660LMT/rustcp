use crate::algebraic_structure::Field;
use crate::macros::should;
use crate::num_number::Number;
use crate::poly::{Inverse};
use crate::poly_common::poly_trim;
use crate::{
    math::max_batch,
    modint::ModInt,
    num_integer::Integer,
    num_number::FromNumber,
    poly::Convolution,
    poly_ntt::ConvolutionNTT,
    static_modint::{StaticModInt, MF167772161, MF469762049, MF998244353},
};
use std::marker::PhantomData;

type mi0 = StaticModInt<i32, MF469762049>;
type mi1 = StaticModInt<i32, MF998244353>;
type mi2 = StaticModInt<i32, MF167772161>;

type cv0 = ConvolutionNTT<i32, mi0>;
type cv1 = ConvolutionNTT<i32, mi1>;
type cv2 = ConvolutionNTT<i32, mi2>;

static M0: i64 = 469762049;
static M1: i64 = 998244353;
static M2: i64 = 167772161;
static mod2: i64 = M0 * M1;
static inv10: i64 = 208783132;
static inv01: i64 = 554580198;
static inv012: i64 = 29562547;
static p1inv10: i64 = 208416582520653596;
static p0inv01: i64 = 260520730147305702;

pub struct ConvolutionMTT<I: Integer, T: ModInt<I>>(PhantomData<(I, T)>);

impl<I: Integer, T: ModInt<I>>  ConvolutionMTT<I, T> {
    fn decrypt(c0: Vec<mi0>, c1: Vec<mi1>, c2: Vec<mi2>) -> Vec<T> {
        let modulus: i64 = FromNumber::from(T::modulus());
        let &len = max_batch(&[c0.len(), c1.len(), c2.len()][..]).unwrap();
        let mut ans = Vec::with_capacity(len);
        for i in 0..len {
            let a0: i64 = c0.get(i).map(ModInt::value).unwrap_or(i32::ZERO) as i64;
            let a1: i64 = c1.get(i).map(ModInt::value).unwrap_or(i32::ZERO) as i64;
            let a2: i64 = c2.get(i).map(ModInt::value).unwrap_or(i32::ZERO) as i64;
            let t0 = i64::add_mod(
                i64::mul_mod(a0, p1inv10, mod2),
                i64::mul_mod(a1, p0inv01, mod2),
                mod2,
            );
            let t1 = i64::modular(a2 - t0, M2) * inv012 % M2;
            let val = (t1 * M0 % modulus * M1 + t0) % modulus;
            ans.push(FromNumber::from(val));
        }

        poly_trim(ans)
    }
}
impl<I: Integer, T: ModInt<I>> Convolution<T> for ConvolutionMTT<I, T> {
    fn convolution(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        should!(I::BITS <= 32);
        let c0 = cv0::convolution(
            a.iter().map(|&x| FromNumber::from(x.value())).collect(),
            b.iter().map(|&x| FromNumber::from(x.value())).collect(),
        );
        let c1 = cv1::convolution(
            a.iter().map(|&x| FromNumber::from(x.value())).collect(),
            b.iter().map(|&x| FromNumber::from(x.value())).collect(),
        );
        let c2 = cv2::convolution(
            a.iter().map(|&x| FromNumber::from(x.value())).collect(),
            b.iter().map(|&x| FromNumber::from(x.value())).collect(),
        );
        Self::decrypt(c0, c1, c2)
    }

    fn pow2(a: Vec<T>) -> Vec<T> {
        should!(I::BITS <= 32);
        let c0 = cv0::pow2(
            a.iter().map(|&x| FromNumber::from(x.value())).collect(),
        );
        let c1 = cv1::pow2(
            a.iter().map(|&x| FromNumber::from(x.value())).collect(),
        );
        let c2 = cv2::pow2(
            a.iter().map(|&x| FromNumber::from(x.value())).collect(),
        );
        Self::decrypt(c0, c1, c2)
    }
}

impl<I: Integer, T: ModInt<I> + Field> Inverse<T> for ConvolutionMTT<I, T> {
}