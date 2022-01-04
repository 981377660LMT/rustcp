use contest::{math::pow, num_float::float, num_number::{FromNumber, Number}};
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut n = n as i64;
        let mut x:float = FromNumber::from(x);
        if n < 0 {
            x = float::ONE / x;
            n = -n;
        }
        pow(x, n).as_f64()
    }
}

struct Solution;
fn main() {
}
