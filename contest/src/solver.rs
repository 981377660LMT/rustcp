use std::ops::{Add, Div, Mul, Sub};
use std::{
    io::{BufRead, Write},
    panic,
};
use template::{
    fast_input::FastInput,
    range_affine_range_sum::RangeAffineRangeSum,
    static_modint::{StaticModInt, MF998_244_353},
};

use crate::contest_macro::input;

type mi = StaticModInt<u32, MF998_244_353>;

pub unsafe fn solve_one<I: BufRead>(test_id: u32, fi: &mut FastInput<I>, fo: &mut impl Write) {
    let n = fi.ru();
    let q = fi.ru();
    let mut data = Vec::with_capacity(n);
    for _ in 0..n {
        data.push(mi::new(fi.read()));
    }
    let mut rars = RangeAffineRangeSum::new(0, n - 1, &|x| data[x]);
    for i in 0..q {
        let t = fi.ri();
        let l = fi.ru();
        let r = fi.ru() - 1;
        if t == 0 {
            rars.update(l, r, mi::new(fi.read()), mi::new(fi.read()));
        } else {
            let sum = rars.query(l, r);
            writeln!(fo, "{}", sum);
        }
    }
}
pub unsafe fn solve_multi<I: BufRead>(fi: &mut FastInput<I>, fo: &mut impl Write) {
    let t: u32 = 1; //fi.read();
    for test_id in 1..t + 1 {
        solve_one(test_id, fi, fo);
    }
}
