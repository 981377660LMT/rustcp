use crate::{num_integer::Integer, template_macro::should};


pub fn lowest_k_one<T>(k: T) -> T
where T: Integer
{
    should!(k <= T::from_i32(T::BITS));
    T::from_i32(-1).bit_unsigned_right_shift(T::BITS - k.as_i32())
}

pub fn highest_k_one<T>(k: T) -> T
where T: Integer
{
    should!(k <= T::from_i32(T::BITS));
    T::from_i32(-1).bit_left_shift(T::BITS - k.as_i32()) 
}

#[derive(Clone, Copy, Debug)]
pub struct BinaryIterator<T: Integer> {
    data: T,
    offset: usize,
}
impl<T: Integer> BinaryIterator<T> {
    pub fn new(data: T, offset: usize) -> Self {
        Self{data, offset: offset + 1}
    }
}
impl<T: Integer> Iterator for BinaryIterator<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.offset {
            0 => None,
            _ => {
                self.offset -= 1;
                Some(((self.data >> T::from_usize(self.offset)) & T::ONE).as_usize())
            }
        }
    }
}

