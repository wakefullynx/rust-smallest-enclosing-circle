use geometry_predicates::orient2d;

use crate::geometry::point::PointLike;

/// Defines the determined state as a result of the [`Orientation::orientation`] operation, i.e., whether the three given points are in [`OrientationState::CounterClockwise`], [`OrientationState::Clockwise`], or [`OrientationState::Collinear`] order (mathematical, upward y-axis).
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OrientationState {
    CounterClockwise,
    Clockwise,
    Collinear,
}

/// A trait that allows the determination whether the three given points are in [`OrientationState::CounterClockwise`], [`OrientationState::Clockwise`], or [`OrientationState::Collinear`] order.
pub trait Orientation<T> {
    fn orientation(
        a: &impl PointLike<T, 2>,
        b: &impl PointLike<T, 2>,
        c: &impl PointLike<T, 2>,
    ) -> OrientationState;
}

/// An empty struct that implements the default [`Orientation`] trait used in this library.
pub struct DefaultOrientation;

impl Orientation<f64> for DefaultOrientation {
    /// Default implementation of the [`Orientation`] trait, uses the [`geometry_predicates`] crate.
    fn orientation(
        a: &impl PointLike<f64, 2>,
        b: &impl PointLike<f64, 2>,
        c: &impl PointLike<f64, 2>,
    ) -> OrientationState {
        let o = orient2d(a.coordinates(), b.coordinates(), c.coordinates());
        if o > 0. {
            OrientationState::CounterClockwise
        } else if o < 0. {
            OrientationState::Clockwise
        } else {
            OrientationState::Collinear
        }
    }
}

/// *Almost* identical to the [`Orientation`] trait, but returns the signed area of the spanned parallelogram of the given three points. The sign also indicates the same information as [`Orientation::orientation`] (positive if counterclockwise, negative if clockwise, otherwise collinear for mathematical, upward y-axis). This is identical to the [`geometry_predicates`] definition and necessary for the computation of circumcircles.
pub trait OrientationArea<T> {
    fn orientation(
        a: &impl PointLike<T, 2>,
        b: &impl PointLike<T, 2>,
        c: &impl PointLike<T, 2>,
    ) -> T;
}

/// An empty struct that implements the default [`OrientationArea`] trait used in this library.
pub struct DefaultOrientationArea;

impl OrientationArea<f64> for DefaultOrientationArea {
    /// Default implementation of the [`Orientation`] trait, uses the [`geometry_predicates`] crate.
    fn orientation(
        a: &impl PointLike<f64, 2>,
        b: &impl PointLike<f64, 2>,
        c: &impl PointLike<f64, 2>,
    ) -> f64 {
        orient2d(a.coordinates(), b.coordinates(), c.coordinates())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod orientation {
        use super::*;
        #[test]
        fn counter_clockwise() {
            assert_eq!(
                DefaultOrientation::orientation(&[0.0, 0.0], &[1.0, 0.0], &[1.0, 1.0]),
                OrientationState::CounterClockwise
            )
        }

        #[test]
        fn clockwise() {
            assert_eq!(
                DefaultOrientation::orientation(&[0.0, 0.0], &[1.0, 0.0], &[1.0, -1.0]),
                OrientationState::Clockwise
            )
        }

        #[test]
        fn collinear() {
            assert_eq!(
                DefaultOrientation::orientation(&[0.0, 0.0], &[1.0, 0.0], &[2.0, 0.0]),
                OrientationState::Collinear
            )
        }
    }

    mod orientation_area {
        use super::*;
        #[test]
        fn counter_clockwise() {
            assert_eq!(
                DefaultOrientationArea::orientation(&[0.0, 0.0], &[1.0, 0.0], &[1.0, 1.0]),
                1.0
            )
        }

        #[test]
        fn clockwise() {
            assert_eq!(
                DefaultOrientationArea::orientation(&[0.0, 0.0], &[1.0, 0.0], &[1.0, -1.0]),
                -1.0
            )
        }

        #[test]
        fn collinear() {
            assert_eq!(
                DefaultOrientationArea::orientation(&[0.0, 0.0], &[1.0, 0.0], &[2.0, 0.0]),
                0.0
            )
        }
    }
}
