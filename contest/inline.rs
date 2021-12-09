pub mod fast_input{
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
pub mod util{
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
    ($($e: expr),*) => {
    }
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
    ($($e: expr),*) => {
    }
}



#[cfg(feature = "local-build")]
macro_rules! debug {
    ($e: expr) => {
        dbg!($e)
    }
}

#[cfg(not(feature = "local-build"))]
macro_rules! debug {
    ($e: expr) => {
        $e
    }
}



pub (crate)use debug;
pub (crate)use should;
pub (crate)use should_eq;

}
pub mod num_number{
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::ops::Mul;
use std::fmt::Display;
use std::fmt::Debug;



pub trait Number: Copy + Add<Output = Self> + Sub<Output = Self> + 
Mul<Output = Self> + Div<Output = Self> + PartialEq + PartialOrd + 
Display + Debug
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
pub mod num_real{
use crate::num_number::Number;


pub trait Real: Number {
    fn average(a: Self, b: Self) -> Self {
        (a + b) / Self::from_i8(2)
    }
}
}
pub mod arithmetic{
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use crate::num_number::Number;
use crate::num_real::Real;




pub trait CommutativeAdd: Add<Output = Self> + Copy + Debug {}
pub trait AssociativeAdd: Add<Output = Self> + Copy + Debug {}
pub trait IdentityAdd: Add<Output = Self> + Copy + Debug {
    const ZERO: Self;
}
pub trait CommutativeMul: Mul<Output = Self> + Copy + Debug {}
pub trait AssociativeMul: Mul<Output = Self> + Copy + Debug {}
pub trait IdentityMul: Mul<Output = Self> + Copy + Debug {
    const ONE: Self;
}
pub trait IdempotentAdd: CommutativeAdd + AssociativeAdd {}
pub trait IdempotentMul: CommutativeMul + AssociativeMul {}
pub trait IntegralMul: Mul<Output = Self> + Copy + Debug {}
macro_rules! AddGenerator {
    ($t: ty, $zero: expr) => {
        impl CommutativeAdd for $t {}
        impl IdentityAdd for $t {
            const ZERO: Self = $zero;
        }
        impl IdempotentAdd for $t {}
        impl AssociativeAdd for $t {}
    };
}

macro_rules! MulGenerator {
    ($t: ty, $one: expr) => {
        impl CommutativeMul for $t {}
        impl IdentityMul for $t {
            const ONE: Self = $one;
        }
        impl IdempotentMul for $t {}
        impl AssociativeMul for $t {}
        impl IntegralMul for $t {}
    };
}

macro_rules! AllGenerator {
    ($t: ty, $zero: expr, $one: expr) => {
        AddGenerator!($t, $zero);
        MulGenerator!($t, $one);
    };
}


impl<T> CommutativeAdd for T where T: Number {}
impl<T> IdentityAdd for T
where
    T: Number,
{
    const ZERO: Self = <T as Number>::ZERO;
}
impl<T> AssociativeAdd for T where T: Number {}
impl<T> CommutativeMul for T where T: Number {}
impl<T> IdentityMul for T
where
    T: Number,
{
    const ONE: Self = <T as Number>::ONE;
}
impl<T> AssociativeMul for T where T: Number {}
impl<T> IntegralMul for T where T: Real {}

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
AllGenerator!(Nil, Nil, Nil);
pub(crate) use AddGenerator;
pub(crate) use MulGenerator;
pub(crate) use AllGenerator;

}
pub mod algebraic_structure{
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use crate::arithmetic::*;



pub trait Magma: Add<Output = Self> + Copy + PartialEq {}
impl<T> Magma for T 
where T: Add<Output = Self> + Copy + PartialEq {}
pub trait Semigroup: Magma + AssociativeAdd {}
impl<T> Semigroup for T 
where T: Magma + AssociativeAdd{}
pub trait Monoid: Semigroup + IdentityAdd {}
impl<T> Monoid for T 
where T: Semigroup + IdentityAdd{}
pub trait Group: Monoid + Sub<Output = Self> {}
impl<T> Group for T 
where T: Monoid + Sub<Output = Self>{}
pub trait AbelianGroup: Group + CommutativeAdd {}
impl<T> AbelianGroup for T 
where T: Group + CommutativeAdd {}
pub trait Ring: AbelianGroup + Mul<Output = Self> + IdentityMul {}
impl<T> Ring for T 
where T: AbelianGroup + Mul<Output = Self> + IdentityMul {}
pub trait CommutativeRing: Ring + CommutativeMul {}
impl<T> CommutativeRing for T 
where T: Ring + CommutativeMul {}
pub trait IntegralDomain: CommutativeRing + IntegralMul {}
impl<T> IntegralDomain for T 
where T: CommutativeRing + IntegralMul {}
pub trait Field: IntegralDomain + Div<Output = Self> {}
impl<T> Field for T 
where T: IntegralDomain + Div<Output = Self> {}
}
pub mod num_integer{
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::Shr;
use crate::num_number::Number;




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

