use std::fmt::Debug;

use crate::template_macro::should_eq;

const NIL: usize = 0;

#[derive(Clone, Debug)]
pub struct Row<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    pub left: usize,
    pub right: usize,
    pub father: usize,
    pub reverse: bool,
    pub tree_father: usize,
    pub id: usize,
    pub weight: T,
    pub sum: T,
    pub rev_sum: T,
    pub upd: U,

    pub tree_size: usize,
    pub vtree_size: usize,
    pub tree_weight: usize,
}

impl<T, U> Row<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug + Eq,
{
    pub fn new(s: T, u: U, tree_weight: usize, id: usize) -> Self {
        Self {
            left: 0,
            right: 0,
            father: 0,
            reverse: false,
            tree_father: 0,
            id,
            sum: s.clone(),
            rev_sum: s.clone(),
            upd: u,
            weight: s,
            tree_size: tree_weight,
            vtree_size: 0,
            tree_weight: tree_weight,
        }
    }
}

///
/// Link-cut-tree implementation
///
/// # Example
///
/// 0
/// | \
/// 1   3
///
/// 2 - 4
///
///
/// ```
/// use template::link_cut_tree::*;
/// use std::cmp::*;
/// let mx_f = |x: &i32,y: &i32| -> i32 {*max(x,y)};
/// let mut lct = LinkCutTree::new(5, 0, 0, mx_f, mx_f, mx_f, |x| 1);
/// for i in 0..5 {
///     lct.modify(i, &(i as i32));
/// }
/// lct.join(0, 1);
/// lct.join(0, 3);
/// lct.join(2, 4);
/// lct.find_path(2, 4);
/// lct.splay(2);
///
/// lct.find_path(0, 3);
///
/// lct.splay(0);
/// assert_eq!(lct.node(0).sum, 3);
/// assert_eq!(lct.node(2).sum, 4);
///
/// assert_eq!(lct.node(0).tree_size, 3);
/// assert_eq!(lct.node(2).tree_size, 2);
/// lct.make_root(0);
///
/// assert_eq!(lct.lca(1, 3), 0);
/// assert!(lct.connected(0, 1));
/// assert!(lct.connected(2, 4));
/// assert!(!lct.connected(1, 4));
/// lct.access(3);
/// lct.splay(0);
/// lct.modify(0, &2);
/// assert_eq!(lct.node(0).sum, 3);
/// lct.access(1);
/// lct.splay(0);
/// assert_eq!(lct.node(0).sum, 2);
/// lct.cut(0, 1);
/// assert_eq!(lct.node(1).sum, 1);
/// ```
///
pub struct LinkCutTree<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug + Eq,
{
    nodes: Vec<Row<T, U>>,
    s_f: Box<dyn Fn(&T, &T) -> T>,
    u_f: Box<dyn Fn(&U, &U) -> U>,
    s_u_f: Box<dyn Fn(&T, &U) -> T>,
    zero_T: T,
    zero_U: U,
}

impl<T, U> Debug for LinkCutTree<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkCutTree")
            .field("nodes", &self.nodes)
            .finish()
    }
}

impl<T, U> LinkCutTree<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug + Eq,
{
    pub fn new(
        n: usize,
        zero_T: T,
        zero_U: U,
        s_f: impl Fn(&T, &T) -> T + 'static,
        u_f: impl Fn(&U, &U) -> U + 'static,
        s_u_f: impl Fn(&T, &U) -> T + 'static,
        tree_weight: impl Fn(usize) -> usize,
    ) -> Self {
        let mut res = Self {
            nodes: Vec::with_capacity(n + 1),
            s_f: Box::new(s_f),
            u_f: Box::new(u_f),
            s_u_f: Box::new(s_u_f),
            zero_T,
            zero_U,
        };
        res.nodes.push(Row::new(
            res.zero_T.clone(),
            res.zero_U.clone(),
            0,
            0,
        ));
        for i in 0..n {
            res.nodes.push(Row::new(
                res.zero_T.clone(),
                res.zero_U.clone(),
                tree_weight(i),
                i + 1,
            ));
        }

        res
    }

    pub fn init(&mut self, mut id: usize) {
        self.nodes[id + 1] = Row::new(
            self.zero_T.clone(),
            self.zero_U.clone(),
            self.nodes[id + 1].tree_weight,
            id,
        );
    }

    pub fn access(&mut self, x: usize) {
        self.access_internal(x + 1);
    }

    pub fn splay(&mut self, id: usize) {
        self.splay_internal(id + 1);
    }

    fn splay_internal(&mut self, x: usize) {
        if x == NIL {
            return;
        }
        loop {
            let y = self.nodes[x].father;
            if y == NIL {
                break;
            }
            let z = self.nodes[y].father;
            if z == NIL {
                self.push_down(y);
                self.push_down(x);

                if x == self.nodes[y].left {
                    self.zig(x);
                } else {
                    self.zag(x);
                }
            } else {
                self.push_down(z);
                self.push_down(y);
                self.push_down(x);
                if x == self.nodes[y].left {
                    if y == self.nodes[z].left {
                        self.zig(y);
                        self.zig(x);
                    } else {
                        self.zig(x);
                        self.zag(x);
                    }
                } else {
                    if y == self.nodes[z].left {
                        self.zag(x);
                        self.zig(x);
                    } else {
                        self.zag(y);
                        self.zag(x);
                    }
                }
            }
        }

        self.push_down(x);
        self.push_up(x);
    }

