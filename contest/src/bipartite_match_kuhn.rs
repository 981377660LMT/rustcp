use crate::{
    graph::DiEdge,
    shuffle::{random_permutation, shuffle},
};

struct Row<'a, E> {
    g: &'a Vec<Vec<E>>,
    mate: Vec<Option<usize>>,
    visit: Vec<usize>,
    version: usize,
}

pub fn bipartite_match_kuhn<E: DiEdge>(g: &Vec<Vec<E>>) -> (Vec<Option<usize>>, usize) {
    let n = g.len();
    let mut row = Row {
        g,
        mate: vec![None; n],
        visit: vec![0; n],
        version: 0,
    };
    let mut cnt = 0;
    let mut perm: Vec<usize> = (0..n).collect();
    loop {
        row.version += 1;
        let mut plus = 0;
        shuffle(&mut perm[..]);
        for &i in perm.iter() {
            if row.mate[i].is_some() {
                continue;
            }
            if dfs(i, &mut row) {
                plus += 1;
            }
        }
        cnt += plus;
        if plus == 0 {
            break;
        }
    }
    (row.mate, cnt)
}

fn dfs<'a, E: DiEdge>(root: usize, row: &mut Row<'a, E>) -> bool {
    if row.visit[root] == row.version {
        return false;
    }
    row.visit[root] = row.version;
    for i in 0..row.g[root].len() {
        let e = &row.g[root][i];
        if row.mate[e.to()].is_none() || dfs(row.mate[e.to()].unwrap(), row) {
            row.mate[e.to()] = Some(root);
            row.mate[root] = Some(e.to());
            return true;
        }
    }
    false
}
