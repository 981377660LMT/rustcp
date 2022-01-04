use crate::{two_sat::TwoSat, strongly_connected_component::strongly_connected_component_di};

#[derive(Debug, Clone)]
pub struct TwoSatSparse {
    adj: Vec<Vec<usize>>,
    n: usize,
}

impl TwoSat for TwoSatSparse {
    fn dep_on(&mut self, a: usize, b: usize) {
        self.adj[a].push(b);
    }
}

impl TwoSatSparse {
    pub fn new(n: usize) -> Self {
        TwoSatSparse {
            adj: vec![Vec::new(); 2 * n],
            n,
        }
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        let n = self.n;
        let mut values = vec![false; 2 * n];
        let set = strongly_connected_component_di(&self.adj);
        for i in 0..n { 
            if set[Self::id(i)] == set[Self::negate_id(i)] {
                return None;
            }
        }
        let mut dfns = vec![false; 2 * n];
        for i in 0..2 * n {
            self.assign(i, &mut dfns, &set, &mut values);
            values[i] = values[set[i]];
        }
        Some(values.into_iter().step_by(2).collect())
    }

    fn assign(&self, root: usize, dfns: &mut Vec<bool>, sets: &Vec<usize>, values: &mut Vec<bool>) {
        if dfns[root] {
            return;
        }
        dfns[root] = true;
        for &e in self.adj[root].iter() {
            self.assign(e, dfns, sets, values);
        }
        if sets[root] == root {
            values[root] = !values[sets[Self::negate(root)]];
        }
    }
}