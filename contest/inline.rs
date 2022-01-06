//Timestamp: 2022-01-06 01:11:12
pub mod fast_input {
    use std::io;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Cursor;

    pub struct FastInput<R: std::io::BufRead> {
        inner: R,
        line: Vec<u8>,
        offset: usize,
    }

    pub fn new_fastinput_from_string(s: String) -> FastInput<BufReader<Cursor<String>>> {
        let x = FastInput::new(BufReader::new(Cursor::new(s)));
        x
    }

    pub fn new_fastinput_from_string_ref(s: &String) -> FastInput<BufReader<Cursor<&String>>> {
        let x = FastInput::new(BufReader::new(Cursor::new(s)));
        x
    }

    impl<R: std::io::BufRead> FastInput<R> {
        pub fn new(inner: R) -> Self {
            FastInput {
                inner,
                line: Vec::with_capacity(1 << 15),
                offset: 0,
            }
        }

        fn next(&mut self) -> Option<&str> {
            self.skip_blank();
            if self.offset == self.line.len() {
                return None;
            }
            let begin = self.offset;
            let mut end = self.offset + 1;
            while end < self.line.len() && self.line[end] > 32 {
                end += 1;
            }
            self.offset = end;
            unsafe { Some(std::str::from_utf8_unchecked(&self.line[begin..end])) }
        }

        fn skip_blank(&mut self) {
            while self.offset < self.line.len() && self.line[self.offset] <= 32 {
                self.offset += 1;
            }
        }

        pub fn eof(&mut self) -> bool {
            loop {
                self.skip_blank();
                if self.offset < self.line.len() {
                    return false;
                }
                if !self.refill() {
                    return true;
                }
            }
        }

        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            loop {
                match self.next() {
                    Some(token) => {
                        return token.parse().ok().expect("Wrong format input");
                    }
                    None => {
                        self.refill();
                    }
                }
            }
        }

        fn refill(&mut self) -> bool {
            self.line.clear();
            let num = self.inner.read_until(b'\n', &mut self.line).unwrap();
            self.offset = 0;
            return num > 0;
        }

        pub fn r<T: std::str::FromStr>(&mut self) -> T {
            self.read()
        }

        pub fn ri(&mut self) -> i32 {
            let res: i32 = self.read();
            return res;
        }

        pub fn rl(&mut self) -> i64 {
            let res: i64 = self.read();
            return res;
        }

        pub fn rf(&mut self) -> f64 {
            let res: f64 = self.read();
            return res;
        }

        pub fn rs(&mut self) -> String {
            let res: String = self.read();
            return res;
        }

        pub fn ru(&mut self) -> usize {
            let res: usize = self.read();
            return res;
        }
    }
}
pub mod collection {
    use std::collections::HashMap;
    use std::mem::swap;

    pub fn group_by_dense<T, V>(
        n: usize,
        data: &[T],
        to_key: impl Fn(usize, &T) -> usize,
        to_value: impl Fn(usize, &T) -> V,
    ) -> Vec<Vec<V>> {
        let mut sizes = vec![0usize; n];
        for (index, x) in data.iter().enumerate() {
            sizes[to_key(index, x)] += 1;
        }
        let mut res: Vec<Vec<V>> = sizes.iter().map(|&x| Vec::with_capacity(x)).collect();
        for (index, x) in data.iter().enumerate() {
            res[to_key(index, x)].push(to_value(index, x));
        }

        res
    }

    pub fn swap_element<T>(data: &mut [T], a: usize, b: usize) {
        if a > b {
            let (p1, p2) = data.split_at_mut(a);
            swap(&mut p1[b], &mut p2[0]);
        } else if b > a {
            let (p1, p2) = data.split_at_mut(b);
            swap(&mut p1[a], &mut p2[0]);
        }
    }

    pub fn swap_element_attr<T, V>(
        data: &mut [T],
        a: usize,
        b: usize,
        extractor: impl Fn(&mut T) -> &mut V,
    ) {
        if a > b {
            let (p1, p2) = data.split_at_mut(a);
            swap(extractor(&mut p1[b]), extractor(&mut p2[0]));
        } else if b > a {
            let (p1, p2) = data.split_at_mut(b);
            swap(extractor(&mut p1[a]), extractor(&mut p2[0]));
        }
    }
}
pub mod macros {

    #[cfg(feature = "local-build")]
    macro_rules! should {
    ($($e: expr),*) => {
        $(
            assert!($e);
        )*
    }
}

    #[cfg(not(feature = "local-build"))]
    macro_rules! should {
        ($($e: expr),*) => {};
    }

    #[cfg(feature = "local-build")]
    macro_rules! should_eq {
    ($($a: expr, $b: expr);*) => {
        $(
            assert_eq!($a, $b);
        )*
    }
}

    #[cfg(not(feature = "local-build"))]
    macro_rules! should_eq {
        ($($e: expr),*) => {};
    }

    #[cfg(feature = "local-build")]
    macro_rules! debug {
        ($e: expr) => {
            dbg!($e)
        };
    }

    #[cfg(not(feature = "local-build"))]
    macro_rules! debug {
        ($e: expr) => {
            std::convert::identity($e)
        };
    }

    #[cfg(feature = "local-build")]
    macro_rules! debug_discard {
        ($e: expr) => {
            dbg!($e)
        };
    }

    #[cfg(not(feature = "local-build"))]
    macro_rules! debug_discard {
        ($e: expr) => {};
    }

    macro_rules! input {

    ($fi:ident, $var:ident $( : $t:ty)?, $($arg:tt)*) => {
        let mut $var $(: $t)? = $fi.read();
        input!($fi, $($arg)*)
    };

    ($fi: ident $(,)?) => {
    };
}

    macro_rules! MergerImpl {
        ($name: ident, $A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
            struct $name;
            impl Merger<$A, $B, $C> for $name {
                fn merge($a: $A, $b: $B) -> $C {
                    $body
                }
            }
        };
    }

    macro_rules! AddImpl {
        ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
            impl Add<$B> for $A {
                type Output = $C;

                fn add(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $body
                }
            }
        };
    }

    macro_rules! SubImpl {
        ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
            impl Mul<$B> for $A {
                type Output = $C;

                fn sub(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $body
                }
            }
        };
    }

    macro_rules! DivImpl {
        ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
            impl Mul<$B> for $A {
                type Output = $C;

                fn div(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $body
                }
            }
        };
    }

    macro_rules! MulImpl {
        ($A: ty, $B: ty, $C: ty, $a: ident, $b: ident, $body: tt) => {
            impl Sub<$B> for $A {
                type Output = $C;

                fn mul(self, $b: $B) -> Self::Output {
                    let $a = self;
                    $body
                }
            }
        };
    }

    macro_rules! SwapAttribute {
        ($a: expr, $b: expr) => {{
            let mut _tmp = std::mem::take(&mut $a);
            std::mem::swap(&mut $b, &mut _tmp);
            std::mem::swap(&mut $a, &mut _tmp);
        }};
        ($a: expr, $b: expr, $def: expr) => {{
            let mut _tmp = std::mem::replace(&mut $a, $def);
            std::mem::swap(&mut $b, &mut _tmp);
            std::mem::swap(&mut $a, &mut _tmp);
        }};
    }

    pub(crate) use debug;
    pub(crate) use debug_discard;
    pub(crate) use input;
    pub(crate) use should;
    pub(crate) use should_eq;
    pub(crate) use AddImpl;
    pub(crate) use DivImpl;
    pub(crate) use MergerImpl;
    pub(crate) use MulImpl;
    pub(crate) use SubImpl;
    pub(crate) use SwapAttribute;
}
pub mod num_number {
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::ops::Div;
    use std::ops::DivAssign;
    use std::ops::Mul;
    use std::ops::MulAssign;
    use std::ops::Sub;
    use std::ops::SubAssign;
    use std::str::FromStr;

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
}
pub mod num_concrete {
    use crate::num_number::Number;
    use std::hash::Hash;

