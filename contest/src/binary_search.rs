use crate::{num_integer::Integer, num_real::Real, num_number::FromNumber};

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

pub fn first_true_float<T>(mut round: u8, mut l: T, mut r: T, f: impl Fn(&T) -> bool) -> Option<T>
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
