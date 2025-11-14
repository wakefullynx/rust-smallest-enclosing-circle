use geometry_predicates::incircle;

use crate::geometry::point::PointLike;

/// Defines the determined state as a result of the [`InCircle::in_circle`] operation, i.e., whether a given probe point lies [`InCircleState::Inside`]of a circle, [`InCircleState::Outside`] of a circle, or exactly [`InCircleState::On`] a circleof a circle given by three points.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InCircleState {
    Inside,
    Outside,
    On,
}

///A trait that allows to determine whether a given probe point lies [`InCircleState::Inside`]of a circle, [`InCircleState::Outside`] of a circle, or exactly [`InCircleState::On`] a circle given by points `a`, `b`, and `c`.
pub trait InCircle<T> {
    fn in_circle(
        a: &impl PointLike<T, 2>,
        b: &impl PointLike<T, 2>,
        c: &impl PointLike<T, 2>,
        probe: &impl PointLike<T, 2>,
    ) -> InCircleState;
}

/// An empty struct that implements the default [`InCircle`] trait used in this library.
pub struct DefaultInCircle;

impl InCircle<f64> for DefaultInCircle {
    /// Default implementation of the [`Orientation`] trait, uses the [`geometry_predicates`] crate.
    fn in_circle(
        a: &impl PointLike<f64, 2>,
        b: &impl PointLike<f64, 2>,
        c: &impl PointLike<f64, 2>,
        probe: &impl PointLike<f64, 2>,
    ) -> InCircleState {
        let o = incircle(
            a.coordinates(),
            b.coordinates(),
            c.coordinates(),
            probe.coordinates(),
        );
        if o > 0. {
            InCircleState::Inside
        } else if o < 0. {
            InCircleState::Outside
        } else {
            InCircleState::On
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside() {
        assert_eq!(
            DefaultInCircle::in_circle(&[0.0, 0.0], &[1.0, 0.0], &[1.0, 1.0], &[0.5, 0.5]),
            InCircleState::Inside
        )
    }

    #[test]
    fn outside() {
        assert_eq!(
            DefaultInCircle::in_circle(&[0.0, 0.0], &[1.0, 0.0], &[1.0, 1.0], &[1.5, 1.5]),
            InCircleState::Outside
        )
    }

    #[test]
    fn on() {
        assert_eq!(
            DefaultInCircle::in_circle(&[0.0, 0.0], &[1.0, 0.0], &[1.0, 1.0], &[0.0, 1.0]),
            InCircleState::On
        )
    }
}
