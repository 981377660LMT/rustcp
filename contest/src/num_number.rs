use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

pub trait FromNumber {
    fn from(num: impl Number) -> Self;
}

pub trait Number:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + PartialOrd
    + Display
    + Debug
    + FromStr
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + FromNumber
{
    const MAX: Self;
    const MIN: Self;
    const ZERO: Self;
    const ONE: Self;

    type HighPrecisionType: Number;
    type SignedType: Number;

    fn upgrade(&self) -> Self::HighPrecisionType {
        FromNumber::from(*self)
    }

    fn sign(&self) -> i8 {
        if self.is_negative() {
            -1
        } else if self.is_positive() {
            1
        } else {
            0
        }
    }
    fn negative(&self) -> Self {
        Self::ZERO - *self
    }
    fn is_negative(&self) -> bool {
        *self < Self::ZERO
    }
    fn is_positive(&self) -> bool {
        *self > Self::ZERO
    }
    fn is_non_negative(&self) -> bool {
        *self >= Self::ZERO
    }
    fn is_non_positive(&self) -> bool {
        *self <= Self::ZERO
    }
    fn absolute(&self) -> Self {
        if self.is_negative() {
            self.negative()
        } else {
            *self
        }
    }
    fn as_signed(&self) -> Self::SignedType;
    fn as_i8(&self) -> i8;
    fn as_u8(&self) -> u8;
    fn as_i16(&self) -> i16;
    fn as_u16(&self) -> u16;
    fn as_i32(&self) -> i32;
    fn as_u32(&self) -> u32;
    fn as_i64(&self) -> i64;
    fn as_u64(&self) -> u64;
    fn as_i128(&self) -> i128;
    fn as_u128(&self) -> u128;
    fn as_isize(&self) -> isize;
    fn as_usize(&self) -> usize;
    fn as_f32(&self) -> f32;
    fn as_f64(&self) -> f64;
}


macro_rules! Generator {
    ($t: ty, $as_method: ident, $H: ident) => {
        Generator!($t, $t, $as_method, $H);
    };
    ($t: ty, $s: ty, $as_method: ident, $H: ident) => {
        impl FromNumber for $t {
            #[inline(always)]
            fn from(num: impl Number) -> Self {
                num.$as_method()
            }
        }

        impl Number for $t {
            type SignedType = $s;
            type HighPrecisionType = $H;
            const MAX: Self = <$t>::MAX;
            const MIN: Self = <$t>::MIN;
            const ZERO: Self = 0 as Self;
            const ONE: Self = 1 as Self;
            #[inline(always)]
            fn absolute(&self) -> Self {
                if self.is_negative() {
                    self.negative()
                } else {
                    *self
                }
            }
            #[inline(always)]
            fn as_signed(&self) -> Self::SignedType {
                *self as Self::SignedType
            }

            fn as_i8(&self) -> i8 {
                *self as i8
            }
            fn as_u8(&self) -> u8 {
                *self as u8
            }
            fn as_i16(&self) -> i16 {
                *self as i16
            }
            fn as_u16(&self) -> u16 {
                *self as u16
            }
            fn as_i32(&self) -> i32 {
                *self as i32
            }
            fn as_u32(&self) -> u32 {
                *self as u32
            }
            fn as_i64(&self) -> i64 {
                *self as i64
            }
            fn as_u64(&self) -> u64 {
                *self as u64
            }
            fn as_i128(&self) -> i128 {
                *self as i128
            }
            fn as_u128(&self) -> u128 {
                *self as u128
            }
            fn as_isize(&self) -> isize {
                *self as isize
            }
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn as_f32(&self) -> f32 {
                *self as f32
            }
            fn as_f64(&self) -> f64 {
                *self as f64
            }
        }
    };
}

Generator!(usize, as_usize, u64);
Generator!(isize, as_isize, i64);

Generator!(i8, as_i8, i16);
Generator!(i16, as_i16, i32);
Generator!(i32, as_i32, i64);
Generator!(i64, as_i64, i128);
Generator!(i128, as_i128, i128);

Generator!(u8, i8, as_u8, u16);
Generator!(u16, i16, as_u16, u32);
Generator!(u32, i32, as_u32, u64);
Generator!(u64, i64, as_u64, u128);
Generator!(u128, i128, as_u128, u128);

Generator!(f32, as_f32, f64);
Generator!(f64, as_f64, f64);