    ///
    /// reverse the whole tree, O(1)
    ///
    pub fn reverse(&mut self, id: usize) {
        self.reverse_internal(id + 1)
    }

    fn reverse_internal(&mut self, id: usize) {
        if id == NIL {
            return;
        }
        self.nodes[id].reverse = !self.nodes[id].reverse;
        
        let tmp = self.nodes[id].sum.clone();
        self.nodes[id].sum = self.nodes[id].rev_sum.clone();
        self.nodes[id].rev_sum = tmp.clone();
    }

    fn push_down(&mut self, id: usize) {
        if id == NIL {
            return;
        }
        if self.nodes[id].reverse {
            self.nodes[id].reverse = false;

            let tmp = self.nodes[id].left;
            self.nodes[id].left = self.nodes[id].right;
            self.nodes[id].right = tmp;

            self.reverse_internal(self.nodes[id].left);
            self.reverse_internal(self.nodes[id].right);
        }

        let left = self.nodes[id].left;
        let right = self.nodes[id].right;

        self.nodes[left].tree_father = self.nodes[id].tree_father;
        self.nodes[right].tree_father = self.nodes[id].tree_father;

        if self.nodes[id].upd != self.zero_U {
            let u = self.nodes[id].upd.clone();
            self.modify_internal(left, &u);
            self.modify_internal(right, &u);
            self.nodes[id].upd = self.zero_U.clone();
        }
    }

    fn push_up(&mut self, id: usize) {
        if id == NIL {
            return;
        }
        let left = self.nodes[id].left;
        let right = self.nodes[id].right;
        self.nodes[id].tree_size = self.nodes[left].tree_size
            + self.nodes[right].tree_size
            + self.nodes[id].tree_weight
            + self.nodes[id].vtree_size;
        self.nodes[id].sum = (self.s_f)(
            &(self.s_f)(&self.nodes[left].sum, &self.nodes[id].weight),
            &self.nodes[right].sum,
        );
        self.nodes[id].rev_sum = (self.s_f)(
            &(self.s_f)(&self.nodes[right].rev_sum, &self.nodes[id].weight),
            &self.nodes[left].rev_sum,
        );
    }

    ///
    /// update subtree of node[id], O(1)
    ///
    pub fn modify(&mut self, id: usize, u: &U) {
        self.modify_internal(id + 1, u);
    }

    pub fn set(&mut self, id: usize, t: T) {
        self.set_internal(id + 1, t);
    }

    fn set_internal(&mut self, id: usize, t: T) {
        should_eq!(self.nodes[id].father, NIL);
        self.nodes[id].weight = t;
        self.push_up(id);
    }

    fn modify_internal(&mut self, id: usize, u: &U) {
        if id == NIL {
            return;
        }
        should_eq!(self.nodes[id].father, NIL);
        self.nodes[id].upd = (self.u_f)(&self.nodes[id].upd, u);
        self.nodes[id].weight = (self.s_u_f)(&self.nodes[id].weight, u);
        self.nodes[id].sum = (self.s_u_f)(&self.nodes[id].sum, u);
    }

    fn access_internal(&mut self, mut x: usize) {
        let mut last = NIL;
        while x != NIL {
            self.splay_internal(x);
            let right = self.nodes[x].right;
            self.nodes[right].father = NIL;
            self.nodes[right].tree_father = x;
            self.nodes[x].vtree_size += self.nodes[right].tree_size;
            self.set_right_internal(x, last);
            self.nodes[x].vtree_size -= self.nodes[last].tree_size;
            self.push_up(x);

            last = x;
            x = self.nodes[x].tree_father;
        }
    }

    pub fn set_left(&mut self, id: usize, left: usize) {
        self.set_left_internal(id + 1, left + 1);
    }

    pub fn set_right(&mut self, id: usize, right: usize) {
        self.set_right_internal(id + 1, right + 1);
    }
    fn set_left_internal(&mut self, id: usize, left: usize) {
        self.nodes[id].left = left;
        self.nodes[left].father = id;
    }

    fn set_right_internal(&mut self, id: usize, right: usize) {
        self.nodes[id].right = right;
        self.nodes[right].father = id;
    }

    fn change_child(&mut self, fa: usize, child: usize, to: usize) {
        if self.nodes[fa].left == child {
            self.set_left_internal(fa, to);
        } else {
            self.set_right_internal(fa, to);
        }
    }

    fn zig(&mut self, x: usize) {
        let y = self.nodes[x].father;
        let z = self.nodes[y].father;
        let b = self.nodes[x].right;

        self.set_left_internal(y, b);
        self.set_right_internal(x, y);
        self.change_child(z, y, x);

        self.push_up(y);
    }

