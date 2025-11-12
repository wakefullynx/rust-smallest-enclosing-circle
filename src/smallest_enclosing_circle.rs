use crate::{
    circle::Circle2D,
    incircle::{DefaultInCircle2D, InCircle2D},
    point::PointLike,
};

#[derive(Debug)]
enum State<Point> {
    S0,
    S1,
    S2(Point),
    S3(Point),
    S4,
}

/// Takes an iterator over two-dimensional points and returns the smallest [Circle] that encloses all points.
///
/// Iterative version of Welzl's algorithm, which originally is [recursive](smallest_enclosing_circle_recursive).
/// The expected input is an iterator of [f64; 2] coordinate pairs with actual numbers (no NANs or Infinites). Duplicates are allowed.
/// Note that the original algorithm is based on randomizing the order of input points.
/// This is omitted in this crate, however randomization can be done by the caller in advance.
/// The advantage over the recursive algorithm is that large problems sizes do not run into call stack problems.
/// The result is a [Circle] struct, i.e., either one of four options:
/// - None (i.e., empty input vector)
/// - Smallest circle is spanned by one point and zero radius (iff input vector has length 1)
/// - Smallest circle is spanned by two points, with the center halfway between them and radius half their distance
/// - Smallest circle is spanned by three points, with the center halfway between them and radius half their distance
///
/// In cases where more than three points lie on the smallest circle, the choice of spanning points is arbitrary. In the same way, the order of spanning points is arbitrary for all [Circle] instances.
///
/// The implementation is based on the following work:
///
/// Welzl, E. (1991). Smallest enclosing disks (balls and ellipsoids).
/// In New results and new trends in computer science (pp. 359-370).
/// Springer, Berlin, Heidelberg.
///
///
/// # Examples
///
/// ```
/// use smallest_enclosing_circle::smallest_enclosing_circle;
///
/// // Input: Four corner points of square box of unit size
/// let points = Vec::from([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
/// let circle = smallest_enclosing_circle(points.into_iter());
/// println!("Circle: {:?}", circle);
/// // Circle: Three([0.0, 1.0], [1.0, 1.0], [1.0, 0.0], false);
/// println!("Center: {:?}", circle.center());
/// // Center: Some([0.5, 0.5])
/// println!("Radius: {:?}", circle.radius());
/// // Radius: 0.7071067811865476
/// ```
pub fn smallest_enclosing_circle_with_predicate<Point, InCirclePredicate>(
    points: impl Iterator<Item = Point>,
) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
    InCirclePredicate: InCircle2D<f64>,
{
    let mut p: Vec<Point> = points.collect();
    let mut r = Vec::new();
    let mut circle = Circle2D::None;
    let mut stack = Vec::from([State::S0]);
    while !stack.is_empty() {
        let state = stack.pop().unwrap();
        match state {
            State::S0 => {
                if p.len() == 0 || r.len() == 3 {
                    circle = Circle2D::new(&r);
                } else {
                    stack.push(State::S1);
                }
            }
            State::S1 => {
                let element = p.pop().unwrap();
                stack.push(State::S2(element));
                stack.push(State::S0);
            }
            State::S2(element) => {
                stack.push(State::S3(element));

                if !circle.contains::<Point, InCirclePredicate>(&element) {
                    r.push(element);
                    stack.push(State::S4);
                    stack.push(State::S0);
                }
            }
            State::S3(element) => {
                p.push(element);
            }
            State::S4 => {
                r.pop();
            }
        }
    }
    circle
}

pub fn smallest_enclosing_circle<Point>(points: impl Iterator<Item = Point>) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
{
    smallest_enclosing_circle_with_predicate::<Point, DefaultInCircle2D>(points)
}

