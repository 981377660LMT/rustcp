use std::cmp::min;

use crate::{num_number::Number, num_concrete::Concrete};

struct Row<'a, T: Concrete> {
    inf: T,
    table: &'a Vec<Vec<T>>,
    left_label: Vec<T>,
    right_label: Vec<T>,
    left_mate: Vec<usize>,
    right_mate: Vec<usize>,
    n: usize,
}

pub fn bipartite_maximum_weight_match_km<T: Concrete>(
    table: &Vec<Vec<T>>,
    inf: T,
) -> (T, Vec<usize>, Vec<usize>, Vec<T>, Vec<T>) {
    let n = table.len();
    let mut row = Row {
        inf,
        table,
        left_label: vec![inf.negative(); n],
        right_label: vec![T::ZERO; n],
        left_mate: vec![usize::MAX; n],
        right_mate: vec![usize::MAX; n],
        n,
    };
    for x in 0..n {
        bfs(&mut row, x);
    }
    let mut sum = T::ZERO;
    for x in 0..n {
        sum += table[x][row.left_mate[x]];
    }
    (
        sum,
        row.left_mate,
        row.right_mate,
        row.left_label,
        row.right_label,
    )
}

fn bfs<'a, T: Concrete>(row: &mut Row<'a, T>, start_x: usize) {
    let n = row.n;
    let mut find = false;
    let mut end_y = usize::MAX;
    let mut y_pre = vec![usize::MAX; n];
    let mut S = vec![false; n];
    let mut T = vec![false; n];
    let mut slack_y = vec![row.inf; n];
    let mut queue = vec![0usize; n];
    let mut qs = 0;
    let mut qe = 0;
    queue[qe] = start_x;
    qe += 1;
    while !find {
        while qs < qe && !find {
            let mut x = queue[qs];
            qs += 1;
            S[x] = true;
            for y in 0..n {
                if T[y] {
                    continue;
                }
                let mut tmp = row.left_label[x] + row.right_label[y] - row.table[x][y];
                if tmp == T::ZERO {
                    T[y] = true;
                    y_pre[y] = x;
                    if row.right_mate[y] == usize::MAX {
                        end_y = y;
                        find = true;
                        break;
                    } else {
                        queue[qe] = row.right_mate[y];
                        qe += 1;
                    }
                } else if slack_y[y] > tmp {
                    slack_y[y] = tmp;
                    y_pre[y] = x;
                }
            }
        }

        if find {
            break;
        }

        let mut a = row.inf;
        for y in 0..n {
            if !T[y] {
                a = min(a, slack_y[y]);
            }
        }
        for i in 0..n {
            if S[i] {
                row.left_label[i] -= a;
            }
            if T[i] {
                row.right_label[i] += a;
            }
        }
        qs = 0;
        qe = 0;
        for y in 0..n {
            if !T[y] && slack_y[y] == a {
                T[y] = true;
                if row.right_mate[y] == usize::MAX {
                    end_y = y;
                    find = true;
                    break;
                } else {
                    queue[qe] = row.right_mate[y];
                    qe += 1;
                }
            }
            slack_y[y] -= a;
        }
    }

    while end_y != usize::MAX {
        let mut pre_x = y_pre[end_y];
        let mut pre_y = row.left_mate[pre_x];
        row.left_mate[pre_x] = end_y;
        row.right_mate[end_y] = pre_x;
        end_y = pre_y;
    }
}
