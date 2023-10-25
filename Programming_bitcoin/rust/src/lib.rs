#[derive(Debug, Clone, Copy)]
pub enum PointWapper<A> {
    Inf,
    Point { x: A, y: A, a: A, b: A },
}
