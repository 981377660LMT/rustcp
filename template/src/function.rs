pub trait Merger<A, B, C> {
    fn merge(a: A, b: B) -> C;
}
