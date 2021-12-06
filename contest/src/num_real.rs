use crate::num_number::Number;

pub trait Real: Number {
    fn average(a: Self, b: Self) -> Self {
        (a + b) / Self::from_i8(2)
    }
}