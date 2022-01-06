use std::marker::PhantomData;

use crate::{poly::{Convolution, PolyInverse}, algebraic_structure::{Ring, Field}, num_number::FromNumber, poly_common::convolution_brute_force};

pub struct ConvolutionBF<T>(PhantomData<(T)>);

impl<T: Ring> Convolution<T> for ConvolutionBF<T> {
    fn convolution(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        convolution_brute_force(a, b)
    }
}
impl<T: Field + FromNumber> PolyInverse<T> for ConvolutionBF<T> {

}