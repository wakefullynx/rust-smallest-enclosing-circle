use geometry_predicates::{incircle, orient2d};

type Point = [f64; 2];
type CounterClockwise = bool;
type Radius = f64;

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
pub enum Circle {
    None,
    One(Point),
    Two(Point, Point),
    Three(Point, Point, Point, CounterClockwise),
}

impl Circle {
    pub fn new(points: &Vec<Point>) -> Self {
        match points.len() {
            0 => Circle::None,
            1 => Circle::One(points[0]),
            2 => {
                if points[0] != points[1] {
                    Circle::Two(points[0], points[1])
                } else {
                    Circle::One(points[0])
                }
            },
            3 => {
                let [a, b, c] = [points[0], points[1], points[2]];
                let [ab, bc, ca] = [a == b, b ==c, c ==a];
                match (ab, bc, ca) {
                    (true, true, true) => Circle::One(a),
                    (true, true, false) | (true, false, true) | (false, true, true) => unreachable!(),
                    (true, false, false) => Circle::Two(a, c),
                    (false, true, false) => Circle::Two(a, b),
                    (false, false, true) => Circle::Two(b, c),
                    (false, false, false) => Circle::Three(
                        a, b, c,
                        orient2d(a, b, c) > 0.,
                    ),
                }
            },
            _ => {
                panic!()
            }
        }
    }

    pub fn radius(&self) -> Radius {
        match self {
            Circle::None => 0.,
            Circle::One(_) => 0.,
            Circle::Two(a, b) => f64::hypot(a[0] - b[0], a[1] - b[1]) / 2.,
            &Circle::Three(a, b, c, _) => circumcircle(a, b, c).1,
        }
    }

    pub fn center(&self) -> Option<Point> {
        match self {
            Circle::None => None,
            &Circle::One(a) => Some(a),
            Circle::Two(a, b) => Some([(a[0] + b[0]) / 2., (a[1] + b[1]) / 2.]),
            &Circle::Three(a, b, c, _) => Some(circumcircle(a, b, c).0),
        }
    }

    fn surrogate(&self) -> Option<Point> {
        match self {
            Circle::None | Circle::One(_) | Circle::Three(_, _, _, _) => None,
            Circle::Two(a, b) => {
                let [mx, my] = [(a[0] + b[0]) / 2., (a[1] + b[1]) / 2.];
                Some([mx - my + a[1], my + mx - a[0]])
            }
        }
    }
}

macro_rules! is_inside_circle {
    ($point: ident, $circle: ident) => {{
        match $circle {
            Circle::None => false,
            Circle::One(a) => a == $point,
            Circle::Two(a, b) => {
                let s = $circle.surrogate().unwrap();
                incircle(a, b, s, $point) > 0.
            }
            Circle::Three(a, b, c, counter_clockwise) => {
                (counter_clockwise && incircle(a, b, c, $point) > 0.)
                    || (!counter_clockwise && incircle(a, c, b, $point) > 0.)
            }
        }
    }};
}

