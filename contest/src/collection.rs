use std::{collections::HashMap, mem::swap};


pub fn group_by_dense<T, V>(n: usize, data: &[T], to_key: impl Fn(usize, &T) -> usize, to_value: impl Fn(usize, &T) -> V) -> Vec<Vec<V>> {
    let mut sizes = vec![0usize; n];
    for (index, x) in data.iter().enumerate() {
        sizes[to_key(index, x)] += 1;
    }
    let mut res: Vec<Vec<V>> = sizes.iter().map(|&x| Vec::with_capacity(x)).collect();
    for (index, x) in data.iter().enumerate() {
        res[to_key(index, x)].push(to_value(index, x));
    }

    res
}

pub fn swap_element<T>(data: &mut [T], a: usize, b: usize) {
    if a > b {
        let (p1, p2) = data.split_at_mut(a);
        swap(&mut p1[b], &mut p2[0]);
    } else if b > a {
        let (p1, p2) = data.split_at_mut(b);
        swap(&mut p1[a], &mut p2[0]);
    }
}

pub fn swap_element_attr<T, V>(data: &mut [T], a: usize, b: usize, extractor: impl Fn(&mut T) -> &mut V) {
    if a > b {
        let (p1, p2) = data.split_at_mut(a);
        swap(extractor(&mut p1[b]), extractor(&mut p2[0]));
    } else if b > a {
        let (p1, p2) = data.split_at_mut(b);
        swap(extractor(&mut p1[a]), extractor(&mut p2[0]));
    }
}