    pub trait Concrete: Number + Eq + Ord + Hash {}

    macro_rules! Concrete {
        ($name: ident) => {
            impl Concrete for $name {}
        };
    }

    Concrete!(i8);
    Concrete!(u8);
    Concrete!(i16);
    Concrete!(u16);
    Concrete!(i32);
    Concrete!(u32);
    Concrete!(i64);
    Concrete!(u64);
    Concrete!(i128);
    Concrete!(u128);
    Concrete!(isize);
    Concrete!(usize);
}
pub mod num_real {
    use crate::num_concrete::Concrete;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;

    pub trait Real: Concrete {
        const PI: Self;
        const E: Self;

        fn average(a: Self, b: Self) -> Self {
            (a + b) / FromNumber::from(2)
        }
        fn sqrt(&self) -> Self;
        fn powf(&self, b: Self) -> Self;
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
}
pub mod num_float {
    use crate::num_concrete::Concrete;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;
    use crate::num_real::Real;
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::fmt::Pointer;
    use std::hash::Hash;
    use std::hash::Hasher;
    use std::mem;
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::ops::Div;
    use std::ops::DivAssign;
    use std::ops::Mul;
    use std::ops::MulAssign;
    use std::ops::Sub;
    use std::ops::SubAssign;
    use std::str::FromStr;

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
            let bits: u64 = unsafe { mem::transmute(self.0) };
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
}
pub mod num_integer {
    use crate::macros::should;
    use crate::num_concrete::Concrete;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;
    use std::hash::Hash;
    use std::ops::BitAnd;
    use std::ops::BitOr;
    use std::ops::BitXor;
    use std::ops::Not;
    use std::ops::Rem;
    use std::ops::Shl;
    use std::ops::Shr;

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
                        ((*self as Self::UnsignedIntegerType)
                            >> (step as Self::UnsignedIntegerType)) as Self
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
}
pub mod arithmetic {
    use crate::macros::should;
    use crate::num_float::float;
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;
    use crate::num_real::Real;
    use std::fmt::Debug;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Sub;

    pub trait CommutativeAdd: Add<Output = Self> + Clone + Debug {}
    pub trait AssociativeAdd: Add<Output = Self> + Clone + Debug {}
    pub trait IdentityAdd: Add<Output = Self> + Clone + Debug {
        fn zero() -> Self;
    }
    pub trait CommutativeMul: Mul<Output = Self> + Clone + Debug {}
    pub trait AssociativeMul: Mul<Output = Self> + Clone + Debug {}
    pub trait IdentityMul: Mul<Output = Self> + Clone + Debug {
        fn one() -> Self;
    }
    pub trait IdempotentAdd: CommutativeAdd + AssociativeAdd {}
    pub trait IdempotentMul: CommutativeMul + AssociativeMul {}
    pub trait IntegralMul: Mul<Output = Self> + Clone + Debug {}
    impl<T> IntegralMul for T where T: Div<Output = Self> + Mul<Output = Self> + Clone + Debug {}

    pub trait PreferDiv: Div<Output = Self> + Clone + Debug {
        fn div_ceil(a: Self, b: Self) -> Self {
            a / b
        }
        fn div_floor(a: Self, b: Self) -> Self {
            a / b
        }
    }

    impl PreferDiv for f32 {}
    impl PreferDiv for f64 {}
    impl<T: Integer> PreferDiv for T {
        fn div_ceil(a: Self, b: Self) -> Self {
            should!(b >= Self::ZERO);
            let res = a / b;
            if res * b > a {
                res - Self::ONE
            } else {
                res
            }
        }
        fn div_floor(a: Self, b: Self) -> Self {
            should!(b >= Self::ZERO);
            let res = a / b;
            if res * b < a {
                res + Self::ONE
            } else {
                res
            }
        }
    }

    pub trait LowerBound: PartialOrd {
        fn min_element() -> Self;
    }

    pub trait UpperBound: PartialOrd {
        fn max_element() -> Self;
    }

    pub trait MulInv: Mul + Clone + Debug {
        fn possible_inv(&self) -> Option<Self>;
    }

    macro_rules! AddTagImpl {
        ($t: ty, $zero: expr) => {
            impl CommutativeAdd for $t {}
            impl IdentityAdd for $t {
                fn zero() -> Self {
                    $zero
                }
            }
            impl AssociativeAdd for $t {}
            impl IdempotentAdd for $t {}
        };
    }

    macro_rules! MulTagImpl {
        ($t: ty, $one: expr) => {
            impl CommutativeMul for $t {}
            impl IdentityMul for $t {
                fn one() -> Self {
                    $one
                }
            }
            impl AssociativeMul for $t {}
            impl IdempotentMul for $t {}
        };
    }

    macro_rules! AddMulTagImpl {
        ($t: ty, $zero: expr, $one: expr) => {
            AddTagImpl!($t, $zero);
            MulTagImpl!($t, $one);
        };
    }

    impl<T> CommutativeAdd for T where T: Number {}
    impl<T> IdentityAdd for T
    where
        T: Number,
    {
        fn zero() -> Self {
            <T as Number>::ZERO
        }
    }
    impl<T> AssociativeAdd for T where T: Number {}
    impl<T> CommutativeMul for T where T: Number {}
    impl<T> IdentityMul for T
    where
        T: Number,
    {
        fn one() -> Self {
            <T as Number>::ONE
        }
    }
    impl MulInv for f32 {
        fn possible_inv(&self) -> Option<Self> {
            if *self == f32::ZERO {
                None
            } else {
                Some(1f32 / *self)
            }
        }
    }
    impl MulInv for f64 {
        fn possible_inv(&self) -> Option<Self> {
            if *self == f64::ZERO {
                None
            } else {
                Some(1f64 / *self)
            }
        }
    }
    impl<T: Real> MulInv for T {
        fn possible_inv(&self) -> Option<Self> {
            if *self == T::zero() {
                None
            } else {
                Some(<T as FromNumber>::from(1) / *self)
            }
        }
    }
    impl<T> AssociativeMul for T where T: Number {}
    impl<T> LowerBound for T
    where
        T: Number,
    {
        fn min_element() -> Self {
            <T as Number>::MIN
        }
    }
    impl<T> UpperBound for T
    where
        T: Number,
    {
        fn max_element() -> Self {
            <T as Number>::MAX
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Nil;
    impl Mul for Nil {
        type Output = Nil;

        fn mul(self, rhs: Nil) -> Self::Output {
            Nil
        }
    }
    impl Add for Nil {
        type Output = Nil;

        fn add(self, rhs: Nil) -> Self::Output {
            Nil
        }
    }
    impl Sub for Nil {
        type Output = Nil;

        fn sub(self, rhs: Nil) -> Self::Output {
            Nil
        }
    }

    impl Div for Nil {
        type Output = Nil;

        fn div(self, rhs: Nil) -> Self::Output {
            Nil
        }
    }
    impl PartialEq for Nil {
        fn eq(&self, other: &Self) -> bool {
            true
        }
    }
    impl Eq for Nil {}
    impl PartialOrd for Nil {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(std::cmp::Ordering::Equal)
        }
    }
    impl LowerBound for Nil {
        fn min_element() -> Self {
            Nil
        }
    }
    impl UpperBound for Nil {
        fn max_element() -> Self {
            Nil
        }
    }
    AddMulTagImpl!(Nil, Nil, Nil);
    pub(crate) use AddMulTagImpl;
    pub(crate) use AddTagImpl;
    pub(crate) use MulTagImpl;
}
pub mod algebraic_structure {
    use crate::arithmetic::*;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Sub;

