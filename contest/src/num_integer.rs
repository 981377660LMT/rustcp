use std::ops::{BitAnd, BitOr, BitXor, Not, Rem, Shl, Shr};
use std::hash::Hash;
use crate::macros::should;
use crate::num_concrete::Concrete;
use crate::num_number::{Number, FromNumber};
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
    Concrete
    + Rem<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Ord
    + std::ops::ShrAssign
    + std::ops::ShlAssign
    + Hash
{
    type UnsignedIntegerType: Integer;
    type SignedIntegerType: Integer;
    type HighPrecisionIntegerType: Integer;
    const BITS: i32;

    fn as_high_precision_integer_type(&self) -> Self::HighPrecisionIntegerType {
        FromNumber::from(*self)
    }

    fn as_unsigned(&self) -> Self::UnsignedIntegerType {
        FromNumber::from(*self)
    }

    fn bit_count(&self) -> Self;
    fn higest_set_bit_offset(&self) -> i32;
    fn lowest_set_bit(&self) -> Self;
    fn higest_one_bit(&self) -> Self;
    fn count_leading_zero(&self) -> i32;
    fn count_trailing_zero(&self) -> i32;
    #[inline]
    fn modular(a: Self, m: Self) -> Self {
        let res = a % m;
        if res.is_negative() {
            res + m
        } else {
            res
        }
    }
    #[inline]
    fn sub_mod(a: Self, b: Self, m: Self) -> Self {
        if a < b {
            a - b + m
        } else {
            a - b
        }
    }
    #[inline]
    fn add_mod(a: Self, b: Self, m: Self) -> Self {
        let res = a + b;
        if res < Self::ZERO || res >= m {
            res - m
        } else {
            res
        }
    }
    fn mul_mod(a: Self, b: Self, m: Self) -> Self;
    fn pow_mod(a: Self, n: Self, m: Self) -> Self {
        if n == Self::ZERO {
            Self::ONE
        } else {
            let ans = Self::pow_mod(a, n >> Self::ONE, m);
            let ans = Self::mul_mod(ans, ans, m);
            if (n & Self::ONE) == Self::ONE {
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
        (a & b) + ((a ^ b) >> FromNumber::from(1))
    }
    fn average_ceil(a: Self, b: Self) -> Self {
        (a | b) - ((a ^ b) >> FromNumber::from(1))
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
            *self << FromNumber::from(step)
        }
    }
    fn kth_bit(&self, k: usize) -> Self {
        should!(k < Self::BITS as usize);
        (*self >> FromNumber::from(k)) & Self::ONE
    }
    fn bit_signed_right_shift(&self, step: i32) -> Self;
    fn bit_unsigned_right_shift(&self, step: i32) -> Self;

    fn mul_overflow(a: Self, b: Self) -> (Self, bool);
    fn div_and_remainder(a: Self, b: Self) -> (Self, Self);
}

macro_rules! IntegerImpl {
    ($t: ty, $h: ty, $u: ty, $s: ty) => {
        impl Integer for $t {
            type UnsignedIntegerType = $u;
            type HighPrecisionIntegerType = $h;
            type SignedIntegerType = $s;
            const BITS: i32 = <$t>::BITS as i32;
            #[inline(always)]
            fn count_trailing_zero(&self) -> i32 {
                let x = 0;
                if *self == <$t as Number>::ZERO {
                    <Self as Integer>::BITS
                } else {
                    <Self as Integer>::BITS - 1 - self.lowest_set_bit().count_leading_zero()
                }
            }
            #[inline(always)]
            fn bit_signed_right_shift(&self, step: i32) -> Self {
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
            #[inline(always)]
            fn bit_unsigned_right_shift(&self, step: i32) -> Self {
                if step >= <Self as Integer>::BITS {
                    Self::ZERO
                } else {
                    ((*self as Self::UnsignedIntegerType) >> (step as Self::UnsignedIntegerType)) as Self
                }
            }
            #[inline(always)]
            fn mul_mod(a: Self, b: Self, m: Self) -> Self {
                let mut res = ((a as Self::HighPrecisionType * b as Self::HighPrecisionType)
                    % (m as Self::HighPrecisionType)) as Self;
                if res.is_negative() {
                    res = res + m;
                }
                res
            }
            #[inline(always)]
            fn add_overflow(a: Self, b: Self) -> (Self, bool) {
                Self::overflowing_add(a, b)
            }
            #[inline(always)]
            fn mul_overflow(a: Self, b: Self) -> (Self, bool) {
                Self::overflowing_mul(a, b)
            }
            #[inline(always)]
            fn bit_count(&self) -> Self {
                self.count_ones() as $t
            }
            #[inline(always)]
            fn higest_set_bit_offset(&self) -> i32 {
                (Self::BITS - 1 - self.leading_zeros()) as i32
            }
            #[inline(always)]
            fn lowest_set_bit(&self) -> Self {
                *self & self.negative()
            }
            #[inline(always)]
            fn higest_one_bit(&self) -> Self {
                if *self == Self::ZERO {
                    0
                } else {
                    Self::ONE << <Self as FromNumber>::from(self.higest_set_bit_offset())
                }
            }
            #[inline(always)]
            fn count_leading_zero(&self) -> i32 {
                self.leading_zeros() as i32
            }
            #[inline(always)]
            fn div_and_remainder(a: Self, b: Self) -> (Self, Self) {
                let d = a / b;
                (d, a - d * b)
            }
        }
    };
}

IntegerImpl!(i8, i16, u8, i8);
IntegerImpl!(u8, i16, u8, i8);
IntegerImpl!(i16, i32, u16, i16);
IntegerImpl!(u16, u32, u16, i16);
IntegerImpl!(i32, i64, u32, i32);
IntegerImpl!(u32, u64, u32, i32);
IntegerImpl!(isize, isize, usize, isize);
IntegerImpl!(usize, usize, usize, isize);
IntegerImpl!(i64, i128, u64, i64);
IntegerImpl!(u64, u128, u64, i64);
IntegerImpl!(i128, i128, u128, i128);
IntegerImpl!(u128, u128, u128, i128);
