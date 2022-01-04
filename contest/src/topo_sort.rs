use crate::graph::DiEdge;

struct Row<'a, E> {
    g: &'a Vec<Vec<E>>,
    visited: Vec<bool>,
    data: Vec<usize>,
}

pub fn topo_dfs<E: DiEdge>(g: &Vec<Vec<E>>) -> Vec<usize> {
    let mut row = Row {
        g,
        visited: vec![false; g.len()],
        data: Vec::with_capacity(g.len()),
    };
    for i in 0..g.len() {
        topo_dfs_internal(i, &mut row);
    }
    row.data
}

fn topo_dfs_internal<'a, E: DiEdge>(root: usize, row: &mut Row<'a, E>) {
    if row.visited[root] {
        return;
    }
    row.visited[root] = true;
    for e in row.g[root].iter() {
        topo_dfs_internal(e.to(), row);
    }
    row.data.push(root);
}