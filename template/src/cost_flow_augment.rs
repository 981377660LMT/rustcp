use std::collections::VecDeque;

use crate::{num_number::Number, cost_flow::CostFlowEdge, graph::{BiEdge, rev_edge, DiEdge}};

pub trait CostFlowAugment<C: Number> {
    fn set_augment_callback(&mut self, callback: impl FnMut(C, C) -> bool + 'static);
}

pub fn cost_flow_shortest_path_spfa<C: Number, E: CostFlowEdge<C>>(g: &Vec<Vec<BiEdge<E>>>, src: &[usize], inf: C)
-> Option<(Vec<C>, Vec<Option<usize>>)> {
    let n = g.len();
    let mut dq = VecDeque::with_capacity(n);
    let mut dist = vec![inf; n];
    let mut prev = vec![None; n];
    let mut inq = vec![false; n];
    for &s in src {
        dist[s] = C::ZERO;
        inq[s] = true;
        dq.push_back(s);
    }
    let mut round = 0;
    let threshold = n as u64 * n as u64;
    while let Some(root) = dq.pop_front() {
        round += 1;
        if round > threshold {
            return None;
        }
        for e in g[root].iter() {
            let to = e.to();
            if rev_edge(g, e).0.flow() == C::ZERO || dist[to] <= dist[root] + e.0.cost() {
                continue;
            }
            dist[to] = dist[root] + e.0.cost();
            prev[to] = Some(root); 
            if !inq[to] {
                inq[to] = true;
                dq.push_back(to);
            }
        }
    }

    Some((dist, prev))
}