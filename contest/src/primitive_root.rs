use crate::{num_integer::Integer, num_number::FromNumber, pollard_rho::factorize};

pub fn any_primitive_root<T: Integer>(n: T) -> Option<T> {
    let to: i128 = FromNumber::from(n);
    first_primitive_root(n, (0..to).into_iter().map(FromNumber::from))
}

pub fn first_primitive_root<T: Integer>(n: T, iter: impl Iterator<Item = T>) -> Option<T> {
    let prime_factors: Vec<T> = factorize(n).into_iter().collect();
    let phi = n - T::ONE;
    for i in iter {
        let mut flag = true;
        for &f in prime_factors.iter() {
            if T::pow_mod(i, phi / f, n) == T::ONE {
                flag = false;
                break;
            }
        }
        if flag {
            return Some(i);
        }
    }
    None
}
