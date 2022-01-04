use std::{io::{Write, BufRead}, panic};
use std::ops::{Add, Sub, Mul, Div};
use crate::{fast_input::FastInput, poly_interpolation::GravityLargrangeInterpolation, static_modint::{StaticModInt, MF998244353}};
use crate::macros::input;
use crate::arithmetic::*;
use crate::algebraic_structure::*;

type mi = StaticModInt<i32, MF998244353>;
pub unsafe fn solve_one<I: BufRead>(test_id: usize, fi: &mut FastInput<I>, fo: &mut impl Write) {
    let n = fi.ru();
    let mut int = GravityLargrangeInterpolation::new(n);
    for _ in 0..n {
        let t = fi.ri(); 
        if t == 1 {
            input! {
                fi,
                x: mi,
                y: mi,
            }
            int.add(x, y);
        } else {
            let k = fi.r();
            let ans = int.estimate_point(k);
            writeln!(fo, "{}", ans);
        }
    }
}
  
pub unsafe fn solve_multi<I: BufRead>(fi: &mut FastInput<I>, fo: &mut impl Write) {
    let t: usize = 1;//fi.read();
    for test_id in 1 ..= t {
        solve_one(test_id, fi, fo);
    }
}