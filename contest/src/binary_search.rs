use crate::{num_integer::Integer, num_real::Real};

pub fn first_true<T>(mut l: T, mut r: T, f: &Box<dyn Fn(T) -> bool>) -> T
    where T: Integer
{
    while l < r {
        let m = T::average_floor(l, r);
        if (*f)(m) {
            r = m;
        }else{
            l = m + T::from_i8(1);
        }
    }
    l
}

pub fn last_true<T>(mut l: T, mut r: T, f: &Box<dyn Fn(T) -> bool>) -> T
    where T: Integer
{
    while l < r {
        let m = T::average_floor(l, r);
        if (*f)(m) {
            l = m;
        }else{
            r = m - T::from_i8(1);
        }
    }
    l
}

pub fn first_true_float<T>(mut round: u8, mut l: T, mut r: T, f: &Box<dyn Fn(T) -> bool>) -> T 
    where T: Real {
    while round > 0 {
        round -= 1;
        let m = (l + r) / T::from_i8(2);
        if (*f)(m) {
            r = m;
        } else {
            l = m;
        }
    }
    l
}

pub fn last_true_float<T>(mut round: u8, mut l: T, mut r: T, f: &Box<dyn Fn(T) -> bool>) -> T 
    where T: Real {
    while round > 0 {
        round -= 1;
        let m = (l + r) / T::from_i8(2);
        if (*f)(m) {
            l = m;
        } else {
            r = m;
        }
    }
    l
}