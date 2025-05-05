use std::ops;

#[inline(always)]
pub fn lerp<N>(a: N, b: N, t: N) -> N
where
    N: Clone + Copy + ops::Add<Output = N> + ops::Sub<Output = N> + ops::Mul<Output = N>,
{
    a + t * (b - a)
}

#[inline(always)]
pub fn map_range<A>(a1: A, a2: A, val: A, b1: A, b2: A) -> A
where
    A: Copy
        + ops::Add<Output = A>
        + ops::Sub<Output = A>
        + ops::Mul<Output = A>
        + ops::Div<Output = A>,
{
    (val - a1) / (a2 - a1) * (b2 - b1) + b1
}
