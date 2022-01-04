use crate::{num_number::Number, max_flow::FlowEdge, graph::{DiEdge, add_bi_edge, BiEdge}};


pub trait CostFlowEdge<C>: FlowEdge<C>
where
    C: Number,
{
    fn cost(&self) -> C;
}
#[derive(Clone, Copy, Debug)]
pub struct SimpleCostFlowEdge<C: Number>(usize, C, bool, C);

impl<C:Number> DiEdge for SimpleCostFlowEdge<C> {
    fn to(&self) -> usize {
        self.0
    }
}
impl<C:Number> FlowEdge<C> for SimpleCostFlowEdge<C> {
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
impl<C:Number> CostFlowEdge<C> for SimpleCostFlowEdge<C> {
    fn cost(&self) -> C {
        self.3
    }
}

pub fn add_cost_flow_edge_di<C: Number>(g: &mut Vec<Vec<BiEdge<SimpleCostFlowEdge<C>>>>, a: usize, b: usize, cap_ab: C, cost: C) {
    add_bi_edge(g, SimpleCostFlowEdge(b, C::ZERO, true, cost), SimpleCostFlowEdge(a, cap_ab, false, cost.negative()));
}