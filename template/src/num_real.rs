use crate::num_number::Number;

pub trait Real: Number {
    fn average(a: Self, b: Self) -> Self {
        (a + b) / Self::from_i8(2)
    }
}

macro_rules! RealImpl {
    ($name: ident) => {
        impl Real for $name {}
    };
}


RealImpl!(f32);
RealImpl!(f64);

