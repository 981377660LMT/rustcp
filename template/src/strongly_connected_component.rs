use std::{collections::VecDeque, cmp::min};

use crate::graph::{DiEdge, BiEdge};

struct Row<'a, E> {
    dq: VecDeque<usize>,
    dfn: Vec<usize>,
    low: Vec<usize>,
    set: Vec<usize>,
    instk: Vec<bool>,
    g: &'a Vec<Vec<E>>,
    time: usize,
}

pub fn strongly_connected_component_di<E: DiEdge>(g: &Vec<Vec<E>>) -> Vec<usize> {
    let n = g.len();
    let mut row = Row {
        dq: VecDeque::with_capacity(n),
        dfn: vec![usize::MAX; n],
        low: vec![usize::MAX; n],
        set: vec![usize::MAX; n],
        instk: vec![false; n],
        g,
        time: 0,
    };

    for i in 0..n {
        if row.dfn[i] == usize::MAX {
            strongly_connected_component_di_internal(i, &mut row);
        }
    }

    row.set
}

fn strongly_connected_component_di_internal<'a, E: DiEdge>(root: usize, row: &mut Row<'a, E>) {
    if row.dfn[root] != usize::MAX {
        return;
    }
    row.time += 1;
    row.dfn[root] = row.time;
    row.low[root] = row.time;
    row.instk[root] = true;
    row.dq.push_back(root);

    for e in row.g[root].iter() {
        strongly_connected_component_di_internal(e.to(), row);
        if row.instk[e.to()] {
            row.low[root] = min(row.low[root], row.low[e.to()]);
        }
    }

    if row.dfn[root] == row.low[root] {
        loop {
            let tail = row.dq.pop_back().unwrap();
            row.set[tail] = root;
            row.instk[tail] = false;
            if tail == root {
                break;
            }
        }
    }
}
pub fn strongly_connected_component_bi<E: DiEdge>(g: &Vec<Vec<BiEdge<E>>>) -> Vec<usize> {
    let n = g.len();
    let mut row = Row {
        dq: VecDeque::with_capacity(n),
        dfn: vec![usize::MAX; n],
        low: vec![usize::MAX; n],
        set: vec![usize::MAX; n],
        instk: vec![false; n],
        g,
        time: 0,
    };

    for i in 0..n {
        if row.dfn[i] == usize::MAX {
            strongly_connected_component_bi_internal(i, usize::MAX, usize::MAX, &mut row);
        }
    }

    row.set
}


fn strongly_connected_component_bi_internal<'a, E: DiEdge>(root: usize, fa: usize, rev: usize, row: &mut Row<'a, BiEdge<E>>) {
    if row.dfn[root] != usize::MAX {
        return;
    }
    row.time += 1;
    row.dfn[root] = row.time;
    row.low[root] = row.time;
    row.instk[root] = true;
    row.dq.push_back(root);

    for (index, e) in row.g[root].iter().enumerate() {
        if e.0.to() == fa && e.rev() == rev {
            continue;
        }
        strongly_connected_component_bi_internal(e.0.to(), root, index, row);
        if row.instk[e.0.to()] {
            row.low[root] = min(row.low[root], row.low[e.0.to()]);
        }
    }

    if row.dfn[root] == row.low[root] {
        loop {
            let tail = row.dq.pop_back().unwrap();
            row.set[tail] = root;
            row.instk[tail] = false;
            if tail == root {
                break;
            }
        }
    }
}