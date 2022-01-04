use crate::{graph::{DiEdge, WeightEdge}, tree_depth::{tree_depth, tree_depth_weight}, math::{argmax, argmax_by}, num_number::Number, num_concrete::Concrete};

pub fn tree_diameter<E: DiEdge>(g: &Vec<Vec<E>>, root: usize) -> (usize, usize, usize) {
    let depth = tree_depth(g, &[root]);
    let end_0 = argmax(&depth[..]).unwrap().0;
    let depth = tree_depth(g, &[end_0]);
    let end_1 = argmax(&depth[..]).unwrap().0;
    (depth[end_1], end_0, end_1)
}


pub fn tree_diameter_weight<W: Concrete, E: WeightEdge<W>>(g: &Vec<Vec<E>>, root: usize) -> (W, usize, usize) {
    let depth = tree_depth_weight(g, &[root]);
    let end_0 = argmax(&depth[..]).unwrap().0;
    let depth = tree_depth_weight(g, &[end_0]);
    let end_1 = argmax(&depth[..]).unwrap().0;
    (depth[end_1], end_0, end_1)
}

