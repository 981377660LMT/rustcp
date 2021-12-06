use std::fmt::{self, Display};

use crate::{math::log2_floor, util::{should_eq, should}};

///
/// sparse table
/// 
/// O(n\log_2n) preprocess time and space complexity
/// 
/// # Example
/// 
/// ```
/// use contest::sparse_table::*;
/// fn merge<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
///     if (x < y) {
///         y    
///     } else {
///         x
///     }
/// }
/// let data = [3, 1, 4, 2];
/// let st = SparseTable::new(&data[..], merge);
/// 
/// assert_eq!(1, *st.query(1, 1));
/// assert_eq!(3, *st.query(0, 1));
/// assert_eq!(4, *st.query(1, 3));
/// ```
/// 
#[derive(Debug)]
pub struct SparseTable<'a, T> {
    ///
    /// data[i][j] cover [j, j+2^i)
    /// 
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

        Self {
            data,
            f,
        }
    }

    pub fn query(&self, l: usize, r: usize) -> &'a T {
        should!(l <= r);
        let log = log2_floor(r - l + 1) as usize;
        (self.f)(self.data[log][l], self.data[log][r + 1 - (1usize << log)])
    }
}