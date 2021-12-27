use crate::{graph::DiEdge, shuffle::{shuffle, random_permutation}};

struct Row<'a, E> {
    g: &'a Vec<Vec<E>>,
    mate: Vec<Option<usize>>,
    visit: Vec<usize>,
    version: usize,
}

pub fn bipartite_match_kuhn_random<E: DiEdge>(g: &Vec<Vec<E>>, perm: &[usize]) -> (Vec<Option<usize>>, usize) {
    let n = g.len();
    let mut row = Row{
        g,
        mate: vec![None; n],
        visit: vec![0; n],
        version: 0,
    };
    let mut cnt = 0;
    for &i in perm {
        if row.mate[i].is_some() {
            continue;
        }
        row.version += 1;
        if dfs(i, &mut row) {
            cnt += 1;
        }
    } 
    (row.mate, cnt)
}

fn dfs<'a, E: DiEdge>(root: usize, row: &mut Row<'a, E>) -> bool {
    if row.visit[root] == row.version {
        return false;
    }
    row.visit[root] = row.version;
    let mut indices: Vec<usize> = random_permutation(row.g[root].len());
    for i in indices {
        let e = &row.g[root][i];
        if row.mate[e.to()].is_none() || dfs(row.mate[e.to()].unwrap(), row) {
            row.mate[e.to()] = Some(root);
            row.mate[root] = Some(e.to());
            return true;
        }
    }
    false
}