use crate::{
    algebraic_structure::Field,
    macros::{should, should_eq, debug_discard, debug},
    num_number::FromNumber,
    poly::{Poly, PolyInverse},
};

pub fn kth_term_of_linear_recurrence<T: Field + FromNumber, C: PolyInverse<T>, I: Iterator<Item = usize>>(
    mut lr: Vec<T>,
    prefix: &Vec<T>,
    k: I,
) -> T {
    should!(lr.len() - 1 <= prefix.len());
    should_eq!(lr[0], T::one());
    lr.reverse();
    let modulus = Poly::<T, C>::new(lr);
    let ans = modulus.downgrade_mod(k);
    ans.iter()
        .zip(prefix.iter())
        .map(|(a, b)| *a * *b)
        .reduce(|a, b| a + b).unwrap()
}
