use crate::graph::DiEdge;

pub fn tree_father<E: DiEdge>(g: &Vec<Vec<E>>, roots: &[usize]) -> Vec<Option<usize>> {
    let mut father = vec![None; g.len()];
    for &root in roots {
        dfs(g, &mut father, root, usize::MAX);
    }
    father
}

fn dfs<E: DiEdge>(g: &Vec<Vec<E>>, father: &mut [Option<usize>], root: usize, fa: usize) {
    for e in g[root].iter() {
        if e.to() == fa {
            continue;
        }
        father[e.to()] = Some(root);
        dfs(g, father, e.to(), root);
    }
}