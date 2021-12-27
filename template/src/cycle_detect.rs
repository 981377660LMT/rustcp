use std::collections::VecDeque;

use crate::graph::{BiEdge, DiEdge};
struct Row<'a, E> {
    g: &'a Vec<Vec<E>>,
    dq: VecDeque<usize>,
    instk: Vec<bool>,
    visit: Vec<bool>,
}

pub fn circle_detect_di<'a, E: DiEdge>(g: &'a Vec<Vec<E>>) -> Option<Vec<usize>> {
    let n = g.len();
    let mut row = Row {
        g,
        dq: VecDeque::with_capacity(n),
        instk: vec![false; n],
        visit: vec![false; n],
    };
    for i in 0..n {
        if row.visit[i] {
            continue;
        }
        if circle_detect_di_internal(i, &mut row) {
            return Some(Vec::from(row.dq));
        }
    }
    None
}

pub fn circle_detect_bi<'a, E: DiEdge>(g: &'a Vec<Vec<BiEdge<E>>>) -> Option<Vec<usize>> {
    let n = g.len();
    let mut row = Row {
        g,
        dq: VecDeque::with_capacity(n),
        instk: Vec::new(),
        visit: vec![false; n],
    };
    for i in 0..n {
        if row.visit[i] {
            continue;
        }
        if circle_detect_bi_internal(i, usize::MAX, usize::MAX, &mut row) {
            return Some(Vec::from(row.dq));
        }
    }
    None
}

fn circle_detect_bi_internal<'a, E: DiEdge>(
    root: usize,
    fa: usize,
    rev_index: usize,
    row: &mut Row<'a, BiEdge<E>>,
) -> bool {
    if row.visit[root] {
        while let Some(x) = row.dq.pop_front() {
            if x == root {
                break;
            }
        }
        
        row.dq.push_front(root);
        return true;
    }
    row.visit[root] = true;
    row.dq.push_back(root);

    for (i, e) in row.g[root].iter().enumerate() {
        if e.0.to() == fa && e.rev() == rev_index {
            continue;
        }
        if circle_detect_bi_internal(e.0.to(), root, i, row) {
            return true;
        }
    }

    row.dq.pop_back();
    return false;
}

fn circle_detect_di_internal<'a, E: DiEdge>(root: usize, row: &mut Row<'a, E>) -> bool {
    if row.visit[root] {
        if row.instk[root] {
            while let Some(x) = row.dq.pop_front() {
                if x == root {
                    break;
                }
            }
            
            row.dq.push_front(root);
            return true;
        }
        return false;
    }
    row.visit[root] = true;
    row.instk[root] = true;
    row.dq.push_back(root);

    for e in row.g[root].iter() {
        if circle_detect_di_internal(e.to(), row) {
            return true;
        }
    }

    row.instk[root] = false;
    row.dq.pop_back();
    return false;
}