    pub trait Magma: Add<Output = Self> + Copy + Eq {}
    impl<T> Magma for T where T: Add<Output = Self> + Copy + Eq {}
    pub trait Semigroup: Magma + AssociativeAdd {}
    impl<T> Semigroup for T where T: Magma + AssociativeAdd {}
    pub trait Monoid: Semigroup + IdentityAdd {}
    impl<T> Monoid for T where T: Semigroup + IdentityAdd {}
    pub trait Group: Monoid + Sub<Output = Self> {}
    impl<T> Group for T where T: Monoid + Sub<Output = Self> {}
    pub trait AbelianGroup: Group + CommutativeAdd {}
    impl<T> AbelianGroup for T where T: Group + CommutativeAdd {}
    pub trait Ring: AbelianGroup + Mul<Output = Self> + IdentityMul {}
    impl<T> Ring for T where T: AbelianGroup + Mul<Output = Self> + IdentityMul {}
    pub trait CommutativeRing: Ring + CommutativeMul {}
    impl<T> CommutativeRing for T where T: Ring + CommutativeMul {}
    pub trait IntegralDomain: CommutativeRing + IntegralMul {}
    impl<T> IntegralDomain for T where T: CommutativeRing + IntegralMul {}
    pub trait Field: IntegralDomain + Div<Output = Self> + MulInv {}
    impl<T> Field for T where T: IntegralDomain + Div<Output = Self> + MulInv {}
}
pub mod binary_search {
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::num_real::Real;

    pub fn first_true<T>(mut l: T, mut r: T, f: impl Fn(&T) -> bool) -> Option<T>
    where
        T: Integer,
    {
        if l > r {
            return None;
        }
        while l < r {
            let m = T::average_floor(l, r);
            if f(&m) {
                r = m;
            } else {
                l = m + T::ONE;
            }
        }
        if f(&l) {
            Some(l)
        } else {
            None
        }
    }

    pub fn last_true<T>(mut l: T, mut r: T, f: impl Fn(&T) -> bool) -> Option<T>
    where
        T: Integer,
    {
        if l > r {
            return None;
        }
        while l < r {
            let m = T::average_ceil(l, r);
            if f(&m) {
                l = m;
            } else {
                r = m - T::ONE;
            }
        }
        if f(&l) {
            Some(l)
        } else {
            None
        }
    }

    pub fn first_true_float<T>(
        mut round: u8,
        mut l: T,
        mut r: T,
        f: impl Fn(&T) -> bool,
    ) -> Option<T>
    where
        T: Real,
    {
        if l > r {
            return None;
        }
        while round > 0 {
            round -= 1;
            let m = (l + r) / <T as FromNumber>::from(2);
            if f(&m) {
                r = m;
            } else {
                l = m;
            }
        }
        if f(&l) {
            Some(l)
        } else {
            None
        }
    }

    pub fn last_true_real<T>(mut round: u8, mut l: T, mut r: T, f: impl Fn(&T) -> bool) -> Option<T>
    where
        T: Real,
    {
        if l > r {
            return None;
        }
        while round > 0 {
            round -= 1;
            let m = (l + r) / <T as FromNumber>::from(2);
            if f(&m) {
                l = m;
            } else {
                r = m;
            }
        }
        if f(&l) {
            Some(l)
        } else {
            None
        }
    }
}
pub mod num_gcd {
    use crate::macros::should;
    use crate::macros::should_eq;
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;

    pub fn gcd<T>(a: T, b: T) -> T
    where
        T: Integer,
    {
        should! {a.is_non_negative(), b.is_non_negative()};
        let mut s = (a, b);
        while s.1 > T::ZERO {
            s = (s.1, s.0 % s.1);
        }
        s.0
    }

    pub fn extgcd<T>(a: T, b: T) -> (T, T, T)
    where
        T: Integer,
    {
        should!(a.is_non_negative(), b.is_non_negative());
        if b == T::ZERO {
            (T::ONE, T::ZERO, a)
        } else {
            let div_and_rem = T::div_and_remainder(a, b);
            let ans = extgcd(b, div_and_rem.1);
            (ans.1, ans.0 - div_and_rem.0 * ans.1, ans.2)
        }
    }

    pub fn inv_mod<T>(a: T, m: T) -> Option<T>
    where
        T: Integer,
    {
        let a1: T::SignedIntegerType = FromNumber::from(a);
        let m1: T::SignedIntegerType = FromNumber::from(m);
        let res = extgcd(a1, m1);
        if res.2 == T::SignedIntegerType::ONE {
            let res = T::from(T::SignedIntegerType::modular(res.0, m1));
            should_eq!(T::mul_mod(res, a, m), T::ONE);
            Some(res)
        } else {
            None
        }
    }
}
pub mod math {
    use crate::algebraic_structure::Field;
    use crate::algebraic_structure::Ring;
    use crate::arithmetic::IdentityMul;
    use crate::binary_search::first_true;
    use crate::binary_search::last_true;
    use crate::num_gcd::gcd;
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use std::cmp::Ordering;

    pub fn pow<T, E>(x: T, n: E) -> T
    where
        T: Ring,
        E: Integer,
    {
        if n == E::ZERO {
            return <T as IdentityMul>::one();
        }
        let ans = pow(x, n >> E::ONE);
        let ans = ans * ans;
        if (n & E::ONE) == E::ONE {
            ans * x
        } else {
            ans
        }
    }

    pub fn log2_floor<T>(x: T) -> i32
    where
        T: Integer,
    {
        let leading_zero = x.count_leading_zero();
        T::BITS - leading_zero - 1
    }
    pub fn log2_ceil<T>(x: T) -> i32
    where
        T: Integer,
    {
        let res = log2_floor(x);
        if res < 0 || (T::ONE << FromNumber::from(res)) < x {
            res + 1
        } else {
            res
        }
    }

    pub fn argmax<'a, T: Ord>(data: &'a [T]) -> Option<(usize, &'a T)> {
        data.iter().enumerate().max_by_key(|(_, x)| *x)
    }

    pub fn argmin<'a, T: Ord>(data: &'a [T]) -> Option<(usize, &'a T)> {
        data.iter().enumerate().min_by_key(|(_, x)| *x)
    }

