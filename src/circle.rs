use std::fmt::Debug;

use crate::{
    geometry::{circumcircle::CircumCircle, point::PointLike},
    predicates::{
        in_circle::{DefaultInCircle, InCircle, InCircleState},
        orientation::{DefaultOrientation, Orientation, OrientationState},
    },
};

/// Represents the result of the main algorithms, a circle defined by up to three points that are located on its circumference (points that *span* the circle).
///
/// This enum has four variants:
/// - [`Circle2D::None`]: No points, and thus, no circle (no center, no radius, appears for degenerate problems only).
/// - [`Circle2D::One`]: Single point, still no circle (no center, no radius, used internally, appears for degenerate problems only).
/// - [`Circle2D::Two`]: Circle is defined by two points (i.e., the points are located opposite each other on the circle), and the radius is their half-distance.
/// - [`Circle2D::Three`]: Circle that intersects the three given points. The three points must be in the given order (clockwise or counterclockwise).
///
/// However, for non-degenerate problems, i.e., any problemset with more than two distinct points, you will encounter only the [`Circle2D::Two`] and [`Circle2D::Three`] variants. Complementary methods are provided to compute the center and radius.
///
/// ```
/// use smallest_enclosing_circle::{Circle2D};
///
/// let circle = Circle2D::new(&[[0., 0.], [1., 0.]]);
/// 
/// assert_eq!(circle.center(), Some([0.5, 0.0]));
/// assert_eq!(circle.radius(), Some(0.5));
/// 
/// assert_eq!(circle.contains(&[0.5, 0.]), true);
/// assert_eq!(circle.contains(&[1.0, 0.]), true);
/// 
/// assert_eq!(circle.is_on_circle(&[0.5, 0.]), false);
/// assert_eq!(circle.is_on_circle(&[1.0, 0.]), true);
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
    /// Creates a new [`Circle2D`] spanned by 0 to 3 points.
    /// 
    /// # Panics
    ///
    /// Panics if more than 3 points are supplied.
    pub fn new(points: &[P]) -> Self {
        Self::new_with_predicate::<DefaultOrientation>(points)
    }

    /// Creates a new [`Circle2D`] spanned by 0 to 3 points. If 3 points are supplied, uses a custom [`Orientation`] predicate to determine whether they are in clockwise or counterclockwise order.
    /// 
    /// # Panics
    ///
    /// Panics if more than 3 points are supplied.
    pub fn new_with_predicate<O: Orientation<f64>>(points: &[P]) -> Self {
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
                        counter_clockwise: O::orientation(&a, &b, &c)
                            == OrientationState::CounterClockwise,
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
    /// For a [`Circle2D`] spanned by 2 points, computes a third (surrogate) point that is used for [`InCircle`] checks. Otherwise `None`.
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


impl<P> Circle2D<P>
where
    P: PointLike<f64, 2>,
{
    /// Computes the radius of the circle. `None` for degenerate circles spanned by 0 or 1 points. This procedure is not numerically robust.
    pub fn radius(&self) -> Option<f64> {
        self.circumcircle().map(|c| c.1)
    }
}

impl<P> Circle2D<P>
where
    P: PointLike<f64, 2>,
{
    /// Computes the center of the circle. `None` for degenerate circles spanned by 0 or 1 points. This procedure is not numerically robust.
    pub fn center(&self) -> Option<[f64; 2]> {
        self.circumcircle().map(|c| c.0)
    }
}

impl<P> Circle2D<P>
where
    P: PointLike<f64, 2>,
{
    /// Tests whether the given point lies exactly *on* the circle.
    pub fn is_on_circle(&self, point: &impl PointLike<f64, 2>) -> bool {
        self.is_on_circle_with_predicate::<DefaultInCircle>(point)
    }

    /// Tests whether the given point lies exactly *on* the circle. Uses the custom [`InCircle`] predicate to determine the location.
    pub fn is_on_circle_with_predicate<IC: InCircle<f64>>(
        &self,
        point: &impl PointLike<f64, 2>,
    ) -> bool {
        match self {
            Circle2D::None => false,
            Circle2D::One { p } => p.coordinates() == point.coordinates(),
            Circle2D::Two { a, b } => {
                let s = self.surrogate().unwrap();
                let i = IC::in_circle(a, b, &s, point);
                i == InCircleState::On
            }
            Circle2D::Three { a, b, c, .. } => {
                let i = IC::in_circle(a, b, c, point);
                i == InCircleState::On
            }
        }
    }

    /// Checks for equivalence between two circles in the graphical sense. Two circles are equal iff every spanning point of the other circle is located exactly *on* this circle and vice-versa.
    pub fn equals(&self, other: &Circle2D<impl PointLike<f64, 2>>) -> bool {
        self.equals_with_predicate::<DefaultInCircle>(other)
    }

    /// Checks for equivalence between two circles in the graphical sense. Two circles are equal iff every spanning point of the other circle is located exactly *on* this circle and vice-versa. Uses the custom [`InCircle`] predicate to determine locations.
    pub fn equals_with_predicate<IC: InCircle<f64>>(
        &self,
        other: &Circle2D<impl PointLike<f64, 2>>,
    ) -> bool {
        self.one_sided_equals_with_predicate::<IC>(other) && other.one_sided_equals_with_predicate::<IC>(self)
    }

    fn one_sided_equals_with_predicate<IC: InCircle<f64>>(
        &self,
        other: &Circle2D<impl PointLike<f64, 2>>,
    ) -> bool {
        match self {
            Circle2D::None => match other {
                Circle2D::None => true,
                _ => false
            },
            Circle2D::One { p: p1 } => match other {
                Circle2D::One { p: p2 } => p1.coordinates() == p2.coordinates(),
                _ => false,
            },
            Circle2D::Two { .. } => match other {
                Circle2D::Two { a, b } => {
                    self.is_on_circle_with_predicate::<IC>(a)
                        && self.is_on_circle_with_predicate::<IC>(b)
                }
                Circle2D::Three { a, b, c, .. } => {
                    self.is_on_circle_with_predicate::<IC>(a)
                        && self.is_on_circle_with_predicate::<IC>(b)
                        && self.is_on_circle_with_predicate::<IC>(c)
                }
                _ => false,
            },
            Circle2D::Three { .. } => match other {
                Circle2D::Two { a, b } => {
                    self.is_on_circle_with_predicate::<IC>(a)
                        && self.is_on_circle_with_predicate::<IC>(b)
                }
                Circle2D::Three { a, b, c, .. } => {
                    self.is_on_circle_with_predicate::<IC>(a)
                        && self.is_on_circle_with_predicate::<IC>(b)
                        && self.is_on_circle_with_predicate::<IC>(c)
                }
                _ => false,
            },
        }
    }
}

impl<A> Circle2D<A>
where
    A: PointLike<f64, 2> + PartialEq,
{
    /// Checks whether the given point is contained by the circle, i.e., whether it lies on *or* inside the circle.
    pub fn contains<P: PointLike<f64, 2> + PartialEq>(&self, point: &P) -> bool {
        self.contains_with_predicate::<P, DefaultInCircle>(point)
    }

    /// Checks whether the given point is contained by the circle, i.e., whether it lies on *or* inside the circle. Uses the custom [`InCircle`] predicate to determine locations.
    pub fn contains_with_predicate<P: PointLike<f64, 2> + PartialEq, IC: InCircle<f64>>(&self, point: &P) -> bool {
        match self {
            Circle2D::None => false,
            Circle2D::One { p } => p.coordinates() == point.coordinates(),
            Circle2D::Two { a, b } => {
                let s = self.surrogate().unwrap();
                let i = IC::in_circle(a, b, &s, point);
                i != InCircleState::Outside
            }
            Circle2D::Three {
                a,
                b,
                c,
                counter_clockwise,
            } => {
                let i = IC::in_circle(a, b, c, point);
                (*counter_clockwise && i == InCircleState::Inside)
                    || (!counter_clockwise && i == InCircleState::Outside)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod circle {
        use super::*;
        mod surrogate {
            use super::*;

            #[test]
            fn two_points() {
                assert_eq!(
                    Circle2D::new(&[[0., 0.], [1., 1.]]).surrogate().unwrap(),
                    [0., 1.]
                )
            }

            #[test]
            fn two_points_reverse() {
                assert_eq!(
                    Circle2D::new(&[[1., 1.], [0., 0.]]).surrogate().unwrap(),
                    [1., 0.]
                )
            }
        }
    }

}
