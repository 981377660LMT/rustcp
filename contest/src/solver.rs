use std::{io::{Write, BufRead}, panic};
use std::ops::{Add, Sub, Mul, Div};
use crate::{fast_input::FastInput, poly_ntt::ConvolutionNTT, poly::Poly, static_modint::{StaticModInt, MF998244353}, num_number::FromNumber, linear_recurrence::kth_term_of_linear_recurrence, num_integer::Integer};
use crate::macros::input;
use crate::arithmetic::*;
use crate::algebraic_structure::*;

type mi = StaticModInt<i32, MF998244353>; 
type conv = ConvolutionNTT<i32, mi>;
pub unsafe fn solve_one<I: BufRead>(test_id: usize, fi: &mut FastInput<I>, fo: &mut impl Write) {
    input!{
        fi,
        d: usize,
        k: u64,
    }
    let a: Vec<mi> = (0..d).map(|_| fi.r()).collect();
    let mut c: Vec<mi> = (0..d).map(|_| fi.r()).collect();
    c.push(FromNumber::from(-1));
    let mut c = c.iter().rev().map(|x| mi::zero() - *x).collect();
    let kth = kth_term_of_linear_recurrence::<_, conv, _>(c, &a, (0..60).map(|i| k.kth_bit(i) as usize));
    writeln!(fo, "{}", kth);
}
  
pub unsafe fn solve_multi<I: BufRead>(fi: &mut FastInput<I>, fo: &mut impl Write) {
    let t: usize = 1;//fi.read();
    for test_id in 1 ..= t {
        solve_one(test_id, fi, fo);
    }
}