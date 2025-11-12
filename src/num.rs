pub trait ConstTwo {
    const TWO: Self;
}

macro_rules! two_impl {
    ($t:ty, $v:expr) => {
        impl ConstTwo for $t {
            const TWO: Self = $v;
        }
    };
}

two_impl!(f32, 2.0);
two_impl!(f64, 2.0);
