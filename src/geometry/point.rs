/// Defines a method that extracts the coordinates of a point-like object. Implemented for n-dimensional arrays. Implement this trait for your own point type if you want to use it directly.
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