#[derive(Debug)]
enum State {
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
pub fn smallest_enclosing_circle<I: Iterator<Item = Point>>(points: I) -> Circle {
    let mut p: Vec<Point> = points.collect();
    let mut r = Vec::new();
    let mut circle = Circle::None;
    let mut stack = Vec::from([State::S0]);
    while !stack.is_empty() {
        let state = stack.pop().unwrap();
        match state {
            State::S0 => {
                if p.len() == 0 || r.len() == 3 {
                    circle = Circle::new(&r);
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

                if !is_inside_circle!(element, circle) {
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
pub fn smallest_enclosing_circle_recursive<I: Iterator<Item = Point>>(points: I) -> Circle {
    fn recursion(p: &Vec<Point>, r: &Vec<Point>) -> Circle {
        if p.len() == 0 || r.len() == 3 {
            Circle::new(&r)
        } else {
            let remainder = &mut p.to_vec();
            let element = remainder.pop().unwrap();
            let mut circle = recursion(remainder, r);
            if !is_inside_circle!(element, circle) {
                let x = &mut r.to_vec();
                x.push(element);
                circle = recursion(remainder, x);
            }
            circle
        }
    }

    recursion(&points.collect(), &Vec::new())
}

fn circumcircle(a: Point, b: Point, c: Point) -> (Point, Radius) {
    let orientation = orient2d(a, b, c);

    let (b, c, denominator) = if orientation > 0. {
        (b, c, 2. * orientation)
    } else if orientation < 0. {
        (c, b, -2. * orientation)
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
    let radius = f64::sqrt(bcxys * acxys * abxys) / denominator;
    (center, radius)
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
                    Circle::Two([0., 0.], [1., 1.]).surrogate().unwrap(),
                    [0., 1.]
                )
            }

            #[test]
            fn two_points_reverse() {
                assert_eq!(
                    Circle::Two([1., 1.], [0., 0.]).surrogate().unwrap(),
                    [1., 0.]
                )
            }
        }
    }

    mod circumcircle {
        use super::*;

        #[test]
        fn box_triangle_lower_right() {
            assert_eq!(
                circumcircle([-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]),
                ([0., 0.], f64::sqrt(2.))
            )
        }
    }

    mod iterative {
        use super::*;

        #[test]
        fn basic_collinear() {
            let r = smallest_enclosing_circle(Vec::from([[0., 0.], [1., 0.], [2., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([2., 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_duplicate() {
            let r = smallest_enclosing_circle(Vec::from([[0., 0.], [1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([1., 0.], [0., 0.]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), 0.5);
        }

        #[test]
        fn basic_duplicate2() {
            let r = smallest_enclosing_circle(Vec::from([[1., 0.], [0., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([0., 0.], [1., 0.]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), 0.5);
        }

        #[test]
        fn basic_empty() {
            let r = smallest_enclosing_circle(Vec::from([]).into_iter());
            assert_eq!(r, Circle::None);
            assert_eq!(r.center(), None);
            assert_eq!(r.radius(), 0.);
        }

        #[test]
        fn basic_single() {
            let r = smallest_enclosing_circle(Vec::from([[0., 0.]]).into_iter());
            assert_eq!(r, Circle::One([0., 0.]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), 0.);
        }

        #[test]
        fn basic_double() {
            let r = smallest_enclosing_circle(Vec::from([[0., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([1.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), 0.5);
        }

        #[test]
        fn basic_double_duplicate() {
            let r = smallest_enclosing_circle(Vec::from([[1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::One([1.0, 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 0.);
        }

        #[test]
        fn basic_opposite_zero() {
            let r = smallest_enclosing_circle(Vec::from([[-1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([1.0, 0.], [-1., 0.]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_small() {
            let r = smallest_enclosing_circle(Vec::from([
                [0., 0.],
                [1e-12, 0.],
                [0.5, 0.],
                [1., 0.],
                [1.1, 0.],
                [1.5, 0.],
                [2. - 1e-12, 0.],
                [2., 0.],
            ]).into_iter());
            assert_eq!(r, Circle::Two([2.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_small2() {
            let r = smallest_enclosing_circle(Vec::from([
                [1e-12, 0.],
                [0.5, 0.],
                [1., 0.],
                [1.1, 0.],
                [1.5, 0.],
                [0., 0.],
                [2. - 1e-12, 0.],
                [2., 0.],
            ]).into_iter());
            assert_eq!(r, Circle::Two([2.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_small3() {
            let r = smallest_enclosing_circle(Vec::from([
                [0., 0.],
                [1e-12, 0.],
                [0.5, 0.],
                [1., 0.],
                [1.1, 0.],
                [1.5, 0.],
                [2. - 1e-12, 0.],
                [2., 0.],
            ]).into_iter());
            assert_eq!(r, Circle::Two([2.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_cocircular() {
            let r =
                smallest_enclosing_circle(Vec::from([[1., 0.], [0., 1.], [-1., 0.], [0., -1.]]).into_iter());
            assert_eq!(r, Circle::Three([0., -1.], [-1., 0.], [0., 1.], false));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), 1.);
        }
    }

    mod recursive {
        use super::*;

        #[test]
        fn basic_collinear() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[0., 0.], [1., 0.], [2., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([2., 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_duplicate() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[0., 0.], [1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([1., 0.], [0., 0.]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), 0.5);
        }

        #[test]
        fn basic_duplicate2() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[1., 0.], [0., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([0., 0.], [1., 0.]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), 0.5);
        }

        #[test]
        fn basic_empty() {
            let r = smallest_enclosing_circle_recursive(Vec::from([]).into_iter());
            assert_eq!(r, Circle::None);
            assert_eq!(r.center(), None);
            assert_eq!(r.radius(), 0.);
        }

        #[test]
        fn basic_single() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[0., 0.]]).into_iter());
            assert_eq!(r, Circle::One([0., 0.]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), 0.);
        }

        #[test]
        fn basic_double() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[0., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([1.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([0.5, 0.]));
            assert_eq!(r.radius(), 0.5);
        }

        #[test]
        fn basic_double_duplicate() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::One([1.0, 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 0.);
        }

        #[test]
        fn basic_opposite_zero() {
            let r = smallest_enclosing_circle_recursive(Vec::from([[-1., 0.], [1., 0.]]).into_iter());
            assert_eq!(r, Circle::Two([1.0, 0.], [-1., 0.]));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_small() {
            let r = smallest_enclosing_circle_recursive(Vec::from([
                [0., 0.],
                [1e-12, 0.],
                [0.5, 0.],
                [1., 0.],
                [1.1, 0.],
                [1.5, 0.],
                [2. - 1e-12, 0.],
                [2., 0.],
            ]).into_iter());
            assert_eq!(r, Circle::Two([2.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_small2() {
            let r = smallest_enclosing_circle_recursive(Vec::from([
                [1e-12, 0.],
                [0.5, 0.],
                [1., 0.],
                [1.1, 0.],
                [1.5, 0.],
                [0., 0.],
                [2. - 1e-12, 0.],
                [2., 0.],
            ]).into_iter());
            assert_eq!(r, Circle::Two([2.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_small3() {
            let r = smallest_enclosing_circle_recursive(Vec::from([
                [0., 0.],
                [1e-12, 0.],
                [0.5, 0.],
                [1., 0.],
                [1.1, 0.],
                [1.5, 0.],
                [2. - 1e-12, 0.],
                [2., 0.],
            ]).into_iter());
            assert_eq!(r, Circle::Two([2.0, 0.], [0., 0.]));
            assert_eq!(r.center(), Some([1., 0.]));
            assert_eq!(r.radius(), 1.);
        }

        #[test]
        fn basic_cocircular() {
            let r =
                smallest_enclosing_circle_recursive(Vec::from([[1., 0.], [0., 1.], [-1., 0.], [0., -1.]]).into_iter());
            assert_eq!(r, Circle::Three([0., -1.], [-1., 0.], [0., 1.], false));
            assert_eq!(r.center(), Some([0., 0.]));
            assert_eq!(r.radius(), 1.);
        }
    }
}
