use std::fmt::Debug;

pub trait DiEdge {
    fn to(&self) -> usize;
}

pub trait BiEdge<'a> {
    fn rev(&'a self) -> &'a Self;
}

pub trait WeightEdge<W>: DiEdge
where
    W: Copy + Clone + Debug,
{
    fn weight(&self) -> W;
}

pub trait FlowEdge<'a, C>: BiEdge<'a>
where
    C: Copy + Clone + Debug,
{
    fn capacity(&self) -> C;
    fn flow(&self) -> C;
    fn send(&self, flow: C);
}

pub trait CostFlowEdge<'a, C>: FlowEdge<'a, C>
where
    C: Copy + Clone + Debug,
{
    fn cost(&self) -> C;
}
#[derive(Clone, Copy, Debug)]
pub struct SimpleDiEdge {
    pub to: usize,
}
impl SimpleDiEdge {
    pub fn new(to: usize) -> SimpleDiEdge {
        Self { to }
    }
}
impl DiEdge for SimpleDiEdge {
    fn to(&self) -> usize {
        self.to
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SimpleWeightDiEdge<W>
where
    W: Copy + Clone + Debug,
{
    pub to: usize,
    pub weight: W,
}
impl<W> SimpleWeightDiEdge<W>
where
    W: Copy + Clone + Debug,
{
    pub fn new(to: usize, weight: W) -> Self {
        Self { to, weight }
    }
}
impl<W> DiEdge for SimpleWeightDiEdge<W>
where
    W: Copy + Clone + Debug,
{
    fn to(&self) -> usize {
        self.to
    }
}
impl<W> WeightEdge<W> for SimpleWeightDiEdge<W>
where
    W: Copy + Clone + Debug,
{
    fn weight(&self) -> W {
        self.weight
    }
}
