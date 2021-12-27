use std::{ops::{Add, Sub, Div, Mul}, fmt::{Display, Debug}, str::FromStr};


pub trait Number: Copy + Add<Output = Self> + Sub<Output = Self> + 
Mul<Output = Self> + Div<Output = Self> + PartialEq + PartialOrd + 
Display + Debug + FromStr
{
    const MAX: Self;
    const MIN: Self; 
    const ZERO: Self;
    const ONE: Self;

    type HighPrecisionType: Number;
    type SignedType: Number;

    fn upgrade(&self) -> Self::HighPrecisionType {
        Self::HighPrecisionType::from_number(*self)
    }

    fn as_i8(&self) -> i8;
    fn as_i16(&self) -> i16;
    fn as_i32(&self) -> i32;
    fn as_i64(&self) -> i64;
    fn as_i128(&self) -> i128;
    fn as_u8(&self) -> u8;
    fn as_u16(&self) -> u16;
    fn as_u32(&self) -> u32;
    fn as_u64(&self) -> u64;
    fn as_u128(&self) -> u128;
    fn as_f32(&self) -> f32;
    fn as_f64(&self) -> f64;
    fn as_isize(&self) -> isize;
    fn as_usize(&self) -> usize;
    fn from_i8(x: i8) -> Self;
    fn from_i16(x: i16) -> Self;
    fn from_i32(x: i32) -> Self; 
    fn from_i64(x: i64) -> Self;
    fn from_i128(x: i128) -> Self;
    fn from_isize(x: isize) -> Self;
    fn from_u8(x: u8) -> Self;
    fn from_u16(x: u16) -> Self;
    fn from_u32(x: u32) -> Self; 
    fn from_u64(x: u64) -> Self;
    fn from_u128(x: u128) -> Self;
    fn from_usize(x: usize) -> Self;
    fn from_f64(x: f64) -> Self;
    fn from_f32(x: f32) -> Self;
    fn from_number(x: impl Number) -> Self;
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
}

macro_rules! Generator {
    ($t: ty, $as_method: ident, $H: ident) => {
        Generator!($t, $t, $as_method, $H);
    };
    ($t: ty, $s: ty, $as_method: ident, $H: ident) => {
        
        impl Number for $t {
            type SignedType = $s;
            type HighPrecisionType = $H;
            const MAX: Self = <$t>::MAX;
            const MIN: Self = <$t>::MIN;
            const ZERO: Self = 0 as Self;
            const ONE: Self = 1 as Self;
            
            fn from_number(x: impl Number) -> Self{
                x.$as_method()
            }

            fn as_i8(&self) -> i8 {
                *self as i8
            }
        
            fn as_i16(&self) -> i16 {
                *self as i16
            }
        
            fn as_i32(&self) -> i32 {
                *self as i32
            }
        
            fn as_i64(&self) -> i64 {
                *self as i64
            }
        
            fn as_i128(&self) -> i128 {
                *self as i128
            }
        
            fn as_f32(&self) -> f32 {
                *self as f32
            }
        
            fn as_f64(&self) -> f64 {
                *self as f64
            }
        
            fn as_isize(&self) -> isize {
                *self as isize
            }
        
            fn absolute(&self) -> Self {
                if self.is_negative() {
                    self.negative()
                }else{
                    *self
                }
            }
        
            fn from_i8(x: i8) -> Self {
                x as Self
            }
        
            fn from_i16(x: i16) -> Self {
                x as Self
            }
        
            fn from_i32(x: i32) -> Self {
                x as Self
            }
        
            fn from_i64(x: i64) -> Self {
                x as Self
            }
        
            fn from_i128(x: i128) -> Self {
                x as Self
            }
        
            fn from_isize(x: isize) -> Self {
                x as Self
            }
        
            fn from_f64(x: f64) -> Self {
                x as Self
            }
        
            fn from_f32(x: f32) -> Self {
                x as Self
            }
        
            fn as_u8(&self) -> u8 {
                *self as u8
            }
        
            fn as_u16(&self) -> u16 {
                *self as u16
            }
        
            fn as_u32(&self) -> u32 {
                *self as u32
            }
        
            fn as_u64(&self) -> u64 {
                *self as u64
            }
        
            fn as_u128(&self) -> u128 {
                *self as u128
            }
        
            fn as_usize(&self) -> usize {
                *self as usize
            }
        
            fn from_u8(x: u8) -> Self {
                x as Self
            }
        
            fn from_u16(x: u16) -> Self {
                x as Self
            }
        
            fn from_u32(x: u32) -> Self {
                x as Self
            }
        
            fn from_u64(x: u64) -> Self {
                x as Self
            }
        
            fn from_u128(x: u128) -> Self {
                x as Self
            }
        
            fn from_usize(x: usize) -> Self {
                x as Self
            }
        
            fn as_signed(&self) -> Self::SignedType {
                *self as Self::SignedType
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

