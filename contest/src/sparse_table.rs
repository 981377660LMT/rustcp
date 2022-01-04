use std::{
    fmt::{self, Debug},
    ops::Add,
};

use crate::{math::log2_floor, macros::should};

///
/// sparse table
///
/// O(n\log_2n) preprocess time and space complexity
///
/// # Example
///
/// ```ignore
///     
/// let data = vec![3, 1, 4, 2];
/// let st = SparseTable::new(&data, |a, b| if a < b {a} else {b});
///
/// assert_eq!(1, st.query(1usize, 1usize));
/// assert_eq!(3, st.query(0usize, 1usize));
/// assert_eq!(4, st.query(1usize, 3usize));
/// ```
///
pub struct SparseTable<T: Copy + Debug + Add<T, Output = T>> {
    ///
    /// data[i][j] cover [j, j+2^i)
    ///
    data: Vec<Vec<T>>,
}

impl<T: Copy + Debug + Add<T, Output = T>> SparseTable<T> {
    pub fn new(s: &[T]) -> Self {
        let n = s.len();
        if n == 0 {
            return Self { data: Vec::new() };
        }
        let level = (log2_floor(n) + 1) as usize;
        let mut data: Vec<Vec<T>> = vec![vec![s[0]; n]; level];
        for i in 0..n {
            data[0][i] = s[i];
        }

        for i in 1..level {
            let step = 1usize << (i - 1);
            for j in 0..n {
                let k = j + step;
                if k < n {
                    data[i][j] = data[i - 1][j] + data[i - 1][k];
                } else {
                    data[i][j] = data[i - 1][j];
                }
            }
        }

        Self { data }
    }

    ///
    /// O(1) find the sum over data[l..r]
    ///
    pub fn query(&self, l: usize, r: usize) -> T {
        should!(l <= r);
        let log = log2_floor(r - l + 1) as usize;
        self.data[log][l] + self.data[log][r + 1 - (1usize << log)]
    }
}

impl<T: Copy + Debug + Add<T, Output = T>> Debug for SparseTable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SparseTable")
            .field("data", &self.data)
            .finish()
    }
}
