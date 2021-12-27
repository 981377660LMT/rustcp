use crate::{max_flow::{add_flow_edge_di, FlowEdge}, max_flow_dinic::dinic, graph::DiEdge};

pub fn bipartite_match_dinic<E: DiEdge>(adj: &Vec<Vec<E>>, rsize: usize) -> (Vec<Option<usize>>, usize) {
    let n = adj.len();
    let src = n + rsize;
    let dst = src + 1;
    let mut g = vec![Vec::new(); dst + 1];
    for (a, list) in adj.iter().enumerate() {
        for b in list {
            add_flow_edge_di(&mut g, a, b.to() + n, 1);
        }
    }
    for i in 0..n {
        add_flow_edge_di(&mut g, src, i, 1);
    }
    for i in 0..rsize {
        add_flow_edge_di(&mut g, i + n, dst, 1);
    }
    let flow = dinic(&mut g, src, dst, n as i32);
    let mut mate = vec![None; n];
    for a in 0..n {
        for e in g[a].iter() {
            if e.0.is_real() && e.0.flow() == 1 {
                let b = e.0.to() - n;
                mate[a] = Some(b);
                break;
            }
        }
    }

    (mate, flow as usize)
}
