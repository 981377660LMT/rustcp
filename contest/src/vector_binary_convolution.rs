use crate::{
    algebraic_structure::{Field, Ring},
    num_number::FromNumber,
};

fn and<T: Ring>(a: T, b: T) -> (T, T) {
    (a + b, b)
}
fn iand<T: Ring>(a: T, b: T) -> (T, T) {
    (a - b, b)
}
fn or<T: Ring>(a: T, b: T) -> (T, T) {
    (a, b + a)
}
fn ior<T: Ring>(a: T, b: T) -> (T, T) {
    (a, b - a)
}
fn xor<T: Ring>(a: T, b: T) -> (T, T) {
    (a + b, a - b)
}
fn ixor<T: Field + FromNumber>(a: T, b: T) -> (T, T) {
    ((a + b) / FromNumber::from(2), (a - b) / FromNumber::from(2))
}

pub struct FWTLayer<T: Ring  + 'static> {
    layers: Vec<Box<dyn FnMut(T, T) -> (T, T)>>,
    rev_layers: Vec<Box<dyn FnMut(T, T) -> (T, T)>>,
}
impl<T: Field + FromNumber + 'static> FWTLayer<T> {
    pub fn add_xor_layer(&mut self) {
        self.add(xor, ixor);
    }
    pub fn add_multi_xor_layer(&mut self, n: usize) {
        for _ in 0..n {
            self.add_xor_layer();
        }
    }
    pub fn add_xor_layer_with_inv2(&mut self, inv2: T) -> &mut Self {
        self.add(xor, move |a, b| ((a + b) * inv2, (a - b) * inv2))
    }
    pub fn add_multi_xor_layer_with_inv2(&mut self, inv2: T, n: usize) {
        for _ in 0..n {
            self.add_xor_layer_with_inv2(inv2);
        }
    }
}
impl<T: Ring + 'static> FWTLayer<T> {
    pub fn new(cap: usize) -> Self {
        FWTLayer {
            layers: Vec::with_capacity(cap),
            rev_layers: Vec::with_capacity(cap),
        }
    }
    pub fn add(&mut self, layer: impl FnMut(T, T) -> (T, T) + 'static, rev: impl FnMut(T, T) -> (T, T) + 'static) -> &mut Self{
        self.layers.push(Box::new(layer));
        self.rev_layers.push(Box::new(rev));
        self
    }
    pub fn add_and_layer(&mut self) -> &mut Self {
        self.add(and, iand)
    }
    pub fn add_multi_and_layer(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.add_and_layer();
        }
        self
    }
    pub fn add_or_layer(&mut self) -> &mut Self {
        self.add(or, ior)
    }
    pub fn add_multi_or_layer(&mut self, n: usize) -> &mut Self{
        for _ in 0..n {
            self.add_or_layer();
        }
        self
    }
    
    pub fn apply(&mut self, data: &mut [T]) {
        self.apply_internal(0, data)
    }

    fn apply_internal(&mut self, offset: usize, data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        let m = data.len() / 2;
        let (a, b) = data.split_at_mut(m);
        self.apply_internal(offset + 1, a);
        self.apply_internal(offset + 1, b);
        
        let layer = &mut self.layers[offset];
        for i in 0..m {
            let (a, b) = layer(data[i], data[i + m]);
            data[i] = a;
            data[i + m] = b;
        }
    }

    pub fn inverse(&mut self, data: &mut [T]) {
        self.inverse_internal(self.layers.len() - 1, data);
    }
    fn inverse_internal(&mut self, offset: usize, data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        let layer = &mut self.rev_layers[offset];
        let m = data.len() / 2;
        for i in 0..m {
            let (a, b) = layer(data[i], data[i + m]);
            data[i] = a;
            data[i + m] = b;
        }
        let (a, b) = data.split_at_mut(m);
        self.inverse_internal(offset - 1, a);
        self.inverse_internal(offset - 1, b);
    }
}
