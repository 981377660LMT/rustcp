use crate::{rand::rng, collection::swap_element};

pub fn shuffle<T>(data: &mut [T]) {
    for i in (0..data.len()).into_iter().rev() {
        let swap_pos = rng().limit_usize(i + 1);
        swap_element(data, i, swap_pos);
    }
}

pub fn random_permutation(n: usize) -> Vec<usize> {
    let mut data: Vec<usize> = (0..n).collect();
    shuffle(&mut data);
    data
}