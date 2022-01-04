use crate::{macros::should, num_integer::Integer, num_number::FromNumber};

pub fn lowest_k_one<T>(k: T) -> T
where
    T: Integer,
{
    should!(k <= FromNumber::from(T::BITS));
    T::from(-1).bit_unsigned_right_shift(T::BITS - <i32 as FromNumber>::from(k))
}

pub fn highest_k_one<T>(k: T) -> T
where
    T: Integer,
{
    should!(k <= FromNumber::from(T::BITS));
    T::from(-1).bit_left_shift(T::BITS - <i32 as FromNumber>::from(k))
}

pub fn range_one<T: Integer>(l: T, r: T) -> T {
    if r < l {
        T::ZERO
    } else {
        lowest_k_one(r + T::ONE) ^ lowest_k_one(l)
    }
}

