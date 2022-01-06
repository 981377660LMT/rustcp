use crate::{linear_recurrence::kth_term_of_linear_recurrence, static_modint::{StaticModInt, MF998244353}, num_number::FromNumber, poly_bf::ConvolutionBF, num_integer::Integer, macros::debug_discard};
use crate::arithmetic::IdentityMul;
use crate::arithmetic::IdentityAdd;
type mi = StaticModInt<i32, MF998244353>;
type conv = ConvolutionBF<mi>;

#[test]
pub fn test_0() {
    let lr: Vec<mi> = vec![1, -1, -1].into_iter().map(FromNumber::from).collect();
    let prefix: Vec<mi> = vec![0, 1].into_iter().map(FromNumber::from).collect();
    let mut fib: (mi, mi) = (mi::zero(), mi::one());
    for i in 0..100 {
        debug_discard!(i);
        let actual = kth_term_of_linear_recurrence::<_, conv, _>(lr.clone(), &prefix, (0..10).map(|k| i.kth_bit(k)));
        assert_eq!(actual, fib.0);
        fib = (fib.1, fib.0 + fib.1);
    }
}