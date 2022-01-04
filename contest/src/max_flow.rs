use std::collections::VecDeque;

use crate::{graph::{BiEdge, DiEdge, add_bi_edge}, num_number::Number};

pub trait FlowEdge<C>: DiEdge
where
    C: Number,
{
    fn flow(&self) -> C;
    fn send(&mut self, flow: C);
    fn is_real(&self) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct SimpleFlowEdge<C: Number>(usize, C, bool);
impl<C:Number> DiEdge for SimpleFlowEdge<C> {
    fn to(&self) -> usize {
        self.0
    }
}

impl<C:Number> FlowEdge<C> for SimpleFlowEdge<C> {
    fn flow(&self) -> C {
        self.1
    }

    fn send(&mut self, flow: C) {
        self.1 = self.1 + flow;
    }

    fn is_real(&self) -> bool {
        self.2
    }
}

pub fn add_flow_edge_di<C: Number>(g: &mut Vec<Vec<BiEdge<SimpleFlowEdge<C>>>>, a: usize, b: usize, cap_ab: C) {
    add_bi_edge(g, SimpleFlowEdge(b, C::ZERO, true), SimpleFlowEdge(a, cap_ab, false));
}

pub fn add_flow_edge_bi<C: Number>(g: &mut Vec<Vec<BiEdge<SimpleFlowEdge<C>>>>, a: usize, b: usize, cap_ab: C, cap_ba: C) {
    add_bi_edge(g, SimpleFlowEdge(b, cap_ba, true), SimpleFlowEdge(a, cap_ab, true));
}

pub fn shortest_path_for_flow<C: Number, E: FlowEdge<C>> (g: &Vec<Vec<BiEdge<E>>>, src: &[usize]) -> Vec<usize> {
    let n = g.len();
    let mut dist = vec![usize::MAX; n];
    let mut dq = VecDeque::with_capacity(n);
    for &root in src {
        dist[root] = 0;
        dq.push_back(root);
    }
    while let Some(root) = dq.pop_front() {
        for e in g[root].iter() {
            if e.0.flow() == C::ZERO {
                continue;
            }
            let to = e.to();
            if dist[to] != usize::MAX {
                continue;
            }
            dist[to] = dist[root] + 1;
            dq.push_back(to);
        }
    }

    dist
}


pub fn send_flow<C: Number, E: FlowEdge<C>>(g: &mut Vec<Vec<BiEdge<E>>>, root: usize, edge: usize, flow: C) {
    let e = &mut g[root][edge];
    let to = e.to();
    let rev = e.rev();
    e.0.send(flow);
    g[to][rev].0.send(flow.negative());
}
