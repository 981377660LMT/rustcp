use std::collections::HashSet;
use std::ops::Add;

use crate::miller_rabin::miller_rabin;
use crate::num_gcd::gcd;
use crate::{num_integer::Integer, rand::random};
use crate::num_number::{Number, FromNumber};

const WITENESS_TIME: usize = 10;

pub fn find_any_factor<T: Integer>(n: T) -> T {
    if n == T::ONE {
        return n;
    }
    if miller_rabin(n, WITENESS_TIME) {
        return n;
    }
    loop {
        let f = pollard_rho(n);
        if f != n {
            return f;
        }
    }
}

pub fn find_any_prime_factor<T: Integer>(n: T) -> T {
    if n == T::ONE {
        return n;
    }
    let ans = find_any_factor(n);
    if ans == n {
        ans
    } else {
        find_any_prime_factor(ans)
    }
}

///
/// Find all prime factors of n
/// 
/// - time: O(n^{1/4} (\log_2n)^2)
/// 
/// 
pub fn factorize<T: Integer>(n: T) -> HashSet<T> {
    let mut set = HashSet::new();
    if n == T::ONE {
        return set;
    }
    factorize_internal(n, &mut set);
    set
}

fn factorize_internal<T: Integer>(n: T, set: &mut HashSet<T>) {
    let f = find_any_factor(n);
    if f == n {
        set.insert(f);
        return;
    }
    factorize_internal(f, set);
    factorize_internal(n / f, set)
}

///
/// time: O(n^{1/4} \log_2 n)
/// 
pub fn pollard_rho<T: Integer>(n: T) -> T {
    if n == T::ONE {
        return T::ONE;
    }
    if n % FromNumber::from(2) == T::ZERO {
        return FromNumber::from(2);
    }
    if n % FromNumber::from(3) == T::ZERO {
        return FromNumber::from(3);
    }
    let mut x = T::ZERO;
    let mut y = x;
    let mut t = T::ZERO;
    let mut q = T::ONE;
    let mut c = random(n - T::ONE) + T::ONE;
    let mut k = 2u64;
    loop {

        for i in 1..=k {
            x = T::mul_mod(x, x, n);
            x = T::add_mod(x, c, n);
            q = T::mul_mod(q, if x < y {y - x} else {x - y}, n);
            if (i & 127) == 0 {
                t = gcd(q, n);
                if t > T::ONE {
                    return t;
                }
            }
        }
        t = gcd(q, n);
        if t > T::ONE {
            return t;
        }

        k <<= 1;
        y = x;
        q = T::ONE;
    }
} 