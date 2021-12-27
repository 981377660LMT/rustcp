use std::{collections::{BinaryHeap, VecDeque}, cmp::Reverse};

use crate::{graph::{WeightEdge, DiEdge}, num_number::Number};


#[derive(Clone, Copy, Debug, PartialEq)]
struct State<T: Number>(T, usize, Option<(usize, usize)>);

impl<T: Number> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T: Number> Eq for State<T> {
    
}

impl<T: Number> Ord for State<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

///
/// dijkstra
/// 
/// - time: O(E\log_2 E)
/// - space: O(V+E)
/// 
/// # Usage
/// 
/// find multiple source multiple sink weighted shortest path
/// 
/// # Return
/// 
/// retval = (distance from source, previous node in the shortest path)
/// 
pub fn shortest_path_weighted_eloge<T: Number, E: WeightEdge<T>>(g: &Vec<Vec<E>>, src: &[usize], inf: T) -> (Vec<T>, Vec<Option<(usize, usize)>>) {
    let n = g.len();
    let cap: usize = g.iter().map(|x| x.len()).sum();
    let mut prev = vec![None; n];
    let mut dist = vec![inf; n];
    let mut pq = BinaryHeap::with_capacity(cap + 1);
    for &root in src {
        pq.push(Reverse(State(T::ZERO, root, None)));
    }
    while let Some(Reverse(State(d, root, last_edge))) = pq.pop() {
        if dist[root] <= d {
            continue;
        }
        dist[root] = d;
        prev[root] = last_edge;

        for (index,e) in g[root].iter().enumerate() {
            let to = e.to();
            let cand = e.weight() + dist[root];
            if dist[to] > cand {
                pq.push(Reverse(State(cand, to, Some((root, index)))));
            }
        }
    }

    (dist, prev)
} 

///
/// dijkstra
/// 
/// - time: O(V^2)
/// - space: O(V+E)
/// 
/// # Usage
/// 
/// find multiple source multiple sink weighted shortest path
/// 
/// # Return
/// 
/// retval = (distance from source, previous edge in the shortest path)
/// 
pub fn shortest_path_weighted_v2<T: Number, E: WeightEdge<T>>(g: &Vec<Vec<E>>, src: &[usize], inf: T) -> (Vec<T>, Vec<Option<(usize, usize)>>) {
    let n = g.len();
    let mut prev = vec![None; n];
    let mut dist = vec![inf; n];
    let mut visited = vec![false; n];
    for &root in src {
        dist[root] = T::ZERO;
    }
    for _ in 0..n {
        let mut head = None;
        for j in 0..n {
            if !visited[j] && (head.is_none() || dist[j] < dist[head.unwrap()]) {
                head = Some(j);
            }
        } 
        let head = head.unwrap();
        if dist[head] == inf {
            break;
        }
        visited[head] = true;
        for (index, e) in g[head].iter().enumerate() {
            let to = e.to();
            let new_dist = dist[head] + e.weight();
            if dist[to] > new_dist {
                dist[to] = new_dist;
                prev[to] = Some((head, index));
            }
        }
    }

    (dist, prev)
} 


///
/// BFS 
/// 
/// - time: O(E+V)
/// - space: O(V)
/// 
/// # Usage
/// 
/// find multiple source multiple sink shortest path (weight for all edge is 1)
/// 
/// # Return
/// 
/// (distance from source, previous node in the shortest path)
/// 
pub fn shortest_path<E: DiEdge>(g: &Vec<Vec<E>>, src: &[usize]) -> (Vec<usize>, Vec<Option<(usize, usize)>>) {
    let n = g.len();
    let mut dist = vec![usize::MAX; n];
    let mut prev = vec![None; n];
    let mut dq = VecDeque::with_capacity(n);
    for &root in src {
        dist[root] = 0;
        dq.push_back(root);
    }
    while let Some(root) = dq.pop_front() {
        for (index, e) in g[root].iter().enumerate() {
            let to = e.to();
            if dist[to] != usize::MAX {
                continue;
            }
            dist[to] = dist[root] + 1;
            prev[to] = Some((root, index));
            dq.push_back(to);
        }
    }

    (dist, prev)
}

///
/// BFS 
/// 
/// - time: O(E+V)
/// - space: O(V)
/// 
/// # Usage
/// 
/// find multiple source multiple sink shortest path (weight for all edge is 0/1)
/// 
/// # Return
/// 
/// (distance from source, previous node in the shortest path)
/// 
pub fn shortest_path_01<W:Number, E: WeightEdge<W>>(g: &Vec<Vec<E>>, src: &[usize]) -> (Vec<usize>, Vec<Option<(usize, usize)>>) {
    let n = g.len();
    let mut dist = vec![usize::MAX; n];
    let mut prev = vec![None; n];
    let mut dq = VecDeque::with_capacity(n);
    let mut visited = vec![false; n];
    for &root in src {
        dist[root] = 0;
        dq.push_back(root);
    }
    while let Some(root) = dq.pop_front() {
        if visited[root] {
            continue;
        }
        visited[root] = true;
        for (index, e) in g[root].iter().enumerate() {
            let to = e.to();
            if dist[to] <= dist[root] + e.weight().as_usize() {
                continue;
            }
            dist[to] = dist[root] + e.weight().as_usize();
            prev[to] = Some((root, index));

            if e.weight() == W::ONE {
                dq.push_back(to);
            } else {
                dq.push_front(to);
            }
        }
    }

    (dist, prev)
}