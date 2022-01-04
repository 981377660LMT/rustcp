use std::collections::VecDeque;

pub fn cartesian_tree<T: PartialOrd>(seq: &[T]) -> (Vec<(Option<usize>, Option<usize>)>, usize){
    let n  = seq.len();
    let mut nodes: Vec<(Option<usize>, Option<usize>)> = vec![(None, None); n];
    let mut dq = VecDeque::with_capacity(n);
    for i in 0..n {
        while !dq.is_empty() {
            let back: usize = *dq.back().unwrap();
            if seq[back] > seq[i] {
                dq.pop_back();
                nodes[back].1 = nodes[i].0;
                nodes[i].0 = Some(back);
            } else {
                break;
            }
        }
        dq.push_back(i);
    }
    while dq.len() > 1 {
        let tail = dq.pop_back().unwrap();
        nodes[*dq.back().unwrap()].1 = Some(tail);
    }

    let root = dq.back().unwrap();

    (nodes, *root)
}