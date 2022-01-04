use crate::{graph::{DiEdge, WeightEdge}, num_number::Number};

pub fn tree_depth<E: DiEdge>(g: &Vec<Vec<E>>, roots: &[usize]) -> Vec<usize> {
    let mut depth = vec![usize::MAX; g.len()]; 
    for &root in roots {
        dfs(g, &mut depth, root, usize::MAX);
    }
    depth
}

fn dfs<E: DiEdge>(g: &Vec<Vec<E>>, depth: &mut Vec<usize>, root: usize, fa: usize) {
    depth[root] = if fa == usize::MAX {
        0
    } else {
        depth[fa] + 1
    };
    for e in g[root].iter() {
        if e.to() == fa {
            continue;
        }
        dfs(g, depth, e.to(), root);
    }
}

pub fn tree_depth_weight<W: Number, E: WeightEdge<W>>(g: &Vec<Vec<E>>, roots: &[usize]) -> Vec<W> {
    let mut depth = vec![W::ZERO; g.len()]; 
    for &root in roots {
        dfs_weight(g, &mut depth, root, usize::MAX, W::ZERO);
    }
    depth
}

fn dfs_weight<W: Number, E: WeightEdge<W>>(g: &Vec<Vec<E>>, depth: &mut Vec<W>, root: usize, fa: usize, d: W) {
    depth[root] = d;
    for e in g[root].iter() {
        if e.to() == fa {
            continue;
        }
        dfs_weight(g, depth, e.to(), root, d + e.weight());
    }
}