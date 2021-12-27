use crate::num_number::Number;

struct Row<'a, T: Number> {
    inf: T,
    table: Vec<Vec<T>>,
    left_label: Vec<T>,
    right_label: Vec<T>,
    left_partner: Vec<usize>,
    right_partner: Vec<usize>,
    n: usize,
    weights: &'a Vec<Vec<T>>,
}




