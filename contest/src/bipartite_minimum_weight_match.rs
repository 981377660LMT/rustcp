use crate::{
    cost_flow::add_cost_flow_edge_di,
    cost_flow_augment_dijkstra::CostFlowAugmentDijkstra,
    graph::{DiEdge, WeightEdge},
    max_flow::FlowEdge,
    num_number::{Number, FromNumber}, num_concrete::Concrete,
};

pub fn bipartite_minimum_weight_match<W: Concrete, E: WeightEdge<W>>(
    g: &Vec<Vec<E>>,
    rsize: usize,
    inf: W,
) -> (usize, W, Vec<Option<usize>>) {
    let n = g.len();
    let src = n + rsize;
    let dst = src + 1;
    let mut net = vec![Vec::new(); dst + 1];
    for a in 0..n {
        for e in g[a].iter() {
            add_cost_flow_edge_di(&mut net, a, n + e.to(), W::ONE, e.weight());
        }
    }
    for a in 0..n {
        add_cost_flow_edge_di(&mut net, src, a, W::ONE, W::ZERO);
    }
    for b in n..n + rsize {
        add_cost_flow_edge_di(&mut net, b, dst, W::ONE, W::ZERO);
    }
    let mut mcmf = CostFlowAugmentDijkstra::new(&mut net, src, dst, inf);
    let (flow, cost) = mcmf.try_push(<W as FromNumber>::from(n));
    let mut mate = vec![None; n];
    for a in 0..n {
        for e in net[a].iter() {
            if e.0.is_real() && e.0.flow() == W::ONE {
                let b = e.0.to() - n;
                mate[a] = Some(b);
                break;
            }
        }
    }

    (flow.as_usize(), cost, mate)
}
