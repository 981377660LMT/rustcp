use crate::graph::DiEdge;

///
/// Get the depth on forest, parent of root is usize::MAX
/// 
/// # Example
/// 
/// ```
/// use template::root_tree::*;
/// let p = vec![usize::MAX, 0, 0, 1, 3, 2];
/// let res = depth_on_tree(&p);
/// assert_eq!(res, vec![0, 1, 1, 2, 3, 2]);
/// ```
/// 
pub fn depth_on_tree(p: &[usize]) -> Vec<usize> {
    let mut depth = vec![usize::MAX; p.len()];
    for i in 0..p.len() {
        depth_on_tree_with_memory(p, &mut depth, i as usize);
    }
    depth
}
fn depth_on_tree_with_memory(p: &[usize], depth: &mut Vec<usize>, root: usize) -> usize {
    if root == usize::MAX {
        return usize::MAX;
    }
    if depth[root as usize] == usize::MAX {
        depth[root as usize] = depth_on_tree_with_memory(p, depth, p[root as usize]) + 1;
    }
    depth[root as usize]
}


#[derive(Debug)]
pub struct EulerTourTrace<'a, E>
where E: DiEdge {
    adj: &'a Vec<Vec<E>>,
    pub euler_trace: Vec<usize>,
    pub first_seen: Vec<usize>,
}

impl<'a, E> EulerTourTrace<'a, E>
where E: DiEdge {
    pub fn new<F>(adj: &'a Vec<Vec<E>>, is_root: F) -> Self
    where F: Fn(usize) -> bool {
        let n = adj.len();
        let mut ans = Self {
            adj,
            euler_trace: Vec::with_capacity(n + n - 1),
            first_seen: vec![0; n],
        };
        for i in 0..n {
            if is_root(i) {
                ans.dfs(i, usize::MAX);
            }
        }
        ans
    }

    fn dfs(&mut self, root: usize, fa: usize) {
        self.first_seen[root] = self.euler_trace.len();
        self.euler_trace.push(root);
        for e in self.adj[root].iter() {
            if e.to() == fa {
                continue;
            }
            self.dfs(e.to(), root);
            self.euler_trace.push(root);
        }
    }
}