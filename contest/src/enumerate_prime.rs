use std::cmp::{max, min};

use crate::{bitset::BitSet, math::sqrt_ceil};

pub fn sieve_eratosthenes(n: usize, consumer: &mut impl FnMut(usize)) {
    if n <= 1 {
        return;
    }
    let block = sqrt_ceil(n).unwrap();
    let mut is_comp = vec![false; block + 1];
    let mut primes = Vec::with_capacity(block + 1);
    for i in 2..=block {
        if is_comp[i] {
            continue;
        }
        primes.push(i);
        for j in (i + i..=block).step_by(i) {
            is_comp[j] = true;
        }
    }
    for &p in primes.iter() {
        consumer(p);
    }
    for l in (block + 1..=n).step_by(block) {
        let r = min(l + block - 1, n);
        is_comp.fill(false);
        for &p in primes.iter() {
            if r < p * p {
                break;
            }
            let top = if l < p * p { 0 } else { l - p * p };
            let bot = p;
            for j in ((top + bot - 1) / bot * p + p * p..=r).step_by(p) {
                is_comp[j - l] = true;
            }
        }
        for j in l..=r {
            if !is_comp[j - l] {
                consumer(j);
            }
        }
    }
}

fn estimate_phi(n: usize) -> usize {
    static phi: [usize; 10] = [
        4, 25, 168, 1229, 9592, 78498, 664579, 5761455, 50847534, 455052511,
    ];
    let mut x = 10;
    let mut i = 0;
    while x < n {
        x *= 10;
        i += 1;
    }
    phi[i]
}
pub fn sieve_euler(n: usize) -> (Vec<usize>, BitSet) {
    let mut is_comp = BitSet::new(n + 1);
    let mut primes: Vec<usize> = Vec::with_capacity(estimate_phi(n));
    for i in 2..=n {
        if !is_comp[i] {
            primes.push(i);
        };
        for &x in primes.iter() {
            let pi = x * i;
            if pi > n {
                break;
            }
            is_comp.set(pi);
            if i % x == 0 {
                break;
            }
        }
    }
    (primes, is_comp)
}
#[test]
pub fn test_0() {
    let n = 1e6 as usize;
    let mut a = Vec::with_capacity(n);
    sieve_eratosthenes(n, &mut |x| a.push(x));
    let b = sieve_euler(n).0;
    assert_eq!(a, b);
}
