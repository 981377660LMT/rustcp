use std::{
    cmp::{Reverse, min},
    collections::{BinaryHeap},
};

use crate::{
    cost_flow::CostFlowEdge,
    cost_flow_augment::{cost_flow_shortest_path_spfa, CostFlowAugment},
    graph::{rev_edge, BiEdge},
    max_flow::send_flow,
    num_number::Number,
    macros::should, num_concrete::Concrete,
};

pub struct CostFlowAugmentDijkstra<'a, C: Concrete, E: CostFlowEdge<C>> {
    last_dist: Vec<C>,
    cur_dist: Vec<C>,
    prev: Vec<Option<(usize, usize)>>,
    callback: Box<dyn FnMut(C, C) -> bool>,
    g: &'a mut Vec<Vec<BiEdge<E>>>,
    inf: C,
    source: usize,
    sink: usize,
    use_priority_queue: bool,
}

impl<'a, C: Concrete, E: CostFlowEdge<C>> CostFlowAugmentDijkstra<'a, C, E> {
    pub fn new(g: &'a mut Vec<Vec<BiEdge<E>>>, source: usize, sink: usize, inf: C) -> Self {
        let last_dist = cost_flow_shortest_path_spfa(g, &[source], inf).unwrap();
        let n = g.len();
        let m: usize = g.iter().map(|x| x.len()).sum();
        Self {
            last_dist: last_dist.0,
            cur_dist: vec![inf; n],
            prev: vec![None; n],
            callback: Box::new(|_, _| true),
            g,
            inf,
            source,
            sink,
            use_priority_queue: m as u64 * 100 + n as u64 <= n as u64 * n as u64,
        }
    }

    pub fn fix_dist(&mut self) {
        for i in 0..self.g.len() {
            self.last_dist[i] = min(self.cur_dist[i] + self.last_dist[i], self.inf);
        }
    }

    fn dijkstra_eloge(&mut self) {
        #[derive(PartialEq)]
        struct State<C: Number>(C, usize, Option<(usize, usize)>);
        impl<C: Number> PartialOrd for State<C> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl<C: Number> Eq for State<C> {}
        impl<C: Number> Ord for State<C> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }
        let n = self.g.len();
        let m = self.g.iter().map(|x| x.len()).sum();

        self.cur_dist.fill(self.inf);
        self.prev.fill(None);

        let mut pq = BinaryHeap::with_capacity(m);
        pq.push(Reverse(State(C::ZERO, self.source, None)));
        while let Some(Reverse(State(weight, head, last_edge))) = pq.pop() {
            if self.cur_dist[head] <= weight {
                continue;
            }
            self.cur_dist[head] = weight;
            self.prev[head] = last_edge;
            for (index, e) in self.g[head].iter().enumerate() {
                if rev_edge(self.g, e).0.flow() == C::ZERO {
                    continue;
                }
                let to = e.0.to();
                let dist =
                    self.cur_dist[head] + e.0.cost() - self.last_dist[to] + self.last_dist[head];
                pq.push(Reverse(State(dist, to, Some((head, index)))));
            }
        }
    }

    fn dijkstra_v2(&mut self) {
        let n = self.g.len();
        self.cur_dist.fill(self.inf);
        self.prev.fill(None);
        let mut visited = vec![false; n];

        self.cur_dist[self.source] = C::ZERO;
        for _ in 0..n {
            let mut head = None;
            for j in 0..n {
                if !visited[j]
                    && (head.is_none() || self.cur_dist[j] < self.cur_dist[head.unwrap()])
                {
                    head = Some(j);
                }
            }
            let head = head.unwrap();
            if self.cur_dist[head] >= self.inf {
                break;
            }
            visited[head] = true;
            for (index, e) in self.g[head].iter().enumerate() {
                if rev_edge(self.g, e).0.flow() == C::ZERO {
                    continue;
                }
                let to = e.0.to();
                let dist =
                    self.cur_dist[head] + e.0.cost() - self.last_dist[to] + self.last_dist[head];
                if self.cur_dist[to] <= dist {
                    continue;
                }
                self.cur_dist[to] = dist;
                self.prev[to] = Some((head, index));
            }
        }
    }

    pub fn try_push(&mut self, send: C) -> (C, C) {
        let mut remain = send;
        let mut cost = C::ZERO;
        while remain > C::ZERO {
            if self.use_priority_queue {
                self.dijkstra_eloge();
            } else {
                self.dijkstra_v2();
            }
            self.fix_dist();
            if self.prev[self.sink].is_none() {
                break;
            }
            let mut max_flow = remain;
            let mut sum_of_cost = C::ZERO;
            let mut trace = self.prev[self.sink];
            while let Some((root, index)) = trace {
                let e = &self.g[root][index];
                max_flow = min(max_flow, rev_edge(self.g, e).0.flow());
                sum_of_cost = sum_of_cost + e.0.cost();
                trace = self.prev[root];
            }
            if !(self.callback)(max_flow, sum_of_cost) {
                break;
            }
            let mut trace = self.prev[self.sink];
            while let Some((root, index)) = trace {
                send_flow(self.g, root, index, max_flow);
                trace = self.prev[root];
            }
            should!(max_flow > C::ZERO);
            cost = cost + sum_of_cost * max_flow;
            remain = remain - max_flow;
        }

        (send - remain, cost)
    }
}

impl<'a, C: Concrete, E: CostFlowEdge<C>> CostFlowAugment<C> for CostFlowAugmentDijkstra<'a, C, E> {
    fn set_augment_callback(&mut self, callback: impl FnMut(C, C) -> bool + 'static) {
     
        self.callback = Box::new(callback);
    }
}
