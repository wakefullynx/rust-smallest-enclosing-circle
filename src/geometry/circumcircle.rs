use num::traits::real::Real;

use crate::{
    geometry::{num::ConstTwo, point::PointLike},
    predicates::orientation::{DefaultOrientationArea, OrientationArea},
};

pub trait CircumCircle<CenterPoint, Radius> {
    fn circumcircle(&self) -> Option<(CenterPoint, Radius)>;
}

impl<P> CircumCircle<[f64; 2], f64> for [P; 3]
where
    P: PointLike<f64, 2>,
{
    fn circumcircle(&self) -> Option<([f64; 2], f64)> {
        let &[a, b, c] = &self.each_ref().map(|p| p.coordinates());
        Some(circumcircle2d::<f64, DefaultOrientationArea>(a, b, c))
    }
}

impl<P> CircumCircle<[f64; 2], f64> for [P; 2]
where
    P: PointLike<f64, 2>,
{
    fn circumcircle(&self) -> Option<([f64; 2], f64)> {
        let &[a, b] = &self.each_ref().map(|p| p.coordinates());
        let center = [(a[0] + b[0]) / 2., (a[1] + b[1]) / 2.];
        let radius = f64::hypot(a[0] - b[0], a[1] - b[1]) / 2.;
        Some((center, radius))
    }
}

/// # Panics
/// 
/// This function panics if the given three points are collinear.
pub fn circumcircle2d<C, O>(a: [C; 2], b: [C; 2], c: [C; 2]) -> ([C; 2], C)
where
    C: Real + ConstTwo,
    O: OrientationArea<C>,
{
    let orientation = O::orientation(&a, &b, &c);

    let (b, c, denominator) = if orientation > C::zero() {
        (b, c, C::TWO * orientation)
    } else if orientation < C::zero() {
        (c, b, -C::TWO * orientation)
    } else {
        panic!()
    };

    let [acx, acy, bcx, bcy, abx, aby] = [
        a[0] - c[0],
        a[1] - c[1],
        b[0] - c[0],
        b[1] - c[1],
        a[0] - b[0],
        a[1] - b[1],
    ];
    let [acxs, acys, bcxs, bcys, abxs, abys] = [
        acx * acx,
        acy * acy,
        bcx * bcx,
        bcy * bcy,
        abx * abx,
        aby * aby,
    ];
    let [acxys, bcxys, abxys] = [acxs + acys, bcxs + bcys, abxs + abys];
    let center = [
        c[0] + (acxys * bcy - bcxys * acy) / denominator,
        c[1] + (acx * bcxys - bcx * acxys) / denominator,
    ];
    let radius = (bcxys * acxys * abxys).sqrt() / denominator;
    (center, radius)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod circumcircle {
        use super::*;

        #[test]
        fn box_triangle_lower_right() {
            assert_eq!(
                circumcircle2d::<f64, DefaultOrientationArea>(
                    [-1.0, -1.0],
                    [1.0, -1.0],
                    [1.0, 1.0]
                ),
                ([0., 0.], f64::sqrt(2.))
            )
        }
    }

}
