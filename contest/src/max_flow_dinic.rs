use std::{marker::PhantomData, cmp::min};

use crate::{
    graph::{rev_edge, BiEdge, DiEdge},
    max_flow::{shortest_path_for_flow, FlowEdge, send_flow},
    num_number::Number, num_concrete::Concrete
};

struct Row<'a, C: Number, E: FlowEdge<C>> {
    iter: Vec<usize>,
    dist: Vec<usize>,
    g: &'a mut Vec<Vec<BiEdge<E>>>,
    phantom: PhantomData<C>,
    sink: usize,
}

pub fn dinic<C: Concrete, E: FlowEdge<C>>(
    g: &mut Vec<Vec<BiEdge<E>>>,
    source: usize,
    sink: usize,
    send: C,
) -> C {
    let mut flow = C::ZERO;
    let n = g.len();
    let mut row = Row {
        iter: vec![0; n],
        dist: Vec::new(),
        g,
        phantom: PhantomData,
        sink,
    };
    while flow < send {
        row.dist = shortest_path_for_flow(&row.g, &[sink]);
        if row.dist[source] == usize::MAX {
            break;
        }
        for i in 0..n {
            row.iter[i] = row.g[i].len() - 1;
        }
        flow = flow + dinic_push(source, send - flow, &mut row);
    }
    flow
}

fn dinic_push<'a, C: Concrete, E: FlowEdge<C>>(root: usize, mut flow: C, row: &mut Row<'a, C, E>) -> C {
    if root == row.sink {
        return flow;
    }
    let snapshot = flow;
    while row.iter[root] != usize::MAX && flow != C::ZERO {
        let e = &row.g[root][row.iter[root]];
        let cap = rev_edge(row.g, e).0.flow();
        if row.dist[e.to()] + 1 == row.dist[root] && cap != C::ZERO {
            let sent = dinic_push(e.to(), min(flow, cap), row);
            if sent != C::ZERO {
                flow = flow - sent;
                send_flow(row.g, root, row.iter[root], sent);
                continue;
            }
        }
        row.iter[root] -= 1;
    }
    snapshot - flow
}