    pub fn argmax_by<'a, T>(
        data: &'a [T],
        f: &mut impl FnMut(&T, &T) -> Ordering,
    ) -> Option<(usize, &'a T)> {
        data.iter().enumerate().max_by(|(_, x), (_, y)| f(x, y))
    }

    pub fn argmin_by<'a, T>(
        data: &'a [T],
        f: &mut impl FnMut(&T, &T) -> Ordering,
    ) -> Option<(usize, &'a T)> {
        data.iter().enumerate().min_by(|(_, x), (_, y)| f(x, y))
    }

    pub fn sqrt_floor<T: Integer>(x: T) -> Option<T> {
        if x < T::ZERO {
            None
        } else {
            let x = x.as_unsigned();
            let limit: T = <T as FromNumber>::from(1) << <T as FromNumber>::from(T::BITS / 2);
            last_true(T::ZERO, limit - T::ONE, |t| (*t * *t).as_unsigned() <= x)
        }
    }

    pub fn sqrt_ceil<T: Integer>(x: T) -> Option<T> {
        if x < T::ZERO {
            None
        } else {
            let x = x.as_unsigned();
            let limit: T = <T as FromNumber>::from(1) << FromNumber::from(T::BITS / 2);
            match first_true(T::ZERO, limit - T::ONE, |t| (*t * *t).as_unsigned() >= x) {
                None => Some(limit + T::ONE),
                Some(x) => Some(x),
            }
        }
    }

    pub fn inverse_batch<T: Field>(mut data: &[T]) -> Vec<T> {
        if data.is_empty() {
            return Vec::new();
        }
        let n = data.len();
        let mut res = data.to_owned();
        for i in 1..n {
            res[i] = res[i - 1] * res[i];
        }
        let mut inv = T::one() / res[n - 1];
        for i in (1..n).rev() {
            res[i] = inv * res[i - 1];
            inv = inv * data[i];
        }
        res[0] = inv;
        res
    }

    pub fn max_batch<'a, T: Ord>(a: &'a [T]) -> Option<&'a T> {
        a.iter().max()
    }
    pub fn min_batch<'a, T: Ord>(a: &'a [T]) -> Option<&'a T> {
        a.iter().min()
    }
    pub fn dot_mul<T: Ring>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
        a.iter().zip(b.iter()).map(|(x, y)| *x * *y).collect()
    }
    pub fn dot_mul_plus<T: Ring>(a: &Vec<T>, b: &Vec<T>, dest: &mut Vec<T>) {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| *x * *y)
            .enumerate()
            .for_each(|(index, v)| dest[index] = dest[index] + v);
    }
}
pub mod modint {
    use crate::algebraic_structure::CommutativeRing;
    use crate::arithmetic::MulInv;
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;
    use std::hash::Hash;
    use std::str::FromStr;
    use std::string::ParseError;

    pub trait ModInt<T: Integer>: CommutativeRing + FromNumber + MulInv + Hash {
        fn modulus() -> T;
        fn primitive_root() -> Option<Self>;
        fn value(&self) -> T;
    }
    pub fn modint_sum_batch<I: Integer, T: ModInt<I>>(a: &[T], b: &[T]) -> T {
        let modulus = FromNumber::from(T::modulus());
        let max_allow = I::HighPrecisionIntegerType::MAX
            - (modulus - I::HighPrecisionIntegerType::ONE)
                * (modulus - I::HighPrecisionIntegerType::ONE);
        let mut sum = I::HighPrecisionIntegerType::ZERO;
        a.iter().zip(b.iter()).for_each(|(x, y)| {
            let prod = <I::HighPrecisionIntegerType as FromNumber>::from(x.value())
                * <I::HighPrecisionIntegerType as FromNumber>::from(y.value());
            sum = sum + prod;
            if sum > max_allow {
                sum = sum % modulus;
            }
        });
        sum = sum % modulus;
        FromNumber::from(sum)
    }
}
pub mod poly_common {
    use crate::algebraic_structure::Field;
    use crate::algebraic_structure::Ring;
    use crate::math::log2_ceil;
    use crate::num_number::FromNumber;

    pub fn poly_extend<T: Ring>(mut p: Vec<T>, len: usize) -> Vec<T> {
        p.resize(len, T::zero());
        p
    }

    pub fn poly_modular<T: Ring>(mut p: Vec<T>, len: usize) -> Vec<T> {
        if p.len() <= len {
            p
        } else {
            poly_trim(poly_extend(p, len))
        }
    }

    pub fn poly_modular_ref<T: Ring>(p: &Vec<T>, len: usize) -> Vec<T> {
        if p.len() <= len {
            p.clone()
        } else {
            poly_trim((0..len).map(|i| p[i]).collect())
        }
    }

    pub fn poly_length(n: usize) -> usize {
        1 << log2_ceil(n)
    }

    pub fn poly_trim<T: Ring>(mut p: Vec<T>) -> Vec<T> {
        let zero = T::zero();
        while p.len() > 1 && *p.last().unwrap() == zero {
            p.pop();
        }
        if p.is_empty() {
            p.push(zero);
        }
        p
    }

    pub fn poly_evaluate<T: Ring>(p: &Vec<T>, x: T) -> T {
        let mut res = T::zero();
        for i in p.iter().rev() {
            res = res * x + *i;
        }
        res
    }

    pub fn poly_div_and_rem<T: Field>(mut a: Vec<T>, mut b: Vec<T>) -> (Vec<T>, Vec<T>) {
        b.reverse();
        let inv_first = b.first().unwrap().possible_inv().unwrap();
        let n = a.len();
        let m = b.len();
        let mut divisor = vec![T::zero(); n - m + 1];
        for i in (m - 1..n).into_iter().rev() {
            if a[i] == T::zero() {
                continue;
            }
            let factor = a[i] * inv_first;
            divisor[i - (m - 1)] = factor;
            for j in 0..m {
                a[i - j] = a[i - j] - b[j] * factor;
            }
        }
        (poly_trim(divisor), poly_trim(a))
    }

    pub fn convolution_brute_force<T: Ring>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        let a = poly_trim(a);
        let b = poly_trim(b);
        let n = a.len();
        let m = b.len();
        let mut c = vec![T::zero(); n + m - 1];
        for i in 0..n {
            for j in 0..m {
                c[i + j] = c[i + j] + a[i] * b[j];
            }
        }
        c
    }
}
pub mod poly {
    use crate::algebraic_structure::Field;
    use crate::algebraic_structure::Ring;
    use crate::macros::should;
    use crate::macros::should_eq;
    use crate::math::inverse_batch;
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::poly_common::poly_evaluate;
    use crate::poly_common::poly_extend;
    use crate::poly_common::poly_length;
    use crate::poly_common::poly_modular;
    use crate::poly_common::poly_modular_ref;
    use crate::poly_common::poly_trim;
    use std::cmp::max;
    use std::fmt::Debug;
    use std::marker::PhantomData;
    use std::mem::take;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Index;
    use std::ops::Mul;
    use std::ops::Rem;
    use std::ops::Sub;

    pub trait Convolution<T: Ring> {
        fn convolution(a: Vec<T>, b: Vec<T>) -> Vec<T>;

        fn pow2(a: Vec<T>) -> Vec<T> {
            let b = a.clone();
            Self::convolution(a, b)
        }
    }

    pub trait PolyInverse<T: Field + FromNumber>: Convolution<T> {
        fn inverse(a: Vec<T>, n: usize) -> Vec<T> {
            poly_trim(Self::inverse_internal(&poly_extend(a, n)[..]))
        }
        fn inverse_internal(p: &[T]) -> Vec<T> {
            if p.len() == 1 {
                return vec![T::one() / p[0]];
            }
            let m = p.len();
            let prev_mod = (m + 1) / 2;
            let proper_len = poly_length(m);
            let C = Self::inverse_internal(p.split_at(prev_mod).0);
            let C = poly_extend(C, proper_len);
            let A = p.to_owned();
            let A = poly_extend(A, proper_len);

            let mut AC = poly_extend(Self::convolution(A, C.clone()), m);
            let zero = T::zero();
            for i in 0..m {
                AC[i] = zero - AC[i];
            }
            AC[0] = AC[0] + <T as FromNumber>::from(2);
            poly_extend(Self::convolution(C, AC), m)
        }
    }

