use std::{
    fmt::{Debug, Display, Pointer},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr, hash::Hasher, mem,
};
use std::hash::Hash;
use crate::{
    num_concrete::Concrete,
    num_number::{FromNumber, Number},
    num_real::Real,
};

#[derive(Clone, Copy)]
pub struct float(f64);

impl Real for float {
    const PI: Self = Self(3.1415926535897932384626433832795028841971693993751058209);

    const E: Self = Self(2.7182818284590452353602874713526624977572470936999595749);

    fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }
    fn powf(&self, exp: Self) -> Self {
        Self(self.0.powf(exp.0))
    }
    fn powi(&self, exp: i32) -> Self {
        Self(self.0.powi(exp))
    }
    fn sin(&self) -> Self {
        Self(self.0.sin())
    }
    fn cos(&self) -> Self {
        Self(self.0.cos())
    }

    fn tan(&self) -> Self {
        Self(self.0.tan())
    }
    fn asin(&self) -> Self {
        Self(self.0.asin())
    }
    fn acos(&self) -> Self {
        Self(self.0.acos())
    }
    fn atan(&self) -> Self {
        Self(self.0.atan())
    }
    fn exp(&self) -> Self {
        Self(self.0.exp())
    }
    fn ln(&self) -> Self {
        Self(self.0.ln())
    }

    fn round(&self) -> Self {
        Self(f64::round(self.0))
    }
    
}

impl float {
    pub const MAX: Self = Self(f64::MAX);
    pub const MIN: Self = Self(f64::MIN);
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);
}

impl Add for float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for float {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for float {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}
impl Mul for float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign for float {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl DivAssign for float {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}
impl Debug for float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
impl Display for float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl FromStr for float {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match f64::from_str(s) {
            Ok(x) => Ok(Self(x)),
            Err(_) => Err(()),
        }
    }
}
impl FromNumber for float {
    fn from(num: impl crate::num_number::Number) -> Self {
        Self(num.as_f64())
    }
}
impl From<f64> for float {
    fn from(x: f64) -> Self {
        Self(x)
    }
}
impl From<f32> for float {
    fn from(x: f32) -> Self {
        Self(x as f64)
    }
}
impl PartialEq for float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for float {}
impl PartialOrd for float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Number for float {
    const MAX: Self = float::MAX;

    const MIN: Self = float::MIN;

    const ZERO: Self = float::ZERO;

    const ONE: Self = float::ONE;

    type HighPrecisionType = float;

    type SignedType = float;

    fn upgrade(&self) -> Self::HighPrecisionType {
        *self
    }

    fn as_signed(&self) -> Self::SignedType {
        FromNumber::from(self.0)
    }

    fn as_i8(&self) -> i8 {
        self.0.as_i8()
    }

    fn as_u8(&self) -> u8 {
        self.0.as_u8()
    }

    fn as_i16(&self) -> i16 {
        self.0.as_i16()
    }

    fn as_u16(&self) -> u16 {
        self.0.as_u16()
    }

    fn as_i32(&self) -> i32 {
        self.0.as_i32()
    }

    fn as_u32(&self) -> u32 {
        self.0.as_u32()
    }

    fn as_i64(&self) -> i64 {
        self.0.as_i64()
    }

    fn as_u64(&self) -> u64 {
        self.0.as_u64()
    }

    fn as_i128(&self) -> i128 {
        self.0.as_i128()
    }

    fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }

    fn as_isize(&self) -> isize {
        self.0.as_isize()
    }

    fn as_usize(&self) -> usize {
        self.0.as_usize()
    }

    fn as_f32(&self) -> f32 {
        self.0.as_f32()
    }

    fn as_f64(&self) -> f64 {
        self.0.as_f64()
    }

    
}

impl Concrete for float {}
impl Hash for float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits: u64 = unsafe {
            mem::transmute(self.0)
        };
        bits.hash(state)
    }
}

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}