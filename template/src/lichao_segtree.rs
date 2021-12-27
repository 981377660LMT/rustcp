use std::{cmp::Ordering, mem::swap};

use crate::{
    arithmetic::PreferDiv,
    binary_search::{first_true, last_true},
    math::unsafe_max,
    num_number::Number,
    segment::{cover, estimate_tree_size, have_intersection},
    template_macro::should,
};

#[derive(Debug, Clone, Copy)]
pub struct Line<T: Number + PreferDiv>(T, T);
impl<T: Number + PreferDiv> Line<T> {
    pub fn apply(&self, x: T) -> T {
        self.0 * x + self.1
    }
}

pub struct Row<T: Number + PreferDiv> {
    line: Line<T>,
    right: usize,
}

pub struct LichaoSegtree<T: Number + PreferDiv>(Vec<Row<T>>, Vec<T>, T);

///
/// LichaoSegtree implementation
///
/// - insert: O((\log_2n)^2)
/// - overall insert: O(\log_2n)
/// - query: O(\log_2n)
///
/// # Example
///
/// ```rust
/// use template::lichao_segtree::*;
/// let mut lichao = LichaoSegtree::<i32>::new(0, vec![1, 3, 0, 2]);
/// lichao.insert(0, 3, 1, 0);
/// lichao.insert(0, 2, -1, 2);
/// lichao.insert(1, 3, 2, -2);
///
/// assert_eq!(Some(2), lichao.query(0));
/// assert_eq!(Some(1), lichao.query(1));
/// assert_eq!(Some(2), lichao.query(2));
/// assert_eq!(Some(4), lichao.query(3));
/// assert_eq!(None, lichao.query(-1));
/// ```
///
impl<T: Number + PreferDiv> LichaoSegtree<T> {
    pub fn new(min_element: T, mut xs: Vec<T>) -> Self {
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        xs.dedup();
        let m = xs.len();
        let size = estimate_tree_size(m);
        let mut res = Self(Vec::with_capacity(size), xs, min_element);
        res.init(0, m - 1);
        res
    }

    fn init(&mut self, l: usize, r: usize) -> usize {
        let root = self.0.len();
        self.0.push(Row {
            line: Line(T::ZERO, self.2),
            right: usize::MAX,
        });
        if l == r {
        } else {
            let m = (l + r) >> 1;
            self.init(l, m);
            self.0[root].right = self.init(m + 1, r);
        }
        root
    }

    fn intersect(a: &Line<T>, b: &Line<T>) -> T {
        ///
        ///     a.0 x + a.1 = b.0 x + b.1
        /// =>  x = (b.1 - a.1) / (a.0 - b.0)
        ///
        should!(a.0 < b.0);
        T::div_floor(a.1 - b.1, b.0 - a.0)
    }

    pub fn insert(&mut self, L: T, R: T, a: T, b: T) {
        let L = first_true(0, self.1.len() - 1, |&x| self.1[x] >= L);
        let R = last_true(0, self.1.len() - 1, |&x| self.1[x] <= R);
        if let (Some(L), Some(R)) = (L, R) {
            self.insert_internal(0, L, R, 0, self.1.len() - 1, Line(a, b));
        }
    }

    fn insert_internal(
        &mut self,
        root: usize,
        L: usize,
        R: usize,
        l: usize,
        r: usize,
        mut line: Line<T>,
    ) {
        if !have_intersection!(L, R, l, r) {
            return;
        }
        let m = (l + r) >> 1;
        if cover!(L, R, l, r) {
            //cool
            if self.0[root].line.0 == line.0 {
                self.0[root].line.1 = unsafe_max(self.0[root].line.1, line.1);
                return;
            }
            let mut small_line = self.0[root].line;
            if small_line.0 > line.0 {
                swap(&mut small_line, &mut line);
            }
            let intersect_at = Self::intersect(&small_line, &line);
            if intersect_at <= self.1[m] {
                self.0[root].line = line;
                if l < r {
                    self.insert_internal(root + 1, L, R, l, m, small_line);
                }
            } else {
                self.0[root].line = small_line;
                if l < r {
                    self.insert_internal(self.0[root].right, L, R, m + 1, r, line);
                }
            }
            return;
        }
        self.insert_internal(root + 1, L, R, l, m, line);
        self.insert_internal(self.0[root].right, L, R, m + 1, r, line);
    }

    pub fn query(&self, x: T) -> Option<T> {
        let pt = first_true(0, self.1.len() - 1, |&pt| self.1[pt] >= x);
        if let Some(pt) = pt {
            if self.1[pt] == x {
                return Some(self.query_internal(0, pt, 0, self.1.len() - 1));
            }
        }
        None
    }

    fn query_internal(&self, root: usize, x: usize, l: usize, r: usize) -> T {
        let best = self.0[root].line.apply(self.1[x]);
        if l == r {
            return best;
        }
        let m = (l + r) >> 1;
        if x <= m {
            unsafe_max(best, self.query_internal(root + 1, x, l, m))
        } else {
            unsafe_max(best, self.query_internal(self.0[root].right, x, m + 1, r))
        }
    }
}