    #[derive(Eq)]
    pub struct Poly<T: Ring, C: Convolution<T>>(Vec<T>, PhantomData<(T, C)>);
    impl<T: Ring, C: Convolution<T>> PartialEq for Poly<T, C> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<T: Ring, C: Convolution<T>> Clone for Poly<T, C> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), PhantomData)
        }
    }
    impl<T: Ring, C: Convolution<T>> Debug for Poly<T, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Poly").field(&self.0).finish()
        }
    }

    impl<T: Field + FromNumber, C: PolyInverse<T>> Poly<T, C> {
        pub fn divider_and_remainder(self, rhs: Self) -> (Self, Self) {
            let a = self.clone() / rhs.clone();
            (a.clone(), self - a * rhs)
        }

        pub fn integral(&self) -> Self {
            let rank = self.rank();
            let p = &self.0;
            let range: Vec<T> = (1..rank + 2).into_iter().map(T::from).collect();
            let inv = inverse_batch(&range[..]);
            let mut ans = vec![T::zero(); rank + 2];
            for i in 0..=rank {
                ans[i + 1] = inv[i] * p[i];
            }
            Self::new(ans)
        }

        pub fn ln(self, n: usize) -> Self {
            should_eq!(self.0[0], T::one());
            let diff = self.differential().modular(n);
            let inv = self.inverse(n);
            let prod = (diff * inv).modular(n);
            let ans = prod.integral();
            ans.modular(n)
        }

        pub fn exp(self, n: usize) -> Self {
            if n == 0 {
                Self::zero()
            } else {
                self.modular(n).exp0(n)
            }
        }

        fn exp0(&self, n: usize) -> Self {
            if n == 1 {
                Self::one()
            } else {
                let mut ans = self.exp0((n + 1) / 2);
                let mut ln = ans.clone().ln(n);
                let mut ln = self.modular(n) - ln;
                ln.0[0] = ln.0[0] + T::one();
                (ans * ln).modular(n)
            }
        }
        fn fast_rem(self, rhs: Self, rhs_inv: &Self) -> Self {
            let rank = self.rank();
            let div = self.clone().fast_div(&rhs, rhs_inv);
            let res = self - rhs * div;
            should!(res.rank() < rank);
            res
        }
        fn fast_div(self, rhs: &Self, rhs_inv: &Self) -> Self {
            let mut a = self.0;
            if a.len() < rhs.0.len() {
                return Self::default();
            }
            a.reverse();
            let c_rank = a.len() - rhs.0.len();
            let proper_len = poly_length(c_rank * 2 + 1);
            let a = poly_modular(a, proper_len);
            let c = poly_modular_ref(&rhs_inv.0, c_rank + 1);
            let mut prod = poly_extend(C::convolution(a, c), c_rank + 1);
            prod.reverse();
            Self::new(prod)
        }
        pub fn downgrade_mod(self: Self, mut n: impl Iterator<Item = usize>) -> Self {
            if self.rank() == 0 {
                return Self::zero();
            }
            let mut data = self.0.clone();
            data.reverse();
            let inv = Self::new(data).inverse((self.rank() - 1) * 2 + 1 + 1);
            self.downgrade_mod_internal(n, &inv)
        }

        fn downgrade_mod_internal(&self, mut n: impl Iterator<Item = usize>, inv: &Self) -> Self {
            if let Some(bit) = n.next() {
                let ans = self.downgrade_mod_internal(n, inv);
                should!(ans.rank() < self.rank());
                let mut ans = ans.pow2();
                if bit == 1 {
                    ans = ans.right_shift(1);
                }
                if ans.rank() < self.rank() {
                    ans
                } else {
                    ans.fast_rem(self.clone(), inv)
                }
            } else {
                Self::one()
            }
        }
    }

    impl<T: Field + FromNumber, C: PolyInverse<T>> Poly<T, C> {
        pub fn inverse(self, n: usize) -> Self {
            if n == 0 {
                Self::zero()
            } else {
                Self::new(C::inverse(self.0, n))
            }
        }
    }

    impl<T: Ring + FromNumber, C: Convolution<T>> Poly<T, C> {
        pub fn new(p: Vec<T>) -> Self {
            let mut res = Self(p, PhantomData);
            res.trim();
            res
        }

        pub fn left_shift(&self, n: usize) -> Self {
            let a = self.0[n..].to_vec();
            Self::new(a)
        }

        pub fn is_zero(&self) -> bool {
            return self.0.len() == 1 && self.0[0] == T::zero();
        }

        pub fn right_shift(&self, n: usize) -> Self {
            if self.is_zero() {
                return Self::zero();
            }
            let mut res = vec![T::zero(); n + self.0.len()];
            for (i, e) in self.0.iter().enumerate() {
                res[i + n] = e.clone();
            }
            Self::new(res)
        }

        pub fn pow2(self: Self) -> Self {
            Self::new(C::pow2(self.0))
        }

        pub fn to_vec(self) -> Vec<T> {
            self.0
        }

        pub fn with_constant(v: T) -> Self {
            Self::new(vec![v])
        }

        pub fn zero() -> Self {
            Self::new(vec![T::zero()])
        }

        pub fn one() -> Self {
            Self::new(vec![T::one()])
        }

        pub fn convolution_delta(mut a: Self, mut b: Self) -> Self {
            let a_rank = a.rank();
            a.0.reverse();
            let mut res = C::convolution(a.0, b.0);
            let mut res = poly_extend(res, a_rank + 1);
            res.reverse();
            Self::new(res)
        }

        pub fn apply(&self, x: T) -> T {
            poly_evaluate(&self.0, x)
        }

        pub fn differential(&self) -> Self {
            let p = &self.0;
            let mut ans = vec![T::zero(); self.rank()];
            for i in 1..=ans.len() {
                ans[i - 1] = p[i] * T::from(i);
            }
            Self::new(ans)
        }

        pub fn dot(&self, rhs: &Self) -> Self {
            Self::new(
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .map(|(a, b)| *a * *b)
                    .collect(),
            )
        }

        fn extend(&mut self, n: usize) {
            if n <= self.0.len() {
                return;
            }
            self.0.resize_with(n, T::zero);
        }

        fn trim(&mut self) {
            self.0 = poly_trim(take(&mut self.0));
        }

        pub fn rank(&self) -> usize {
            self.0.len() - 1
        }

        pub fn modular(&self, n: usize) -> Self {
            Poly::new(poly_modular_ref(&self.0, n))
        }

        pub fn iter(&'_ self) -> core::slice::Iter<'_, T> {
            return self.0.iter();
        }

        pub fn batch_mul(mut polys: &mut [Self]) -> Self {
            if polys.len() == 1 {
                return polys[0].clone();
            }
            let mid = polys.len() >> 1;
            let (a, b) = polys.split_at_mut(mid);
            Self::batch_mul(a) * Self::batch_mul(b)
        }
    }

    impl<T: Ring + FromNumber, C: Convolution<T>> IntoIterator for Poly<T, C> {
        type Item = T;

        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(mut self) -> std::vec::IntoIter<T> {
            return self.0.into_iter();
        }
    }

    impl<T: Ring + FromNumber, C: Convolution<T>> Add for Poly<T, C> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let n = self.0.len();
            let m = rhs.0.len();
            let mut res = poly_extend(self.0, max(n, m));
            for (index, x) in rhs.0.iter().enumerate() {
                res[index] = res[index] + *x;
            }
            Self::new(res)
        }
    }

    impl<T: Ring + FromNumber, C: Convolution<T>> Sub for Poly<T, C> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let n = self.0.len();
            let m = rhs.0.len();
            let mut res = poly_extend(self.0, max(n, m));
            for (index, x) in rhs.0.iter().enumerate() {
                res[index] = res[index] - *x;
            }
            Self::new(res)
        }
    }

    impl<T: Ring + FromNumber, C: Convolution<T>> Default for Poly<T, C> {
        fn default() -> Self {
            Self::new(vec![T::zero()])
        }
    }

    impl<T: Ring + FromNumber, C: Convolution<T>> Mul for Poly<T, C> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            let prod = C::convolution(self.0, rhs.0);
            Self::new(prod)
        }
    }

    impl<T: Field + FromNumber, C: PolyInverse<T>> Div for Poly<T, C> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            let mut a = self.0;
            let mut b = rhs.0;
            if a.len() < b.len() {
                return Self::default();
            }
            a.reverse();
            b.reverse();
            let c_rank = a.len() - b.len();
            let proper_len = poly_length(c_rank * 2 + 1);
            let a = poly_modular(a, proper_len);
            let b = poly_modular(b, proper_len);
            let c = C::inverse(b, c_rank + 1);
            let mut prod = poly_extend(C::convolution(a, c), c_rank + 1);
            prod.reverse();
            Self::new(prod)
        }
    }

    impl<T: Field + FromNumber, C: PolyInverse<T>> Rem for Poly<T, C> {
        type Output = Self;

        fn rem(self, rhs: Self) -> Self::Output {
            self.divider_and_remainder(rhs).1
        }
    }

    impl<T: Ring + FromNumber, C: Convolution<T>> Index<usize> for Poly<T, C> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }
}
pub mod num_integer_reverse {
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;

