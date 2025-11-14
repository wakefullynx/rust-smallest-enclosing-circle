pub trait PointLike<C, const N: usize> {
    fn coordinates(&self) -> [C; N];
}

impl<C, const N: usize> PointLike<C, N> for [C; N] where C: Copy {
    fn coordinates(&self) -> [C; N] {
        *self
    }
}

impl<C, const N: usize> PointLike<C, N> for &[C; N] where C: Copy {
    fn coordinates(&self) -> [C; N] {
        **self
    }
}
