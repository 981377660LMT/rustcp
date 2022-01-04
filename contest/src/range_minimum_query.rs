use crate::{
    binary::{highest_k_one},
    math::{log2_floor},
    num_integer::Integer,
    sparse_table::SparseTable,
    macros::should, addition_wrapper::MinAdd,
};
use std::{cmp::min, fmt::Debug};

const SHIFT: usize = 5;
const BLOCK_SIZE: usize = 1 << SHIFT;
const AND_MASK: usize = BLOCK_SIZE - 1;

///
/// - new: O(n)
/// - query: O(1)
///
/// # Reference
///
/// [https://codeforces.com/blog/entry/92310](https://codeforces.com/blog/entry/92310)
///
/// # Example
///
/// ```
/// use template::range_minimum_query::*;
/// let rmq = RangeMinimumQuery::new(vec![1, 5, 2, 4, 3]);
/// assert_eq!(rmq.query(0, 2), 1);
/// assert_eq!(rmq.query(1, 2), 2);
/// assert_eq!(rmq.query(1, 3), 2);
/// assert_eq!(rmq.query(1, 4), 2);
/// assert_eq!(rmq.query(3, 4), 3);
/// ```
///
#[derive(Debug)]
pub struct RangeMinimumQuery<T: Copy + Debug + Ord>
{
    data: Vec<T>,
    to_left: Vec<usize>,
    st: SparseTable<MinAdd<T>>,
}

impl<T: Copy + Debug + Ord> RangeMinimumQuery<T>
{
    pub fn new(data: Vec<T>) -> Self {
        let n = data.len();
        if n == 0 {
            return Self {
                data,
                to_left: Vec::new(),
                st: SparseTable::new(&Vec::new()),
            };
        }
        let consider_part = ((n - 1) >> SHIFT) + 1;
        let mut min_elements = Vec::with_capacity(consider_part);
        for i in 0..n {
            let to = i >> SHIFT;
            let w = MinAdd(data[i]);
            if min_elements.len() <= to {
                min_elements.push(w);
            } else {
                min_elements[to] = min_elements[to] + w;
            }
        }
        let mut to_left = Vec::with_capacity(n);
        let st = SparseTable::new(&min_elements[..]);
        let mut mask = 0usize;
        for i in 0..n {
            if (i & AND_MASK) == 0 {
                mask = 0;
            }
            let b = i >> SHIFT;
            while mask != 0 {
                let head = log2_floor(mask) as usize;
                if data[i] <= data[(b << SHIFT) | head] {
                    mask = mask & !(1usize << head);
                } else {
                    break;
                }
            }
            mask = mask | (1usize << (i & AND_MASK));
            to_left.push(mask);
        }

        Self { data, st, to_left }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        should!(l <= r);
        let bl = l >> SHIFT;
        let br = r >> SHIFT;
        let to = highest_k_one(32u32 - (l & AND_MASK) as u32) as usize;
        let bs = bl << SHIFT;
        if bl == br {
            let index = (self.to_left[r] & to).count_trailing_zero() as usize | bs;
            self.data[index]
        } else {
            let idx_1 =
                (self.to_left[(bl << SHIFT) | AND_MASK] & to).count_trailing_zero() as usize | bs;
            let idx_2 = self.to_left[r].count_trailing_zero() as usize | (br << SHIFT);
            let mut best = min(self.data[idx_1], self.data[idx_2]);
            if bl + 1 < br {
                best = min(best, self.st.query(bl + 1, br - 1).0);
            }
            best
        }
    }
}