    fn div_floor(a: Self, b: Self) -> Self {
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
    fn div_ceil(a: Self, b: Self) -> Self {
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



macro_rules! Generator {
    ($t: ty, $h: ty, $u: ty) => {
        impl Integer for $t {
            type HighPrecisionType = $h;
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
pub mod fenwick_tree{
use crate::algebraic_structure::Monoid;
use crate::algebraic_structure::Group;
use crate::arithmetic::IdentityAdd;
use crate::num_integer::Integer;
use std::fmt::Debug;
use std::cmp::min;



#[derive(Debug, Clone)]
pub struct FenwickTree<T>
where T: Debug + Monoid {
    data: Vec<T>
}

impl<T> FenwickTree<T> 
where T: Debug + Monoid {
    pub fn new(n: usize) -> Self {
        FenwickTree{
            data: vec![<T as IdentityAdd>::ZERO; n + 1]
        }
    }
                pub fn with_initial_value(data: &[T]) -> Self {
        let n = data.len();
        let mut res = Self::new(n);
        for i in 0..n {
            res.data[i + 1] = data[i];
        }
        for i in 1..(n + 1) {
            let to = i + i.lowest_set_bit();
            if to <= n {
                res.data[to] = res.data[to] + res.data[i];
            }
        }
        res
    }

    pub fn clear(&mut self) {
        self.data.fill(<T as IdentityAdd>::ZERO);
    }

                pub fn update(&mut self, i: usize, u: T) {
        let mut i = i + 1;
        if i <= 0 {
            return;
        }
        while i < self.data.len() {
            self.data[i] = self.data[i] + u;
            i = i + i.lowest_set_bit();
        }
    }

                pub fn query(&self, i: usize) -> T {
        let mut i = min(i + 1, self.data.len() - 1);
        let mut res = <T as IdentityAdd>::ZERO;
        while i > 0 {
            res = res + self.data[i];
            i = i - i.lowest_set_bit();
        }
        res
    }
}

impl<T> FenwickTree<T>
where T: Debug + Group {
    pub fn query_range(&self, l: usize, r: usize) -> T {
        if l > r {
            return <T as IdentityAdd>::ZERO;
        }
        self.query(r) - self.query(l - 1)
     }
}
}
pub mod solver{
use std::io::StdinLock;
use std::io::BufWriter;
use std::io::StdoutLock;
use std::io::Write;
use std::panic;
use std::cmp::min;
use std::ops::Add;
use crate::fast_input::FastInput;
use crate::util::debug;
use crate::arithmetic::*;
use crate::fenwick_tree::FenwickTree;



pub unsafe fn solve_one(test_id: u32, fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
    let n = fi.ru();
    let q = fi.ru();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(fi.rl());
    }
    let mut ft = FenwickTree::with_initial_value(&a);
    
    for _ in 0..q {
               let t = fi.ri();
        if t == 0 {
            let p = fi.ru();
            let x = fi.rl();
            ft.update(p, x);
        } else {
            let l = fi.ru();
            let r = fi.ru() - 1;
            let sum = ft.query_range(l, r);
            writeln!(fo, "{}", sum);
        }
    }
}
  
pub unsafe fn solve_multi(fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
    let t: u32 = 1;    for test_id in 1 .. t + 1 {
        solve_one(test_id, fi, fo);
    }
}
}
use std::io::BufWriter;
use std::thread;
use crate::fast_input::FastInput;
use crate::solver::solve_multi;



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
    .spawn(|| {run_in_current_thread();})
    .unwrap()
    .join();
}

fn main() {
    unsafe {
        run_in_current_thread();
    }
}
