
pub trait TwoSat {
    fn id(x: usize) -> usize {
        x * 2
    }
    fn negate_id(x: usize) -> usize {
        x * 2 + 1
    }
    fn negate(x: usize) -> usize {
        x ^ 1
    }
    fn dep_on(&mut self, a: usize, b: usize);
    fn deduce(&mut self, a: usize, b: usize) {
        self.dep_on(a, b);
        self.dep_on(Self::negate(b), Self::negate(a));
    }
    fn or(&mut self, a: usize, b: usize) {
        self.deduce(Self::negate(a), b);
    }
    fn always_true(&mut self, a: usize) {
        self.dep_on(Self::negate(a), a);
    }
    fn always_false(&mut self, a: usize) {
        self.dep_on(a, Self::negate(a));
    }
    fn same(&mut self, a: usize, b: usize) {
        self.deduce(a, b);
        self.deduce(b, a);
    }
    fn xor(&mut self, a: usize, b: usize) {
        self.same(a, Self::negate(b));
    }
    fn at_least_one_false(&mut self, a: usize, b: usize) {
        self.deduce(a, Self::negate(b));
    }
    fn at_least_one_true(&mut self, a: usize, b: usize) {
        self.or(a, b);
    }
}

