use crate::collection::swap_element;

///
/// find next permutation (strict increasing order)
/// 
/// - O(e) average for s contains only distinct values
/// - O(n) worse case without restraints on s
/// 
pub fn next_permutation<T: Ord>(s: &mut [T]) -> bool {
    let n = s.len();
    for i in (0..n - 1).into_iter().rev() {
        if s[i] < s[i + 1] {
            s[i + 1..n].reverse();
            let mut j = i;
            loop {
                j += 1;
                if s[j] > s[i] {
                    swap_element(s, i, j);
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn previous_permutation<T: Ord>(s: &mut [T]) -> bool {
    let n = s.len();
    for i in (0..n-1).into_iter().rev() {
        if s[i] > s[i + 1] {
            let mut j = n;
            loop {
                j -= 1;
                if s[j] < s[i] {
                    swap_element(s, i, j);
                }
            }
            s[i + 1..n].reverse();
            return true;
        }
    }
    return false;
}