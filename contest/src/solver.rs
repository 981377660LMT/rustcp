use std::{io::{StdinLock, BufWriter, StdoutLock, Write}, panic, cmp::min, ops::Add};
use crate::{fast_input::FastInput, util::debug, arithmetic::*, fenwick_tree::FenwickTree};

pub unsafe fn solve_one(test_id: u32, fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
    let n = fi.ru();
    let q = fi.ru();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(fi.rl());
    }
    let mut ft = FenwickTree::with_initial_value(&a);
    
    for _ in 0..q {
       // ft = debug!(ft);
        let t = fi.ri();
        if t == 0 {
            let p = fi.ru();
            let x = fi.rl();
            ft.update(p, x);
        } else {
            let l = fi.ru();
            let r = fi.ru() - 1;
            let sum = ft.query_range(l, r);
            writeln!(fo, "{}", sum);
        }
    }
}
  
pub unsafe fn solve_multi(fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
    let t: u32 = 1;//fi.read();
    for test_id in 1 .. t + 1 {
        solve_one(test_id, fi, fo);
    }
}