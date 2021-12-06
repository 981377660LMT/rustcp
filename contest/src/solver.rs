use std::{io::{StdinLock, BufWriter, StdoutLock, Write}, panic, cmp::min};
use crate::{fast_input::FastInput, util::debug, sparse_table::SparseTable};

pub unsafe fn solve_one(test_id: u32, fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
    let n = fi.ru();
    let q = fi.ru();
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(fi.ri());
    }
    let st = debug!(SparseTable::new(&v[..], min));
    for i in 0..q {
        let l = fi.ru();
        let r = fi.ru() - 1;
        let (l, r) = debug!((l, r));
        let ans = st.query(l, r);
        writeln!(fo, "{}", ans);
    }   
}
  
pub unsafe fn solve_multi(fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
    let t: u32 = 1;//fi.read();
    for test_id in 1 .. t + 1 {
        solve_one(test_id, fi, fo);
    }
}