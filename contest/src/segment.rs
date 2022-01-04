macro_rules! have_intersection {
    ($l: expr, $r: expr, $L: expr, $R: expr) => {
        (($l) <= ($R) && ($r) >= ($L))
    };
}
use std::collections::HashMap;

pub (crate) use have_intersection;

macro_rules! cover {
    ($l: expr, $r: expr, $L: expr, $R: expr) => {
        (($l) <= ($L) && ($R) <= ($r))
    };
}
pub (crate) use cover;

pub fn estimate_tree_size_with_cache(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if n <= 1 {
        return n;
    }
    match cache.get(&n) {
        Some(k) => *k,
        None => {
            let res = 1 + estimate_tree_size_with_cache(n >> 1, cache) + estimate_tree_size_with_cache((n + 1) >> 1, cache);
            cache.insert(n, res);
            res
        }
    }
}
///
/// log_2(n) 
/// 
pub fn estimate_tree_size(n: usize) -> usize {
    let mut map = HashMap::new();
    estimate_tree_size_with_cache(n, &mut map)
}