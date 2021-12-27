use template::permutation::next_permutation;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::new();

        loop {
            res.push(nums.clone());
            if !next_permutation(&mut nums) {
                break;
            }
        }

        res
    }
}

struct Solution;
fn main() {
}
