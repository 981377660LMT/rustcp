use std::fmt::Debug;

use crate::{num_number::Number};

pub trait DiEdge {
    fn to(&self) -> usize;
}

impl DiEdge for usize {
    fn to(&self) -> usize {
        *self
    }
}

pub trait WeightEdge<W>: DiEdge
where
    W: Number,
{
    fn weight(&self) -> W;
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
    W: Number,
{
    pub to: usize,
    pub weight: W,
}
impl<W> SimpleWeightDiEdge<W>
where
    W: Number,
{
    pub fn new(to: usize, weight: W) -> Self {
        Self { to, weight }
    }
}
impl<W> DiEdge for SimpleWeightDiEdge<W>
where
    W: Number,
{
    fn to(&self) -> usize {
        self.to
    }
}
impl<W> WeightEdge<W> for SimpleWeightDiEdge<W>
where
    W: Number,
{
    fn weight(&self) -> W {
        self.weight
    }
}

#[derive(Debug, Clone)]
pub struct BiEdge<E: DiEdge>(pub E, usize);

impl<E: DiEdge> BiEdge<E> {
    pub fn rev(&self) -> usize {
        self.1
    }

    pub fn unwrap(&self) -> &E {
        &self.0
    }

    pub fn unwrap_mut(&mut self) -> &mut E {
        &mut self.0
    }
}
impl<E: DiEdge> DiEdge for BiEdge<E> {
    fn to(&self) -> usize {
        self.0.to()
    }
} 
impl<W: Number, E: WeightEdge<W>> WeightEdge<W> for BiEdge<E> {
    fn weight(&self) -> W {
        self.0.weight()
    }
}

pub fn add_bi_edge<E: DiEdge>(g: &mut Vec<Vec<BiEdge<E>>>, ab: E, ba: E) {
    let b = ab.to();
    let a = ba.to();
    let bi_ab = BiEdge(ab, g[b].len());
    let mut bi_ba = BiEdge(ba, g[a].len());
    if a == b {
        bi_ba.1 += 1;
    }
    g[a].push(bi_ab);
    g[b].push(bi_ba);
}



pub fn rev_edge<'a, E: DiEdge>(g: &'a Vec<Vec<BiEdge<E>>>, e: &BiEdge<E>) -> &'a BiEdge<E> {
    &g[e.0.to()][e.rev()]
}

pub fn src_of_bi_edge<E: DiEdge>(g: &Vec<Vec<BiEdge<E>>>, e: &BiEdge<E>) -> usize {
    rev_edge(g, e).to()
}
