use std::ops::{BitAnd, BitOr, BitXor, Not, Rem, Shl, Shr};

use crate::num_number::Number;

///
/// integer reprenstation
/// 
/// # Example
/// 
/// ```
/// use template::num_integer::*;
/// 
/// assert_eq!((-1i32).count_leading_zero(), 0);
/// assert_eq!(0i32.count_leading_zero(), 32);
/// assert_eq!(2i32.count_leading_zero(), 30);
/// assert_eq!(10usize.count_trailing_zero(), 1);
/// assert_eq!(11usize.count_trailing_zero(), 0);
/// ```
/// 
pub trait Integer:
    Number
    + Rem<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Ord
{
    type UnsignedType: Integer;
    const BITS: i32;


    fn bit_count(&self) -> i32;
    fn higest_set_bit_offset(&self) -> i32;
    fn lowest_set_bit(&self) -> Self;
    fn higest_one_bit(&self) -> Self;
    fn count_leading_zero(&self) -> i32;
    fn count_trailing_zero(&self) -> i32;
    fn modular(a: Self, m: Self) -> Self {
        let res = a % m;
        if res.is_negative() {
            res + m
        } else {
            res
        }
    }
    fn mul_mod(a: Self, b: Self, m: Self) -> Self;
    fn pow_mod(a: Self, n: u64, m: Self) -> Self {
        if n == 0 {
            Self::ONE
        } else {
            let ans = Self::pow_mod(a, n >> 1, m);
            let ans = Self::mul_mod(ans, ans, m);
            if (n & 1) == 1 {
                Self::mul_mod(ans, a, m)
            } else {
                ans
            }
        }
    }
    fn pow(a: Self, mut n: u32) -> Self {
        let mut ans = a;
        while n > 1 {
            n -= 1u32;
            ans = ans * a;
        }
        ans
    }
    fn average_floor(a: Self, b: Self) -> Self {
        (a & b) + ((a ^ b) >> Self::from_i8(1))
    }
    fn average_ceil(a: Self, b: Self) -> Self {
        (a | b) - ((a ^ b) >> Self::from_i8(1))
    }
    fn add_overflow(a: Self, b: Self) -> (Self, bool);
    fn add_or_default(a: Self, b: Self, def: Self) -> (Self, bool) {
        let mut res = Self::add_overflow(a, b);
        if res.1 {
            res.0 = def;
        }
        res
    }
    fn mul_or_default(a: Self, b: Self, def: Self) -> (Self, bool) {
        let mut res = Self::mul_overflow(a, b);
        if res.1 {
            res.0 = def;
        }
        res
    }
    fn bit_left_shift(&self, step: i32) -> Self {
        if step >= Self::BITS {
            Self::ZERO
        } else {
            *self << Self::from_i32(step)
        }
    }
    fn bit_signed_right_shift(&self, step: i32) -> Self;
    fn bit_unsigned_right_shift(&self, step: i32) -> Self;

    fn mul_overflow(a: Self, b: Self) -> (Self, bool);
    fn div_and_remainder(a: Self, b: Self) -> (Self, Self);
}



macro_rules! IntegerImpl {
    ($t: ty, $h: ty, $u: ty) => {
        impl Integer for $t {
            type UnsignedType = $u;
            const BITS: i32 = <$t>::BITS as i32;
            fn count_trailing_zero(&self) -> i32 { 
                let x = 0;
                if *self == <$t as Number>::ZERO {
                    <Self as Integer>::BITS
                } else {
                    <Self as Integer>::BITS - 1 - self.lowest_set_bit().count_leading_zero()
                }
            }
            fn bit_signed_right_shift(&self, step: i32) -> Self{
                if step >= <Self as Integer>::BITS {
                    if self.is_negative() {
                        !Self::ZERO
                    } else {
                        Self::ZERO
                    }
                } else {
                    ((*self as Self::SignedType) >> (step as Self::SignedType)) as Self
                }
            }

            fn bit_unsigned_right_shift(&self, step: i32) -> Self {
                if step >= <Self as Integer>::BITS {
                    Self::ZERO
                } else {
                    ((*self as Self::UnsignedType) >> (step as Self::UnsignedType)) as Self
                }
            }

            fn mul_mod(a: Self, b: Self, m: Self) -> Self {
                let mut res = ((a as Self::HighPrecisionType * b as Self::HighPrecisionType)
                    % (m as Self::HighPrecisionType)) as Self;
                if res.is_negative() {
                    res = res + m;
                }
                res
            }
        
            fn add_overflow(a: Self, b: Self) -> (Self, bool) {
                Self::overflowing_add(a, b)
            }
        
            fn mul_overflow(a: Self, b: Self) -> (Self, bool) {
                Self::overflowing_mul(a, b)
            }
        
            fn bit_count(&self) -> i32 {
                self.count_ones() as i32
            }
        
            fn higest_set_bit_offset(&self) -> i32 {
                (Self::BITS - 1 - self.leading_zeros()) as i32
            }
        
            fn lowest_set_bit(&self) -> Self {
                *self & self.negative()
            }
        
            fn higest_one_bit(&self) -> Self {
                if *self == Self::ZERO {
                    0
                } else {
                    Self::ONE << Self::from_i32(self.higest_set_bit_offset())
                }
            }

            fn count_leading_zero(&self) -> i32 {
                self.leading_zeros() as i32
            }
        
            fn div_and_remainder(a: Self, b: Self) -> (Self, Self) {
                let d = a / b;
                (d, a - d * b)
            }
        }
    };
}

IntegerImpl!(i8, i16, u8);
IntegerImpl!(u8, i16, u8);
IntegerImpl!(i16, i32, u16);
IntegerImpl!(u16, u32, u16);
IntegerImpl!(i32, i64, u32);
IntegerImpl!(u32, u64, u32);
IntegerImpl!(isize, isize, usize);
IntegerImpl!(usize, usize, usize);
IntegerImpl!(i64, i128, u64);
IntegerImpl!(u64, u128, u64);
IntegerImpl!(i128, i128, u128);
IntegerImpl!(u128, u128, u128);