    const CACHE: [u8; 256] = [
        0, 128, 64, 192, 32, 160, 96, 224, 16, 144, 80, 208, 48, 176, 112, 240, 8, 136, 72, 200,
        40, 168, 104, 232, 24, 152, 88, 216, 56, 184, 120, 248, 4, 132, 68, 196, 36, 164, 100, 228,
        20, 148, 84, 212, 52, 180, 116, 244, 12, 140, 76, 204, 44, 172, 108, 236, 28, 156, 92, 220,
        60, 188, 124, 252, 2, 130, 66, 194, 34, 162, 98, 226, 18, 146, 82, 210, 50, 178, 114, 242,
        10, 138, 74, 202, 42, 170, 106, 234, 26, 154, 90, 218, 58, 186, 122, 250, 6, 134, 70, 198,
        38, 166, 102, 230, 22, 150, 86, 214, 54, 182, 118, 246, 14, 142, 78, 206, 46, 174, 110,
        238, 30, 158, 94, 222, 62, 190, 126, 254, 1, 129, 65, 193, 33, 161, 97, 225, 17, 145, 81,
        209, 49, 177, 113, 241, 9, 137, 73, 201, 41, 169, 105, 233, 25, 153, 89, 217, 57, 185, 121,
        249, 5, 133, 69, 197, 37, 165, 101, 229, 21, 149, 85, 213, 53, 181, 117, 245, 13, 141, 77,
        205, 45, 173, 109, 237, 29, 157, 93, 221, 61, 189, 125, 253, 3, 131, 67, 195, 35, 163, 99,
        227, 19, 147, 83, 211, 51, 179, 115, 243, 11, 139, 75, 203, 43, 171, 107, 235, 27, 155, 91,
        219, 59, 187, 123, 251, 7, 135, 71, 199, 39, 167, 103, 231, 23, 151, 87, 215, 55, 183, 119,
        247, 15, 143, 79, 207, 47, 175, 111, 239, 31, 159, 95, 223, 63, 191, 127, 255,
    ];
    fn reverse_internal<T: Integer>(data: T, size: T) -> T {
        if size == FromNumber::from(8) {
            let res: T = <T as FromNumber>::from(CACHE[<usize as FromNumber>::from(data)]);
            res
        } else {
            let half = size >> T::ONE;
            let mask = (T::ONE << half) - T::ONE;
            reverse_internal(data >> half, half) | (reverse_internal(data & mask, half) << half)
        }
    }

    pub trait BitReverse {
        fn reverse(&self) -> Self;
    }
    impl<T: Integer> BitReverse for T {
        fn reverse(&self) -> Self {
            T::from(reverse_internal(
                self.as_unsigned(),
                FromNumber::from(T::BITS),
            ))
        }
    }
}
pub mod poly_ntt {
    use crate::algebraic_structure::Field;
    use crate::collection::swap_element;
    use crate::macros::should_eq;
    use crate::math::dot_mul;
    use crate::math::log2_ceil;
    use crate::math::pow;
    use crate::modint::ModInt;
    use crate::num_integer::Integer;
    use crate::num_integer_reverse::BitReverse;
    use crate::num_number::FromNumber;
    use crate::poly::Convolution;
    use crate::poly::PolyInverse;
    use crate::poly_common::convolution_brute_force;
    use crate::poly_common::poly_extend;
    use crate::poly_common::poly_length;
    use crate::poly_common::poly_trim;
    use std::marker::PhantomData;

    pub fn ntt<I: Integer, T: ModInt<I>>(mut p: Vec<T>, inv: bool) -> Vec<T> {
        let modulus = T::modulus();
        let g = T::primitive_root().unwrap();
        let m = log2_ceil(p.len()) as usize;
        let n = 1 << m;
        let shift = usize::BITS as usize - n.count_trailing_zero() as usize;
        for i in 1..n {
            let j = (i << shift).reverse();
            if i < j {
                swap_element(&mut p, i, j);
            }
        }
        let mut ws = vec![T::zero(); n / 2];
        should_eq!((modulus - I::ONE) % FromNumber::from(n), I::ZERO);
        let unit = pow(g, (modulus - I::ONE) / FromNumber::from(n));
        if ws.len() >= 1 {
            ws[0] = T::one();
        }
        for i in 1..ws.len() {
            ws[i] = ws[i - 1] * unit;
        }

        for d in 0..m {
            let s = 1usize << d;
            let s2 = s << 1;
            let right = n >> (1 + d);
            for i in (0..n).step_by(s2) {
                for j in 0..s {
                    let a = i + j;
                    let b = a + s;
                    let t = ws[j * right] * p[b];
                    p[b] = p[a] - t;
                    p[a] = p[a] + t;
                }
            }
        }

        if inv {
            let inv_n = T::from(n).possible_inv().unwrap();
            let mut i = 0;
            let mut j = 0;
            while i <= j {
                let a = p[j];
                p[j] = p[i] * inv_n;

                if i != j {
                    p[i] = a * inv_n;
                }

                i += 1;
                j = n - i;
            }
        }

        p
    }

    #[derive(Clone, Copy)]
    pub struct ConvolutionNTT<I: Integer, T: ModInt<I>>(PhantomData<(I, T)>);
    impl<I: Integer, T: ModInt<I>> ConvolutionNTT<I, T> {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    impl<I: Integer, T: ModInt<I> + Field> PolyInverse<T> for ConvolutionNTT<I, T> {
        fn inverse_internal(data: &[T]) -> Vec<T> {
            if data.len() == 1 {
                return vec![data[0].possible_inv().unwrap()];
            }
            let m = data.len();
            let prev_len = (m + 1) / 2;
            let ans = Self::inverse_internal(&data[0..prev_len]);
            let n = (prev_len - 1) * 2 + m - 1 + 1;
            let proper_len = poly_length(n);
            let ans = poly_extend(ans, proper_len);
            let prefix: Vec<T> = poly_extend(data.to_owned(), proper_len);
            let prefix = ntt(prefix, false);
            let mut ans = ntt(ans, false);
            for i in 0..proper_len {
                ans[i] = ans[i] * (T::from(2) - prefix[i] * ans[i]);
            }
            let ans = ntt(ans, true);
            poly_extend(ans, m)
        }
    }
    impl<I: Integer, T: ModInt<I>> Convolution<T> for ConvolutionNTT<I, T> {
        fn convolution(a: Vec<T>, b: Vec<T>) -> Vec<T> {
            let mut a = poly_trim(a);
            let mut b = poly_trim(b);
            let len = a.len() + b.len() - 1;
            let proper_len = 1 << log2_ceil(len);
            a.resize_with(proper_len, T::zero);
            b.resize_with(proper_len, T::zero);
            let a = ntt(a, false);
            let b = ntt(b, false);
            let prod = dot_mul(&a, &b);
            poly_trim(ntt(prod, true))
        }

