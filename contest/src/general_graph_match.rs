use std::mem::swap;

use crate::macros::should;
pub struct GeneralGraphMatch {
    n: usize,
    pre: Vec<usize>,
    edges: Vec<Vec<bool>>,
    mate: Vec<usize>,
    link: Vec<usize>,
    vis: Vec<usize>,
    fa: Vec<usize>,
    que: Vec<usize>,
    hd: usize,
    tl: usize,
    ss: Vec<usize>,
    tim: usize,
}


impl GeneralGraphMatch {
    pub fn new(n: usize) -> Self {
        let len = n + 1;
        Self {
            n,
            pre: vec![0; len],
            edges: vec![vec![false; len]; len],
            mate: vec![0; len],
            link: vec![0; len],
            vis: vec![0; len],
            fa: vec![0; len],
            que: vec![0; len],
            ss: vec![0; len],
            hd: 0,
            tl: 0,
            tim: 0,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.fa[x] == x {
        } else {
            self.fa[x] = self.find(self.fa[x]);
        }
        self.fa[x]
    }
    fn lca(&mut self, mut x: usize, mut y: usize) -> usize {
        self.tim += 1;
        while self.ss[x] != self.tim {
            if x != 0 {
                self.ss[x] = self.tim;
                x = self.find(self.link[self.mate[x]]);
            }
            swap(&mut x, &mut y);
        }
        x
    }
    
    fn flower(&mut self, mut x: usize, mut y: usize, p: usize) {
        while self.find(x) != p {
            self.link[x] = y;
            y = self.mate[x];
            self.fa[y] = p;
            self.fa[x] = p;
            if self.vis[y] == 1 {
                self.que[self.tl] = y;
                self.vis[y] = 2;
                self.tl += 1;
            }
            x = self.link[y];
        }
    }

    pub fn try_match(&mut self, mut x: usize) -> bool {
        self.hd = 0;
        self.tl = 0;
        for i in 1..=self.n {
            self.fa[i] = i;
            self.vis[i] = 0;
        }
        self.que[self.tl] = x;
        self.vis[x] = 2;
        self.tl += 1;
        while self.hd < self.tl {
            x = self.que[self.hd];
            self.hd += 1;
            for mut u in 1..=self.n {
                if !self.edges[x][u] {
                    continue;
                }
                if self.vis[u] == 0 {
                    self.vis[u] = 1;
                    self.link[u] = x;
                    if self.mate[u] == 0 {
                        while x != 0 {
                            x = self.mate[self.link[u]];
                            self.mate[u] = self.link[u];
                            self.mate[self.link[u]] = u;
                            u = x;
                        }
                        return true;
                    } else {
                        self.que[self.tl] = self.mate[u];
                        self.vis[self.mate[u]] = 2;
                        self.tl += 1;
                    }
                } else if self.vis[u] == 2 && self.find(u) != self.find(x) {
                    let p = self.lca(x, u);
                    self.flower(x, u, p);
                    self.flower(u, x, p);
                }
            }
        }
        false
    }
    pub fn delete_edge_if_not_derease_matching_number(&mut self, mut a: usize, mut b: usize) -> bool {
        a += 1;
        b += 1;
        should!(self.edges[a][b]);
        if self.mate[a] != b {
            self.edges[a][b] = false;
            self.edges[b][a] = false;
            return true;
        }
        self.mate[a] = 0;
        self.mate[b] = 0;
        self.edges[a][b] = false;
        self.edges[b][a] = false;
        if self.try_match(a) || self.try_match(b) {
            return true;
        }
        //rollback
        self.mate[a] = b;
        self.mate[b] = a;
        self.edges[a][b] = true;
        self.edges[b][a] = true;
        return false;
    }

    pub fn mate(&self, mut i: usize) -> Option<usize> {
        let i = i + 1;
        if self.mate[i] == 0 {
            None
        } else {
            Some(self.mate[i] - 1)
        }
    }

    pub fn add_edge(&mut self, x: usize, y: usize) {
        let x = x + 1;
        let y = y + 1;
        self.edges[x][y] = true;
        self.edges[y][x] = true;
    }

    pub fn max_match(&mut self, greedy: bool, perm: &[usize]) -> usize {
        let mut total = 0;
        if greedy {
            for i in 1..=self.n {
                for j in i+1..=self.n {
                    if self.edges[i][j] && self.mate[i] == 0 && self.mate[j] == 0 {
                        self.mate[i] = j;
                        self.mate[j] = i;
                        total += 1;
                    }
                }
            }
        }

        for i in perm {
            let i = i + 1;
            if self.mate[i] == 0 && self.try_match(i) {
                total += 1;
            }
        }

        total
    }
}