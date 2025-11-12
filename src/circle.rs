use std::fmt::Debug;

use crate::{
    circumcircle::CircumCircle,
    incircle::{InCircle2D, InCircle2DState},
    orient2d::{DefaultOrient2D, Orient2D, Orient2DState},
    point::PointLike,
};

/// Represents a circle, defined by up to three points that are located on its circumference.
///
/// This enum has four variants:
/// - None: No circle, i.e., no result.
/// - One: Circle is defined by a single point (center point), and has radius zero.
/// - Two: Circle is defined by two points (i.e., the points are located opposite each other on the circle), and the radius is their half-distance.
/// - Three: Circle is fully define by three points, and additionally holds whether these three points are in counter-clockwise order.
///
/// # Examples
///
/// ```
/// use smallest_enclosing_circle::Circle;
///
/// let c0 = Circle::None;
/// let c1 = Circle::One([0., 0.]);
/// let c2 = Circle::Two([0., 0.], [1., 0.]);
/// let c3 = Circle::Three([0., 0.], [1., 0.], [1., 1.], true);
/// println!("C0: Center: {:?}, Radius: {:?};", c0.center(), c0.radius());
/// // C0: Center: None, Radius: 0.0;
/// println!("C1: Center: {:?}, Radius: {:?};", c1.center(), c1.radius());
/// // C1: Center: Some([0.0, 0.0]), Radius: 0.0;
/// println!("C2: Center: {:?}, Radius: {:?};", c2.center(), c2.radius());
/// // C2: Center: Some([0.5, 0.0]), Radius: 0.5;
/// println!("C3: Center: {:?}, Radius: {:?};", c3.center(), c3.radius());
/// // C3: Center: Some([0.5, 0.5]), Radius: 0.7071067811865476;
/// ```
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Circle2D<Point> {
    None,
    One {
        p: Point,
    },
    Two {
        a: Point,
        b: Point,
    },
    Three {
        a: Point,
        b: Point,
        c: Point,
        counter_clockwise: bool,
    },
}

impl<P> Circle2D<P>
where
    P: PartialEq + PointLike<f64, 2> + Copy,
{
    pub fn new(points: &[P]) -> Self {
        println!("{:?}", points.len());
        Self::new_with_orient2d::<DefaultOrient2D>(points)
    }

    pub fn new_with_orient2d<O: Orient2D<f64, P>>(points: &[P]) -> Self {
        match points.len() {
            0 => Circle2D::None,
            1 => Circle2D::One { p: points[0] },
            2 => {
                if points[0] != points[1] {
                    Circle2D::Two {
                        a: points[0],
                        b: points[1],
                    }
                } else {
                    Circle2D::One { p: points[0] }
                }
            }
            3 => {
                let [a, b, c] = [points[0], points[1], points[2]];
                let [ab, bc, ca] = [a == b, b == c, c == a];
                match (ab, bc, ca) {
                    (true, true, true) => Circle2D::One { p: a },
                    (true, false, false) => Circle2D::Two { a, b },
                    (false, true, false) => Circle2D::Two { a, b },
                    (false, false, true) => Circle2D::Two { a: b, b: c },
                    (false, false, false) => Circle2D::Three {
                        a,
                        b,
                        c,
                        counter_clockwise: O::orient2d(a, b, c) == Orient2DState::CounterClockwise,
                    },
                    (true, true, false) | (true, false, true) | (false, true, true) => {
                        unreachable!()
                    }
                }
            }
            _ => {
                panic!()
            }
        }
    }
}

impl<P> Circle2D<P>
where
    P: PointLike<f64, 2>,
{
    pub fn surrogate(&self) -> Option<[f64; 2]> {
        match self {
            Circle2D::Two { a, b } => {
                let [a, b] = [a.coordinates(), b.coordinates()];
                let [mx, my] = [(a[0] + b[0]) / 2., (a[1] + b[1]) / 2.];
                Some([mx - my + a[1], my + mx - a[0]])
            }
            _ => None,
        }
    }

    //TODO: replace with on_circle predicate derived from incircle predicate
    /*pub fn is_spanned_by(&self, point: &P) -> bool {
        match self {
            Circle::None => false,
            Circle::One(p) => p == point,
            Circle::Two(a, b) => point == a || point == b,
            Circle::Three(a, b, c, _) => incircle(*a, *b, *c, *point) == 0.,
        }
    }*/
}

impl<P> CircumCircle<[f64; 2], f64> for Circle2D<P>
where
    P: PointLike<f64, 2>,
{
    fn circumcircle(&self) -> Option<([f64; 2], f64)> {
        match self {
            Circle2D::None => None,
            Circle2D::One { .. } => None,
            Circle2D::Two { a, b } => [a.coordinates(), b.coordinates()].circumcircle(),
            Circle2D::Three { a, b, c, .. } => {
                [a.coordinates(), b.coordinates(), c.coordinates()].circumcircle()
            }
        }
    }
}

pub trait Radius<T> {
    fn radius(&self) -> Option<T>;
}

impl<P> Radius<f64> for Circle2D<P>
where
    P: PointLike<f64, 2>,
{
    fn radius(&self) -> Option<f64> {
        self.circumcircle().map(|c| c.1)
    }
}

pub trait Center<T> {
    fn center(&self) -> Option<T>;
}

impl<P> Center<[f64; 2]> for Circle2D<P>
where
    P: PointLike<f64, 2>,
{
    fn center(&self) -> Option<[f64; 2]> {
        self.circumcircle().map(|c| c.0)
    }
}

impl<A> Circle2D<A>
where
    A: PointLike<f64, 2> + PartialEq,
{
    pub fn contains<P: PointLike<f64, 2> + PartialEq, IC: InCircle2D<f64>>(
        &self,
        point: &P,
    ) -> bool {
        match self {
            Circle2D::None => false,
            Circle2D::One { p } => p.coordinates() == point.coordinates(),
            Circle2D::Two { a, b } => {
                let s = self.surrogate().unwrap();
                let i = IC::incircle(a, b, &s, point);
                i != InCircle2DState::Outside
            }
            Circle2D::Three {
                a,
                b,
                c,
                counter_clockwise,
            } => {
                let i = IC::incircle(a, b, c, point);
                (*counter_clockwise && i == InCircle2DState::Inside)
                    || (!counter_clockwise && i == InCircle2DState::Outside)
            }
        }
    }
}