/// Recursive version of [smallest_enclosing_circle] with identical functionality for demonstration purposes only. Use the iterative version.
///
/// Implementation of Welzl's algorithm. The expected input is an iterator of [f64; 2] coordinate pairs with actual numbers (no NANs or Infinites). Duplicates are allowed. Note that the original algorithm is based on randomizing the order of input points. This is omitted in this crate, however randomization can be done by the caller in advance.
/// Since the implementation makes recursive calls, for larger problems the call stack will be exceeded. Thus, you should use [smallest_enclosing_circle].
/// The API behaves the same as well.
///
/// The implementation is based on the following work:
/// Welzl, E. (1991). Smallest enclosing disks (balls and ellipsoids).\n
/// In New results and new trends in computer science (pp. 359-370).
/// Springer, Berlin, Heidelberg.
///
///
/// # Examples
///
/// ```
/// use smallest_enclosing_circle::smallest_enclosing_circle_recursive;
///
/// // Input: Four corner points of square box of unit size
/// let points = Vec::from([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
/// let circle = smallest_enclosing_circle_recursive(points.into_iter());
/// println!("Circle: {:?}", circle);
/// // Circle: Three([0.0, 1.0], [1.0, 1.0], [1.0, 0.0], false);
/// println!("Center: {:?}", circle.center());
/// // Center: Some([0.5, 0.5])
/// println!("Radius: {:?}", circle.radius());
/// // Radius: 0.7071067811865476
/// ```
pub fn smallest_enclosing_circle_recursive_with_predicate<Point, InCirclePredicate>(
    points: impl Iterator<Item = Point>,
) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
    InCirclePredicate: InCircle2D<f64>,
{
    fn recursion<Point, InCirclePredicate>(p: &Vec<Point>, r: &Vec<Point>) -> Circle2D<Point>
    where
        Point: PartialEq + PointLike<f64, 2> + Copy,
        InCirclePredicate: InCircle2D<f64>,
    {
        if p.len() == 0 || r.len() == 3 {
            Circle2D::new(&r)
        } else {
            let remainder = &mut p.to_vec();
            let element = remainder.pop().unwrap();
            let mut circle = recursion::<Point, InCirclePredicate>(remainder, r);
            if !circle.contains::<Point, InCirclePredicate>(&element) {
                let x = &mut r.to_vec();
                x.push(element);
                circle = recursion::<Point, InCirclePredicate>(remainder, x);
            }
            circle
        }
    }

    recursion::<Point, InCirclePredicate>(&points.collect(), &Vec::new())
}