        fn pow2(a: Vec<T>) -> Vec<T> {
            let mut a = poly_trim(a);
            let len = a.len() + a.len() - 1;
            let proper_len = 1 << log2_ceil(len);
            a.resize_with(proper_len, T::zero);
            let a = ntt(a, false);
            let prod = dot_mul(&a, &a);
            poly_trim(ntt(prod, true))
        }
    }
}
pub mod static_modint {
    use crate::algebraic_structure::*;
    use crate::arithmetic::*;
    use crate::macros::should;
    use crate::modint::ModInt;
    use crate::num_gcd::inv_mod;
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;
    use std::fmt;
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::fmt::Error;
    use std::hash::Hash;
    use std::marker::PhantomData;
    use std::num::ParseIntError;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Sub;
    use std::str::FromStr;
    use std::string::ParseError;

    pub trait StaticModulusFactory<T> {
        const M: T;
        const ZERO: T;
        const ONE: T;
        const ROOT: T;
    }

    macro_rules! StaticModulusFactoryImpl {
        ($name: ident, $T: ty, $M: expr, $R: expr) => {
            pub struct $name;
            impl StaticModulusFactory<$T> for $name {
                const M: $T = ($M) as $T;

                const ZERO: $T = <$T>::ZERO;

                const ONE: $T = <$T>::ONE % ($M) as $T;

                const ROOT: $T = ($R) as $T;
            }
        };
    }

    StaticModulusFactoryImpl!(MF469762049, i32, 469762049, 3);
    StaticModulusFactoryImpl!(MF167772161, i32, 167772161, 3);
    StaticModulusFactoryImpl!(MF998244353, i32, 998_244_353, 3);
    StaticModulusFactoryImpl!(MF1000000007, i32, 1_000_000_007, 5);
    StaticModulusFactoryImpl!(MF1000000009, i32, 1_000_000_009, 13);
    StaticModulusFactoryImpl!(MF9223372036737335297, i64, 9223372036737335297, 3);

    pub struct StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        v: T,
        phantom: PhantomData<F>,
    }

    impl<T, F> FromStr for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        type Err = ();
        #[inline(always)]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Ok(x) = T::from_str(s) {
                Ok(FromNumber::from(x))
            } else {
                Result::Err(())
            }
        }
    }

    impl<T, F> Clone for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        #[inline(always)]
        fn clone(&self) -> Self {
            Self {
                v: self.v.clone(),
                phantom: PhantomData,
            }
        }
    }

    impl<T, F> Copy for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
    }

    impl<T, F> PartialEq for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        #[inline(always)]
        fn eq(&self, other: &Self) -> bool {
            self.v == other.v
        }
    }

    impl<T, F> Eq for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
    }

    impl<T: Integer, F: StaticModulusFactory<T>> ModInt<T> for StaticModInt<T, F> {
        #[inline(always)]
        fn modulus() -> T {
            F::M
        }
        #[inline(always)]
        fn primitive_root() -> Option<Self> {
            Some(Self::new(F::ROOT))
        }
        #[inline(always)]
        fn value(&self) -> T {
            self.v
        }
    }

    impl<T, F> FromNumber for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        fn from(num: impl Number) -> Self {
            Self::new(T::modular(FromNumber::from(num), F::M))
        }
    }

    impl<T, F> StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        #[inline(always)]
        pub fn new(v: T) -> Self {
            should!(v >= T::zero() && v < F::M);
            Self {
                v,
                phantom: PhantomData,
            }
        }
    }

    impl<T, F> Display for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self.v, f)
        }
    }
    impl<T, F> Debug for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self.v, f)
        }
    }

    impl<T, F> Div for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self * rhs.possible_inv().unwrap()
        }
    }

    impl<T, F> Mul for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        type Output = Self;
        #[inline(always)]
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(T::mul_mod(self.v, rhs.v, F::M))
        }
    }

    impl<T, F> Sub for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        type Output = Self;
        #[inline(always)]
        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(if self.v >= rhs.v {
                self.v - rhs.v
            } else {
                self.v - rhs.v + F::M
            })
        }
    }

    impl<T, F> Add for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        type Output = Self;
        #[inline(always)]
        fn add(self, rhs: Self) -> Self::Output {
            Self::new({
                let res = self.v + rhs.v;
                if res < self.v || res >= F::M {
                    res - F::M
                } else {
                    res
                }
            })
        }
    }

    impl<T, F> StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        #[inline(always)]
        fn mul_inv(&self) -> Self {
            self.possible_inv().unwrap()
        }
    }

    impl<T, F> CommutativeAdd for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
    }

    impl<T, F> AssociativeAdd for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
    }

    impl<T, F> IdentityAdd for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        #[inline(always)]
        fn zero() -> Self {
            Self::new(F::ZERO)
        }
    }

    impl<T, F> CommutativeMul for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
    }

    impl<T, F> AssociativeMul for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
    }

    impl<T, F> IdentityMul for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        #[inline(always)]
        fn one() -> Self {
            Self::new(F::ONE)
        }
    }
    impl<T, F> MulInv for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        #[inline(always)]
        fn possible_inv(&self) -> Option<Self> {
            inv_mod(self.v, F::M).map(Self::new)
        }
    }
    impl<T, F> Hash for StaticModInt<T, F>
    where
        T: 'static + Integer,
        F: StaticModulusFactory<T>,
    {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.v.hash(state);
        }
    }
    pub(crate) use StaticModulusFactoryImpl;
}
pub mod linear_recurrence {
    use crate::algebraic_structure::Field;
    use crate::macros::debug;
    use crate::macros::debug_discard;
    use crate::macros::should;
    use crate::macros::should_eq;
    use crate::num_number::FromNumber;
    use crate::poly::Poly;
    use crate::poly::PolyInverse;

    pub fn kth_term_of_linear_recurrence<
        T: Field + FromNumber,
        C: PolyInverse<T>,
        I: Iterator<Item = usize>,
    >(
        mut lr: Vec<T>,
        prefix: &Vec<T>,
        k: I,
    ) -> T {
        should!(lr.len() - 1 <= prefix.len());
        should_eq!(lr[0], T::one());
        lr.reverse();
        let modulus = Poly::<T, C>::new(lr);
        let ans = modulus.downgrade_mod(k);
        ans.iter()
            .zip(prefix.iter())
            .map(|(a, b)| *a * *b)
            .reduce(|a, b| a + b)
            .unwrap()
    }
}
pub mod solver {
    use crate::algebraic_structure::*;
    use crate::arithmetic::*;
    use crate::fast_input::FastInput;
    use crate::linear_recurrence::kth_term_of_linear_recurrence;
    use crate::macros::input;
    use crate::num_integer::Integer;
    use crate::num_number::FromNumber;
    use crate::poly::Poly;
    use crate::poly_ntt::ConvolutionNTT;
    use crate::static_modint::StaticModInt;
    use crate::static_modint::MF998244353;
    use std::io::BufRead;
    use std::io::Write;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Sub;
    use std::panic;