    fn zag(&mut self, x: usize) {
        let y = self.nodes[x].father;
        let z = self.nodes[y].father;
        let b = self.nodes[x].left;

        self.set_right_internal(y, b);
        self.set_left_internal(x, y);
        self.change_child(z, y, x);

        self.push_up(y);
    }

    pub fn make_root(&mut self, id: usize) {
        self.make_root_internal(id + 1);
    }

    fn make_root_internal(&mut self, id: usize) {
        self.access_internal(id);
        self.splay_internal(id);
        self.reverse_internal(id);
    }

    pub fn cut(&mut self, y: usize, x: usize) {
        self.cut_internal(y + 1, x + 1);
    }

    fn cut_internal(&mut self, y: usize, x: usize) {
        self.make_root_internal(y);
        self.access_internal(x);
        self.splay_internal(y);
        let right = self.nodes[y].right;
        should_eq!(x, right);
        self.nodes[right].tree_father = NIL;
        self.nodes[right].father = NIL;
        self.set_right_internal(y, NIL);
        self.push_up(y);
    }

    pub fn join(&mut self, y: usize, x: usize) {
        self.join_internal(y + 1, x + 1);
    }
    fn join_internal(&mut self, y: usize, x: usize) {
        self.make_root_internal(x);
        self.make_root_internal(y);
        self.nodes[x].tree_father = y;
        self.nodes[y].vtree_size += self.nodes[x].tree_size;
        self.push_up(y);
    }
    pub fn find_path(&mut self, y: usize, x: usize) {
        self.find_path_internal(y + 1, x + 1);
    }
    fn find_path_internal(&mut self, y: usize, x: usize) {
        self.make_root_internal(y);
        self.access_internal(x);
    }
    pub fn find_root(&mut self, x: usize) -> usize {
        self.find_root_internal(x + 1) - 1
    }
    fn find_root_internal(&mut self, mut x: usize) -> usize {
        self.splay_internal(x);
        self.push_down(x);
        while self.nodes[x].left != NIL {
            x = self.nodes[x].left;
            self.push_down(x);
        }
        self.splay_internal(x);
        x
    }

    pub fn node<'a>(&'a self, x: usize) -> &'a Row<T, U> {
        &self.nodes[x + 1]
    }

    pub fn node_mut<'a>(&'a mut self, x: usize) -> &'a mut Row<T, U> {
        &mut self.nodes[x + 1]
    }
    pub fn lca(&mut self, a: usize, b: usize) -> usize {
        self.lca_internal(a + 1, b + 1) - 1
    }
    fn lca_internal(&mut self, a: usize, b: usize) -> usize {
        if a == b {
            return a;
        }
        self.access_internal(a);
        self.splay_internal(a);
        self.access_internal(b);
        self.splay_internal(b);
        if self.nodes[a].tree_father != NIL {
            return self.nodes[a].tree_father;
        }
        if self.nodes[a].father != NIL {
            return a;
        }
        return NIL;
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.connected_internal(a + 1, b + 1)
    }
    fn connected_internal(&mut self, a: usize, b: usize) -> bool {
        self.lca_internal(a, b) != NIL
    }

    pub fn dfs_internal(&mut self, root: usize, consumer: &mut impl FnMut(&Row<T, U>)) {
        if root == NIL {
            return;
        }
        self.push_down(root);
        self.dfs_internal(self.nodes[root].left, consumer);
        consumer(&self.nodes[root]);
        self.dfs_internal(self.nodes[root].right, consumer);
    }

    fn to_string_tree_structure_internal(&mut self, root: usize, fa: usize, consumer: &mut impl FnMut(usize, usize)) {
        if root == NIL {
            return;
        }
        if fa != NIL {
            consumer(fa, root);
        }
        self.push_down(root);
        self.to_string_tree_structure_internal(self.nodes[root].left, root, consumer);
        self.to_string_tree_structure_internal(self.nodes[root].right, root, consumer);
    }
    pub fn to_string_tree_structure(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![Vec::new(); self.nodes.len() - 1];
        let borrow = self.nodes.clone();
        for i in 1..res.len() + 1 {
            if self.nodes[i].father == NIL {
                self.to_string_tree_structure_internal(i, self.nodes[i].tree_father, &mut |fa, child| {
                    if fa < child {
                        res[fa - 1].push(child - 1)
                    } else {
                        res[child - 1].push(fa - 1)
                    }
                });
            }
        }
        self.nodes = borrow;
        res
    }

}


impl<T, U> LinkCutTree<T, U>
where
    T: Debug + Clone + std::fmt::Display,
    U: Debug + Clone + Eq,
{
    pub fn to_string_tree_weight(&mut self, root: usize) -> String {
        let backup = self.nodes.clone();
        let mut ans = Vec::new();
        self.dfs_internal(root + 1, &mut |x| ans.push(x.weight.to_string()));
        self.nodes = backup;
        ans.join(", ")
    }

}