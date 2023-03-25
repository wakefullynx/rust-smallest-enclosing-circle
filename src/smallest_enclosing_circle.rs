use geometry_predicates::{incircle, orient2d};

type Point = [f64; 2];
type CounterClockwise = bool;
type Radius = f64;

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

pub fn smallest_enclosing_circle_recursive<I: Iterator<Item = Point>>(points: I) -> Circle {
    fn recursion(p: &Vec<Point>, r: &Vec<Point>) -> Circle {
        if p.len() == 0 || r.len() == 3 {
            Circle::new(&r)
        } else {
            let remainder = &mut p.clone().to_vec();
            let element = remainder.pop().unwrap();
            let mut circle = recursion(remainder, r);
            if !is_inside_circle!(element, circle) {
                let x = &mut r.clone().to_vec();
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
    }
}
