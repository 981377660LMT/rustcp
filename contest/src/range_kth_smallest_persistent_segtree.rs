use std::{cmp::Ordering, fmt::Debug};

use crate::{
    binary_search::{first_true, last_true},
    persistent_segtree::PersistentSegtree,
};

///
/// as name
///
/// # Example
///
/// ```
/// use template::range_kth_smallest_persistent_segtree::*;
/// let rks = RangeKthSmallest::new(&vec![2, 1, 4, 3], i32::cmp);
/// assert_eq!(1, rks.kth_range(0, 3, 1));
/// assert_eq!(2, rks.kth_range(0, 3, 2));
/// assert_eq!(3, rks.kth_range(1, 3, 2));
/// assert_eq!(2, rks.rect(0, 3, &2, &3));
/// assert_eq!(1, rks.rect(1, 3, &0, &2));
/// ```
///
pub struct RangeKthSmallest<T: PartialOrd + Copy + Debug> {
    st: PersistentSegtree<usize>,
    sorted: Vec<T>,
    cmp: Box<dyn Fn(&T, &T) -> Ordering>,
    roots: Vec<usize>,
}
impl<T: PartialOrd + Copy + Debug> Debug for RangeKthSmallest<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RangeKthSmallest")
            .field("st", &self.st)
            .field("sorted", &self.sorted)
            .field("roots", &self.roots)
            .finish()
    }
}

impl<T: PartialOrd + Copy + Debug> RangeKthSmallest<T> {
    pub fn new(data: &[T], cmp: impl Fn(&T, &T) -> Ordering + 'static) -> Self {
        let mut sorted: Vec<T> = data.iter().map(|x| *x).collect();
        sorted.sort_by(|a, b| cmp(a, b));
        sorted.dedup_by(|a, b| cmp(a, b) == Ordering::Equal);
        let mut st = PersistentSegtree::new(0, sorted.len() - 1, 0usize);
        let mut roots = Vec::with_capacity(data.len());
        roots.push(st.latest_version());
        for x in data.iter() {
            let index = sorted.binary_search_by(|a| cmp(a, x)).unwrap();
            roots.push(st.update(st.latest_version(), index, 1));
        }
        Self {
            st,
            roots,
            sorted,
            cmp: Box::new(cmp),
        }
    }

    pub fn kth_range(&self, l: usize, r: usize, k: usize) -> T {
        let res = self.st.first_true_delta(
            self.roots[l],
            self.roots[r + 1],
            0,
            self.sorted.len() - 1,
            |i, a, b| b - a >= k,
        );
        self.sorted[res.0]
    }

    pub fn rect(&self, l: usize, r: usize, b: &T, t: &T) -> usize {
        let from = first_true(0, self.sorted.len() - 1, |&x| {
            (self.cmp)(&self.sorted[x], b) != Ordering::Less
        });
        let end = last_true(0, self.sorted.len() - 1, |&x| {
            (self.cmp)(&self.sorted[x], t) != Ordering::Greater
        });
        if let (Some(from), Some(end)) = (from, end) {
            self.st.query(self.roots[r + 1], from, end) - self.st.query(self.roots[l], from, end)
        } else {
            0
        }
    }
}
