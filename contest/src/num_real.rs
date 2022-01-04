use crate::{num_number::{Number, FromNumber}, num_concrete::Concrete};

pub trait Real: Concrete {
    const PI: Self;
    const E: Self;

    fn average(a: Self, b: Self) -> Self {
        (a + b) / FromNumber::from(2)
    }
    fn sqrt(&self) -> Self;
    fn powf(&self, b:Self) -> Self; 
    fn powi(&self, b: i32) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn exp(&self) -> Self;
    fn ln(&self) -> Self;
    fn round(&self) -> Self;
}



