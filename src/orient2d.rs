use geometry_predicates::orient2d;

use crate::point::PointLike;

#[derive(PartialEq, Copy, Clone)]
pub enum Orient2DState {
    CounterClockwise,
    Clockwise,
    Collinear
}

pub trait Orient2D<C, P> where P: PointLike<C, 2> {
    fn orient2d(a: P, b: P, c: P) -> Orient2DState;
}

pub struct DefaultOrient2D;

impl<P> Orient2D<f64, P> for DefaultOrient2D where P: PointLike<f64, 2> {
    fn orient2d(a: P, b: P, c: P) -> Orient2DState {
        let o = orient2d(a.coordinates(), b.coordinates(), c.coordinates());
        if o > 0. {
            Orient2DState::CounterClockwise
        }else if o < 0. {
            Orient2DState::Clockwise
        } else {
            Orient2DState::Collinear
        }
    }
}

pub trait Orient2DArea<C, P> where P: PointLike<C, 2> {
    fn orient2d(a: P, b: P, c: P) -> C;
}

pub struct DefaultOrient2DArea;

impl<P> Orient2DArea<f64, P> for DefaultOrient2DArea where P: PointLike<f64, 2> {
    fn orient2d(a: P, b: P, c: P) -> f64 {
        orient2d(a.coordinates(), b.coordinates(), c.coordinates())
    }
}
