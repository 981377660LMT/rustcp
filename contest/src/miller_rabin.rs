
use crate::num_integer::Integer;
use crate::num_number::FromNumber;
use crate::rand::{rng, random};
///
/// time: O(slog_2n)
/// 
pub fn miller_rabin<T: Integer>(n: T, s: usize) -> bool {
    if n <= T::ONE {
        return false;
    }
    if n == FromNumber::from(2) {
        return true;
    }
    if n.kth_bit(0) == T::ZERO {
        return false;
    }
    let mut m = n - T::ONE;
    while m.kth_bit(0) == T::ZERO {
        m >>= T::ONE;
    }
    let modulus = n;
    for _ in 0..s {
        let x = random(n - FromNumber::from(2)) + FromNumber::from(2);
        if !miller_rabin_internal(x, n, m, modulus) {
            return false;
        }
    }
    return true;
}



fn miller_rabin_internal<T: Integer>(x: T, n: T, m: T, modulus: T) -> bool {
    test(T::pow_mod(x, m, modulus), m, n, modulus)
}

fn test<T: Integer>(y: T, exp: T, n: T, modulus: T) -> bool {
    let y2 = T::mul_mod(y, y, modulus);
    if !((exp == n - T::ONE) || test(y2, exp * FromNumber::from(2), n, modulus)) {
        return false;
    }
    if exp != n - T::ONE && y2 != T::ONE {
        return true;
    }
    if y != T::ONE && y != n - T::ONE {
        return false;
    }
    return true;
}