    type mi = StaticModInt<i32, MF998244353>;
    type conv = ConvolutionNTT<i32, mi>;
    pub unsafe fn solve_one<I: BufRead>(
        test_id: usize,
        fi: &mut FastInput<I>,
        fo: &mut impl Write,
    ) {
        input! {
            fi,
            d: usize,
            k: u64,
        }
        let a: Vec<mi> = (0..d).map(|_| fi.r()).collect();
        let mut c: Vec<mi> = (0..d).map(|_| fi.r()).collect();
        c.push(FromNumber::from(-1));
        let mut c = c.iter().rev().map(|x| mi::zero() - *x).collect();
        let kth = kth_term_of_linear_recurrence::<_, conv, _>(
            c,
            &a,
            (0..60).map(|i| k.kth_bit(i) as usize),
        );
        writeln!(fo, "{}", kth);
    }

    pub unsafe fn solve_multi<I: BufRead>(fi: &mut FastInput<I>, fo: &mut impl Write) {
        let t: usize = 1;
        for test_id in 1..=t {
            solve_one(test_id, fi, fo);
        }
    }
}
pub mod rand {
    use crate::macros::should;
    use crate::num_number::FromNumber;
    use crate::num_number::Number;

    struct xorshift128p_state(u64, u64);

    #[cfg(feature = "local-build")]
    fn seed() -> u64 {
        dbg!("use fix seed 0");
        0
    }

    #[cfg(not(feature = "local-build"))]
    fn seed() -> u64 {
        let start = std::time::SystemTime::now();
        start
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    fn splitmix64(state: &mut u64) -> u64 {
        *state += 0x9E3779B97f4A7C15;
        let mut result = *state;
        result = (result ^ (result >> 30)) * 0xBF58476D1CE4E5B9;
        result = (result ^ (result >> 27)) * 0x94D049BB133111EB;
        return result ^ (result >> 31);
    }

    fn xorshift128p_init(seed: u64) -> xorshift128p_state {
        let mut smstate = seed;
        let mut result = xorshift128p_state(0, 0);

        result.0 = splitmix64(&mut smstate);
        result.1 = splitmix64(&mut smstate);

        result
    }

    fn xorshift128p(state: &mut xorshift128p_state) -> u64 {
        let mut t = state.0;
        let s = state.1;
        state.0 = s;
        t ^= t << 32;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);
        state.1 = t;
        t + s
    }

    pub fn rng() -> &'static mut Rng {
        static mut singleton: Option<Rng> = None;
        unsafe {
            match &mut singleton {
                Some(x) => x,
                None => {
                    singleton = Some(Rng::new());
                    rng()
                }
            }
        }
    }

    pub struct Rng {
        state: xorshift128p_state,
    }

    impl Rng {
        pub fn new() -> Self {
            Self {
                state: xorshift128p_init(seed()),
            }
        }
        pub fn init(&mut self, seed: u64) {
            self.state = xorshift128p_init(seed);
        }
        pub fn new_with_seed(seed: u64) -> Self {
            Self {
                state: xorshift128p_init(seed),
            }
        }
        #[inline]
        pub fn u64(&mut self) -> u64 {
            xorshift128p(&mut self.state)
        }
        #[inline]
        pub fn usize(&mut self) -> usize {
            self.u64() as usize
        }
        #[inline]
        pub fn f64(&mut self) -> f64 {
            loop {
                let res = self.u64() as f64 / usize::MAX as f64;
                if res < 1f64 {
                    return res;
                }
            }
        }
        #[inline]
        pub fn limit_usize(&mut self, n: usize) -> usize {
            self.limit_u64(n as u64) as usize
        }
        #[inline]
        pub fn limit_u64(&mut self, n: u64) -> u64 {
            loop {
                let res = (self.f64() * n as f64) as u64;
                if res < n {
                    return res;
                }
            }
        }
        #[inline]
        pub fn range_u64(&mut self, l: u64, r: u64) -> u64 {
            self.limit_u64(r - l + 1) + l
        }
        #[inline]
        pub fn range_usize(&mut self, l: usize, r: usize) -> usize {
            self.range_u64(l as u64, r as u64) as usize
        }
    }

    pub fn random<T: Number>(n: T) -> T {
        should!(n > T::ZERO);
        loop {
            let res = FromNumber::from(rng().f64() * n.as_f64());
            if res < n {
                return res;
            }
        }
    }
}
pub mod stress_external_member {
    use crate::fast_input::FastInput;
    use crate::rand::Rng;
    use std::io::BufRead;
    use std::io::Write;
    use std::mem::swap;

    pub unsafe fn brute_force<InT>(fi: &mut FastInput<InT>, fo: &mut impl Write)
    where
        InT: BufRead,
    {
    }

    pub fn generate_test(rng: &mut Rng, fo: &mut impl Write) {}

    pub fn against(a: &String, b: &String) -> bool {
        let da: Vec<_> = a.split_ascii_whitespace().collect();
        let db: Vec<_> = b.split_ascii_whitespace().collect();
        da == db
    }
}
pub mod stress {
    use crate::fast_input::FastInput;
    use crate::rand::rng;
    use crate::rand::Rng;
    use crate::solver::solve_multi;
    use crate::stress_external_member::against;
    use crate::stress_external_member::brute_force;
    use crate::stress_external_member::generate_test;
    use std::fmt::Display;
    use std::io::BufReader;
    use std::io::Cursor;
    use std::io::Write;

    fn printable_input(s: String) -> String {
        let len = 1000;
        if s.len() < len {
            s
        } else {
            s[0..len].to_string() + "..."
        }
    }

    pub unsafe fn stress() {
        let mut round = 0;
        let mut rng_for_input_generator = Rng::new_with_seed(0);
        loop {
            round += 1;
            if round >= 10000 {
                println!("Pass stress!");
                return;
            }
            rng().init(0);

            let mut input = OutputWrapper::new();
            generate_test(&mut rng_for_input_generator, &mut input);
            println!("Test {}:", round);
            println!("{}\n", printable_input(input.to_string()));
            let mut actual_output = OutputWrapper::new();
            let mut expect_output = OutputWrapper::new();
            solve_multi(
                &mut FastInput::new(BufReader::new(Cursor::new(input.to_string()))),
                &mut actual_output,
            );
            brute_force(
                &mut FastInput::new(BufReader::new(Cursor::new(input.to_string()))),
                &mut expect_output,
            );
            if !against(&actual_output.to_string(), &expect_output.to_string()) {
                println!("Test case {}:", round);
                println!("\nInput:\n{}", input);
                println!("\nExpect:\n{}", expect_output);
                println!("\nActual:\n{}", actual_output);
                panic!("Fail!");
            }
        }
    }
    struct OutputWrapper(Vec<u8>);
    impl OutputWrapper {
        pub fn new() -> OutputWrapper {
            OutputWrapper(Vec::new())
        }
    }
    impl Display for OutputWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = String::from_utf8(self.0.clone()).unwrap();
            f.write_str(s.as_str())
        }
    }
    impl Write for OutputWrapper {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut to = buf.iter().map(|x| *x).collect();
            self.0.append(&mut to);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
use crate::fast_input::FastInput;
use crate::solver::solve_multi;
use crate::stress::stress;
use std::io::BufWriter;
use std::thread;

unsafe fn run_in_current_thread() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut fi = FastInput::new(stdin.lock());
    let mut fo = BufWriter::new(stdout.lock());
    solve_multi(&mut fi, &mut fo);
}

unsafe fn run_in_new_thread() {
    thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(|| {
            run_in_current_thread();
        })
        .unwrap()
        .join();
}
#[cfg(not(feature = "stress"))]
fn main() {
    unsafe {
        run_in_current_thread();
    }
}

#[cfg(feature = "stress")]
fn main() {
    unsafe {
        stress();
    }
}
