use crate::{static_modint::{StaticModInt, MF998244353}, poly_ntt::{ntt, ConvolutionNTT}, poly::Poly};

type mi = StaticModInt<i32, MF998244353>;
type conv = ConvolutionNTT<i32, mi>;
#[test]
fn test_0()  {
    let a = vec![mi::new(0), mi::new(1)];
    let b = ntt(a.clone(), false);
    let dft: Vec<_> = [1, 998244352].into_iter().map(mi::new).collect();
    assert_eq!(dft, b);
    let b = ntt(b, true);
    assert_eq!(a, b);
}
#[test]
fn test_1() {
    let a = vec![mi::new(1), mi::new(1)];
    let a = Poly::<mi, conv>::new(a);
    let b = a.clone().inverse(2);
    let prod = a * b;
    let prod = prod.modular(2);
    assert_eq!(prod, Poly::<mi, conv>::one());
}