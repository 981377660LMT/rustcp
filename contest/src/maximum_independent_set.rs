use crate::{num_integer::Integer, num_number::Number};

pub fn maximum_independent_set<W: Number>(
    edges: &Vec<Vec<bool>>,
    weights: &Vec<W>,
) -> (W, Vec<bool>) {
    let n = weights.len();
    if n > 60 {
        panic!("Too large set");
    }
    let mut adj = vec![0u64; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if edges[i][j] || edges[j][i] {
                adj[i] = adj[i] | 1 << j;
            }
        }
    }

    let left_half = (n + 1) / 2;
    let right_half = n - left_half;

    let mut subsets = vec![0u32; 1 << left_half];
    let mut left_sum = vec![W::ZERO; 1 << left_half];
    let mut log = 0;
    for i in 1..1 << left_half {
        while 1 << log + 1 <= i {
            log += 1;
        }
        let highest = 1 << log;
        left_sum[i] = left_sum[i - highest] + weights[log];
        subsets[i] = subsets[i - highest];
        let possible = subsets[((i - highest) as u64 & (!adj[log])) as usize];
        if left_sum[subsets[i] as usize] < left_sum[possible as usize] + weights[log] {
            subsets[i] = possible | highest as u32;
        }
    }

    let mask: u64 = (1 << left_half) - 1;
    let mut right_sum = vec![W::ZERO; 1 << right_half];
    let mut nearby = vec![0u64; 1 << right_half];

    let mut solution = subsets[mask as usize] as u64;
    let mut ans = left_sum[subsets[mask as usize] as usize];
    let min = W::MIN;
    log = 0;
    for i in 1..1 << right_half {
        while 1 << log + 1 <= i {
            log += 1;
        }
        let highest = 1 << log;
        right_sum[i] = if nearby[i - highest].kth_bit(left_half + log) == 1 {
            min
        } else {
            right_sum[i - highest] + weights[left_half + log]
        };
        nearby[i] = nearby[i - highest] | adj[left_half + log];

        let leftSubset = subsets[(mask & (!nearby[i])) as usize];
        let cand = right_sum[i] + left_sum[leftSubset as usize];
        if cand > ans {
            ans = cand;
            solution = ((i as u64) << (left_half as u64)) | leftSubset as u64;
        }
    }

    let mut selections = vec![false; n];
    for i in 0..n {
        selections[i] = solution.kth_bit(i) == 1;
    }
    (ans, selections)
}
