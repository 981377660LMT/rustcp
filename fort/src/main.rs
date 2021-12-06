pub mod fast_input {
    use std::io;

    pub struct FastInput<R: std::io::BufRead> {
        inner: R,
        line: Vec<u8>,
        offset: usize,
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
            while self.offset < self.line.len() && self.line[self.offset] <= 32 {
                self.offset += 1;
            }
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

        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            loop {
                match self.next() {
                    Some(token) => {
                        return token.parse().ok().expect("Wrong format input");
                    }
                    None => {
                        self.line.clear();
                        self.inner.read_until(b'\n', &mut self.line).unwrap();
                        self.offset = 0;
                    }
                }
            }
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
pub mod util {
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
            $e
        };
    }

    pub(crate) use debug;
    pub(crate) use should;
    pub(crate) use should_eq;
}
pub mod num_number {
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Sub;

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
    {
        const MAX: Self;
        const MIN: Self;
        const ZERO: Self;
        const ONE: Self;

        type SignedType;

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
        ($t: ty) => {
            Generator!($t, $t);
        };
        ($t: ty, $s: ty) => {
            impl Number for $t {
                type SignedType = $s;

                const MAX: Self = <$t>::MAX;
                const MIN: Self = <$t>::MIN;
                const ZERO: Self = 0 as Self;
                const ONE: Self = 1 as Self;

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
                    } else {
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

    Generator!(usize);
    Generator!(isize);

    Generator!(i8);
    Generator!(i16);
    Generator!(i32);
    Generator!(i64);
    Generator!(i128);

    Generator!(u8, i8);
    Generator!(u16, i16);
    Generator!(u32, i32);
    Generator!(u64, i64);
    Generator!(u128, i128);

    Generator!(f32);
    Generator!(f64);
}
pub mod num_integer {
    use crate::num_number::Number;
    use std::ops::BitAnd;
    use std::ops::BitOr;
    use std::ops::BitXor;
    use std::ops::Not;
    use std::ops::Rem;
    use std::ops::Shl;
    use std::ops::Shr;

    pub trait Integer:
        Number
        + Rem<Output = Self>
        + Shl<Output = Self>
        + Shr<Output = Self>
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + Not<Output = Self>
    {
        type HighPrecisionType;
        type UnsignedType;
        const BITS: i32;

        fn div_floor(mut a: Self, mut b: Self) -> Self {
            let mut r = a / b;
            if r * b > a {
                if b > Self::ZERO {
                    r = r - Self::ONE;
                } else {
                    r = r + Self::ONE;
                }
            }
            r
        }
        fn div_ceil(mut a: Self, mut b: Self) -> Self {
            let mut r = a / b;
            if r * b < a {
                if b > Self::ZERO {
                    r = r + Self::ONE;
                } else {
                    r = r - Self::ONE;
                }
            }
            r
        }
        fn bit_count(&self) -> i32;
        fn higest_set_bit_offset(&self) -> i32;
        fn lowest_set_bit(&self) -> Self;
        fn higest_one_bit(&self) -> Self;
        fn count_leading_zero(&self) -> i32;
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

    macro_rules! Generator {
        ($t: ty, $h: ty, $u: ty) => {
            impl Integer for $t {
                type HighPrecisionType = $h;
                type UnsignedType = $u;
                const BITS: i32 = <$t>::BITS as i32;

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

    Generator!(i8, i16, u8);
    Generator!(u8, i16, u8);
    Generator!(i16, i32, u16);
    Generator!(u16, u32, u16);
    Generator!(i32, i64, u32);
    Generator!(u32, u64, u32);
    Generator!(isize, isize, usize);
    Generator!(usize, usize, usize);
    Generator!(i64, i128, u64);
    Generator!(u64, u128, u64);
    Generator!(i128, i128, u128);
    Generator!(u128, u128, u128);
}
pub mod num_real {
    use crate::num_number::Number;

    pub trait Real: Number {
        fn average(a: Self, b: Self) -> Self {
            (a + b) / Self::from_i8(2)
        }
    }
}
pub mod algebraic_structure {
    use crate::num_integer::Integer;
    use crate::num_number::Number;
    use crate::num_real::Real;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Sub;

    pub trait Magma: Add<Output = Self> + Copy + PartialEq {}
    pub trait Semigroup: Magma {}

    pub trait Monoid: Semigroup {
        fn add_identity() -> Self;
    }

    pub trait Group: Monoid + Sub<Output = Self> {
        fn add_inv(&self) -> Self;
    }

    pub trait AbelianGroup: Group {}

    pub trait Ring: AbelianGroup + Mul<Output = Self> {
        fn mul_identity() -> Self;
    }
    pub trait CommutativeRing: Ring {}
    pub trait IntegralDomain: CommutativeRing {}
    pub trait Field: IntegralDomain + Div<Output = Self> {
        fn mul_inv(&self) -> Self;
    }

    impl<T> CommutativeRing for T where T: Number {}
    impl<T> Ring for T
    where
        T: Number,
    {
        fn mul_identity() -> Self {
            T::ONE
        }
    }
    impl<T> AbelianGroup for T where T: Number {}
    impl<T> Group for T
    where
        T: Number,
    {
        fn add_inv(&self) -> Self {
            self.negative()
        }
    }
    impl<T> Monoid for T
    where
        T: Number,
    {
        fn add_identity() -> Self {
            Self::ZERO
        }
    }
    impl<T> Semigroup for T where T: Number {}
    impl<T> Magma for T where T: Number {}

    impl<T> Field for T
    where
        T: Real,
    {
        fn mul_inv(&self) -> Self {
            Self::ONE / *self
        }
    }
    impl<T> IntegralDomain for T where T: Real {}
}
pub mod math {
    use crate::algebraic_structure::Ring;
    use crate::num_integer::Integer;

    pub fn pow<T, E>(x: T, n: E) -> T
    where
        T: Ring,
        E: Integer,
    {
        if n == E::ZERO {
            return T::mul_identity();
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
        if res < 0 || (T::ONE << T::from_i32(res)) < x {
            res + 1
        } else {
            res
        }
    }
}
pub mod sparse_table {
    use crate::math::log2_floor;
    use crate::util::should;
    use crate::util::should_eq;
    use std::fmt;
    use std::fmt::Display;

    #[derive(Debug)]
    pub struct SparseTable<'a, T> {
        data: Vec<Vec<&'a T>>,
        f: fn(&'a T, &'a T) -> &'a T,
    }

    impl<'a, T> SparseTable<'a, T> {
        pub fn new(s: &'a [T], f: fn(&'a T, &'a T) -> &'a T) -> Self {
            let n = s.len();
            if n == 0 {
                return Self {
                    data: Vec::new(),
                    f,
                };
            }
            let level = (log2_floor(n) + 1) as usize;
            let mut data: Vec<Vec<&'a T>> = vec![vec![&s[0]; n]; level];
            for i in 0..n {
                data[0][i] = &s[i];
            }

            for i in 1..level {
                let step = 1usize << (i - 1);
                for j in 0..n {
                    let k = j + step;
                    if k < n {
                        data[i][j] = f(data[i - 1][j], data[i - 1][k]);
                    } else {
                        data[i][j] = data[i - 1][j];
                    }
                }
            }

            Self { data, f }
        }

        pub fn query(&self, l: usize, r: usize) -> &'a T {
            should!(l <= r);
            let log = log2_floor(r - l + 1) as usize;
            (self.f)(self.data[log][l], self.data[log][r + 1 - (1usize << log)])
        }
    }
}
pub mod solver {
    use crate::fast_input::FastInput;
    use crate::sparse_table::SparseTable;
    use crate::util::debug;
    use std::cmp::min;
    use std::io::BufWriter;
    use std::io::StdinLock;
    use std::io::StdoutLock;
    use std::io::Write;
    use std::panic;

    pub unsafe fn solve_one(
        test_id: u32,
        fi: &mut FastInput<StdinLock>,
        fo: &mut BufWriter<StdoutLock>,
    ) {
        let n = fi.ru();
        let q = fi.ru();
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(fi.ri());
        }
        let st = debug!(SparseTable::new(&v[..], min));
        for i in 0..q {
            let l = fi.ru();
            let r = fi.ru() - 1;
            let (l, r) = debug!((l, r));
            let ans = st.query(l, r);
            writeln!(fo, "{}", ans);
        }
    }

    pub unsafe fn solve_multi(fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
        let t: u32 = 1;
        for test_id in 1..t + 1 {
            solve_one(test_id, fi, fo);
        }
    }
}
use crate::fast_input::FastInput;
use crate::solver::solve_multi;
use std::io::BufWriter;

fn main() {
    unsafe {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut fi = FastInput::new(stdin.lock());
        let mut fo = BufWriter::new(stdout.lock());
        solve_multi(&mut fi, &mut fo);
    }
}
