use crate::{
    algebraic_structure::Ring,
    macros::should_eq,
    math::{dot_mul_plus, log2_floor},
    vector_binary_convolution::FWTLayer, num_integer::Integer
};

pub fn binary_subset_transform<T: Ring + 'static>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let n = a.len();
    should_eq!(a.len(), b.len());
    should_eq!(n, n.lowest_set_bit() as usize);
    if n == 0 {
        return Vec::new();
    }
    let log = log2_floor(n) as usize;
    let zero = T::zero();
    let mut fwta = vec![vec![zero; n]; log + 1];
    let mut fwtb = vec![vec![zero; n]; log + 1];
    let mut c = vec![zero; n];
    let mut T = vec![zero; n];
    for i in 0..n {
        let bitcount = i.count_ones() as usize;
        fwta[bitcount][i] = a[i];
        fwtb[bitcount][i] = b[i];
    }
    let mut orfwt = FWTLayer::new(log);
    orfwt.add_multi_or_layer(log);
    for i in 0..=log {
        orfwt.apply(&mut fwta[i]);
        orfwt.apply(&mut fwtb[i]);
    }
    for i in 0..=log {
        T.fill(zero);
        for j in 0..=i {
            let k = i - j;
            dot_mul_plus(&fwta[j], &fwtb[k], &mut T);
        }
        orfwt.inverse(&mut T);
        for j in 0..n {
            if j.count_ones() as usize == i {
                c[j] = T[j];
            }
        }
    }
    c
}