pub fn smallest_enclosing_circle_recursive<Point>(
    points: impl Iterator<Item = Point>,
) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
{
    smallest_enclosing_circle_recursive_with_predicate::<Point, DefaultInCircle2D>(points)
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

    mod circumcircle {
        use crate::{circumcircle::circumcircle2d, orient2d::DefaultOrient2DArea};

        #[test]
        fn box_triangle_lower_right() {
            assert_eq!(
                circumcircle2d::<f64, DefaultOrient2DArea>([-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]),
                ([0., 0.], f64::sqrt(2.))
            )
        }
    }

    mod iterative {
        use crate::circle::{Center, Radius};

        use super::*;

        #[test]
        fn basic_collinear() {
            let r =
                smallest_enclosing_circle(Vec::from([[0., 0.], [1., 0.], [2., 0.]]).into_iter());
            assert_eq!(
                r,
                Circle2D::Two {
                    a: [2., 0.],
                    b: [0., 0.]
                }
            );
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_duplicate() {
            let r =
                smallest_enclosing_circle(Vec::from([[0., 0.], [1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[1., 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), Some(0.5));
        }

        #[test]
        fn basic_duplicate2() {
            let r =
                smallest_enclosing_circle(Vec::from([[1., 0.], [0., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[0., 0.], [1., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), Some(0.5));
        }

        #[test]
        fn basic_empty() {
            let r = smallest_enclosing_circle::<[f64; 2]>(Vec::from([]).into_iter());
            assert_eq!(r, Circle2D::new(&[]));
            assert_eq!(r.center(), None);
            assert_eq!(r.radius(), None);
        }

        #[test]
        fn basic_single() {
            let r = smallest_enclosing_circle(Vec::from([[0., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[0., 0.]]));
            assert_eq!(r.center(), None);
            assert_eq!(r.radius(), None);
        }

        #[test]
        fn basic_double() {
            let r = smallest_enclosing_circle(Vec::from([[0., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[1.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), Some(0.5));
        }

        #[test]
        fn basic_double_duplicate() {
            let r = smallest_enclosing_circle(Vec::from([[1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[1.0, 0.]]));
            assert_eq!(r.center(), None);
            assert_eq!(r.radius(), None);
        }

        #[test]
        fn basic_opposite_zero() {
            let r = smallest_enclosing_circle(Vec::from([[-1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[1.0, 0.], [-1., 0.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_small() {
            let r = smallest_enclosing_circle(
                Vec::from([
                    [0., 0.],
                    [1e-12, 0.],
                    [0.5, 0.],
                    [1., 0.],
                    [1.1, 0.],
                    [1.5, 0.],
                    [2. - 1e-12, 0.],
                    [2., 0.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[2.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_small2() {
            let r = smallest_enclosing_circle(
                Vec::from([
                    [1e-12, 0.],
                    [0.5, 0.],
                    [1., 0.],
                    [1.1, 0.],
                    [1.5, 0.],
                    [0., 0.],
                    [2. - 1e-12, 0.],
                    [2., 0.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[2.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_small3() {
            let r = smallest_enclosing_circle(
                Vec::from([
                    [0., 0.],
                    [1e-12, 0.],
                    [0.5, 0.],
                    [1., 0.],
                    [1.1, 0.],
                    [1.5, 0.],
                    [2. - 1e-12, 0.],
                    [2., 0.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[2.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_cocircular() {
            let r = smallest_enclosing_circle(
                Vec::from([[1., 0.], [0., 1.], [-1., 0.], [0., -1.]]).into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[-1., 0.], [1., 0.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_multiple() {
            let r = smallest_enclosing_circle(
                Vec::from([
                    [-1., -1.],
                    [-1., -1.],
                    [-1., -1.],
                    [-1., -1.],
                    [0., 0.],
                    [0., 0.],
                    [0., 0.],
                    [0., 0.],
                    [0., 0.],
                    [1., 1.],
                    [1., 1.],
                    [1., 1.],
                    [1., 1.],
                    [-1., -1.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[1., 1.], [-1., -1.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(f64::sqrt(2.)));
        }

        #[test]
        fn basic_multiple2() {
            let r = smallest_enclosing_circle(
                Vec::from([
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[1., 1.], [-1., -1.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(f64::sqrt(2.)));
        }

        #[test]
        fn basic_triangle() {
            let r = smallest_enclosing_circle(
                Vec::from([[0., 0.], [1., 0.], [1., 1.], [1., 1.]]).into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[1., 1.], [0., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.5]));
            assert_eq!(r.radius(), Some(f64::sqrt(2.) / 2.));
        }
    }

    
    mod recursive {
        use crate::circle::{Center, Radius};

        use super::*;

        #[test]
        fn basic_collinear() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([[0., 0.], [1., 0.], [2., 0.]]).into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[2., 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_duplicate() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([[0., 0.], [1., 0.], [1., 0.]]).into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[1., 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), Some(0.5));
        }

        #[test]
        fn basic_duplicate2() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([[1., 0.], [0., 0.], [1., 0.]]).into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[0., 0.], [1., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), Some(0.5));
        }

        #[test]
        fn basic_empty() {
            let r = smallest_enclosing_circle_recursive::<[f64; 2]>(Vec::from([]).into_iter());
            assert_eq!(r, Circle2D::new(&[]));
            assert_eq!(r.center(), None);
            assert_eq!(r.radius(), None);
        }

        #[test]
        fn basic_single() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[0., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[0., 0.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(0.));
        }

        #[test]
        fn basic_double() {
            let r =
                smallest_enclosing_circle_recursive(Vec::from([[0., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[1.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), Some(0.5));
        }

        #[test]
        fn basic_double_duplicate() {
            let r =
                smallest_enclosing_circle_recursive(Vec::from([[1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[1.0, 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(0.));
        }

        #[test]
        fn basic_opposite_zero() {
            let r =
                smallest_enclosing_circle_recursive(Vec::from([[-1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle2D::new(&[[1.0, 0.], [-1., 0.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_small() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([
                    [0., 0.],
                    [1e-12, 0.],
                    [0.5, 0.],
                    [1., 0.],
                    [1.1, 0.],
                    [1.5, 0.],
                    [2. - 1e-12, 0.],
                    [2., 0.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[2.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_small2() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([
                    [1e-12, 0.],
                    [0.5, 0.],
                    [1., 0.],
                    [1.1, 0.],
                    [1.5, 0.],
                    [0., 0.],
                    [2. - 1e-12, 0.],
                    [2., 0.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[2.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_small3() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([
                    [0., 0.],
                    [1e-12, 0.],
                    [0.5, 0.],
                    [1., 0.],
                    [1.1, 0.],
                    [1.5, 0.],
                    [2. - 1e-12, 0.],
                    [2., 0.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[2.0, 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_cocircular() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([[1., 0.], [0., 1.], [-1., 0.], [0., -1.]]).into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[-1., 0.], [0., 1.], [1., 0.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(1.));
        }

        #[test]
        fn basic_multiple() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([
                    [-1., -1.],
                    [-1., -1.],
                    [-1., -1.],
                    [-1., -1.],
                    [0., 0.],
                    [0., 0.],
                    [0., 0.],
                    [0., 0.],
                    [0., 0.],
                    [1., 1.],
                    [1., 1.],
                    [1., 1.],
                    [1., 1.],
                    [-1., -1.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[-1., -1.], [1., 1.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(f64::sqrt(2.)));
        }

        #[test]
        fn basic_multiple2() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                    [-1., -1.],
                    [0., 0.],
                    [1., 1.],
                ])
                .into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[-1., -1.], [1., 1.]]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), Some(f64::sqrt(2.)));
        }

        #[test]
        fn basic_triangle() {
            let r = smallest_enclosing_circle_recursive(
                Vec::from([[0., 0.], [1., 0.], [1., 1.], [1., 1.]]).into_iter(),
            );
            assert_eq!(r, Circle2D::new(&[[1., 1.], [1., 0.], [0., 0.]]));
            assert_eq!(r.center(), Some([0.5, 0.5]));
            assert_eq!(r.radius(), Some(f64::sqrt(2.) / 2.));
        }
    }
}
