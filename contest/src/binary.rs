use crate::{num_integer::Integer, util::should};


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