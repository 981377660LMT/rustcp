use crate::{ num_integer::Integer};
use crate::num_gcd::extgcd;

pub fn extcrt<T: Integer>(points: &[(T, T)]) -> Option<(T, T)>  {
    let (mut m1, mut x1) = points[0];
    for &(m2, x2) in points.iter().skip(1) {
        let (x, _, g) = extgcd(m1, m2);
        let a = x % m2;
        if (x2 - x1) % g != T::ZERO {
            return None;
        }
        let m = m1 / g * m2;
        x1 = T::modular(a * ((x2 - x1) / g) % m * m1 % m + x1, m);
        m1 = m;
    }
    Some((m1, x1))
}