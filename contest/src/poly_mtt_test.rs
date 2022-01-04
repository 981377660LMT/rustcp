use crate::{poly_mtt::ConvolutionMTT, static_modint::{StaticModInt, MF998244353}, poly::Poly, num_number::FromNumber};

type mi = StaticModInt<i32, MF998244353>;
type conv = ConvolutionMTT<i32, mi>;

fn test_inverse(input: &[i32]) {
    let a = input.iter().map(|x| *x).map(FromNumber::from).collect();
    let a = Poly::<mi, conv>::new(a);
    let b = a.clone().inverse(2);
    let prod = a * b;
    let prod = prod.modular(2);
    assert_eq!(prod, Poly::<mi, conv>::one());
}

#[test]
fn test_1() {
    test_inverse(&[1, 1][..]);
}

#[test]
fn test_2() {
    test_inverse(&[2, 5][..]);
}

#[test]
fn test_3() {
    test_inverse(&[5, 4, 3, 2, 1][..]);
}