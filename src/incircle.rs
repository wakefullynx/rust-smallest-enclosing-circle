use geometry_predicates::incircle;

use crate::point::PointLike;

#[derive(PartialEq, Copy, Clone)]
pub enum InCircle2DState {
    Inside,
    Outside,
    On,
}

pub trait InCircle2D<T> {
    fn incircle(
        a: &impl PointLike<T, 2>,
        b: &impl PointLike<T, 2>,
        c: &impl PointLike<T, 2>,
        probe: &impl PointLike<T, 2>,
    ) -> InCircle2DState;
}

pub struct DefaultInCircle2D;

impl InCircle2D<f64> for DefaultInCircle2D {
    fn incircle(
        a: &impl PointLike<f64, 2>,
        b: &impl PointLike<f64, 2>,
        c: &impl PointLike<f64, 2>,
        probe: &impl PointLike<f64, 2>,
    ) -> InCircle2DState {
        let o = incircle(
            a.coordinates(),
            b.coordinates(),
            c.coordinates(),
            probe.coordinates(),
        );
        if o > 0. {
            InCircle2DState::Inside
        } else if o < 0. {
            InCircle2DState::Outside
        } else {
            InCircle2DState::On
        }
    }
}
