use crate::{
    circle::Circle2D,
    geometry::point::PointLike,
    predicates::in_circle::{DefaultInCircle, InCircle},
};

enum State<Point> {
    S0,
    S1,
    S2(Point),
    S3(Point),
    S4,
}

/// See [`smallest_enclosing_circle`]. Additionally, supports a custom [`InCircle`] predicate.
///
/// # Examples
///
/// ```
/// use smallest_enclosing_circle::smallest_enclosing_circle_with_predicate;
/// use smallest_enclosing_circle::predicates::in_circle::DefaultInCircle;
///
/// // Input: Four corner points of square box of unit size
/// let circle = smallest_enclosing_circle_with_predicate::<_, DefaultInCircle>([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
/// assert_eq!(circle.center(), Some([0.5, 0.5]));
/// assert_eq!(circle.radius(), Some(f64::sqrt(2.) / 2.));
/// ```
pub fn smallest_enclosing_circle_with_predicate<Point, InCirclePredicate>(
    points: impl IntoIterator<Item = Point>,
) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
    InCirclePredicate: InCircle<f64>,
{
    let mut p: Vec<Point> = points.into_iter().collect();
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

                if !circle.contains_with_predicate::<Point, InCirclePredicate>(&element) {
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

/// Takes an iterator over two-dimensional points and returns the smallest [Circle] that encloses all points.
///
/// Iterative version of Welzl's algorithm, which was originally formulated as recursive algorithm.
/// The expected input is an of [f64; 2] coordinate pairs with actual numbers (no NaNs or Infinites). Duplicates are allowed.
/// Note that the original algorithm is based on randomizing the order of input points.
/// This is omitted in this crate, however randomization can be done by the caller in advance.
/// The advantage over the recursive algorithm is that large problem sizes do not run into call stack problems.
/// The result is a [`Circle2D`] enum.
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
/// use smallest_enclosing_circle::predicates::in_circle::DefaultInCircle;
///
/// // Input: Four corner points of square box of unit size
/// let circle = smallest_enclosing_circle([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
/// assert_eq!(circle.center(), Some([0.5, 0.5]));
/// assert_eq!(circle.radius(), Some(f64::sqrt(2.) / 2.));
/// ```
pub fn smallest_enclosing_circle<Point>(points: impl IntoIterator<Item = Point>) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
{
    smallest_enclosing_circle_with_predicate::<Point, DefaultInCircle>(points)
}

/// See [`smallest_enclosing_circle_with_predicate`]. This is the recursive version, implemented for educational purposes only. You should prefer [`smallest_enclosing_circle_with_predicate`].
///
/// # Examples
///
/// ```
/// use smallest_enclosing_circle::algorithm::smallest_enclosing_circle_recursive_with_predicate;
/// use smallest_enclosing_circle::predicates::in_circle::DefaultInCircle;
///
/// // Input: Four corner points of square box of unit size
/// let circle = smallest_enclosing_circle_recursive_with_predicate::<_, DefaultInCircle>([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
/// assert_eq!(circle.center(), Some([0.5, 0.5]));
/// assert_eq!(circle.radius(), Some(f64::sqrt(2.) / 2.));
/// ```
pub fn smallest_enclosing_circle_recursive_with_predicate<Point, InCirclePredicate>(
    points: impl IntoIterator<Item = Point>,
) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
    InCirclePredicate: InCircle<f64>,
{
    fn recursion<Point, InCirclePredicate>(p: &Vec<Point>, r: &Vec<Point>) -> Circle2D<Point>
    where
        Point: PartialEq + PointLike<f64, 2> + Copy,
        InCirclePredicate: InCircle<f64>,
    {
        if p.len() == 0 || r.len() == 3 {
            Circle2D::new(&r)
        } else {
            let remainder = &mut p.to_vec();
            let element = remainder.pop().unwrap();
            let mut circle = recursion::<Point, InCirclePredicate>(remainder, r);
            if !circle.contains_with_predicate::<Point, InCirclePredicate>(&element) {
                let x = &mut r.to_vec();
                x.push(element);
                circle = recursion::<Point, InCirclePredicate>(remainder, x);
            }
            circle
        }
    }

    recursion::<Point, InCirclePredicate>(&points.into_iter().collect(), &Vec::new())
}

/// See [`smallest_enclosing_circle`]. This is the recursive version, implemented for educational purposes only. You should prefer [`smallest_enclosing_circle`].
///
/// # Examples
///
/// ```
/// use smallest_enclosing_circle::algorithm::smallest_enclosing_circle_recursive;
/// use smallest_enclosing_circle::predicates::in_circle::DefaultInCircle;
///
/// // Input: Four corner points of square box of unit size
/// let circle = smallest_enclosing_circle_recursive([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
/// assert_eq!(circle.center(), Some([0.5, 0.5]));
/// assert_eq!(circle.radius(), Some(f64::sqrt(2.) / 2.));
/// ```
pub fn smallest_enclosing_circle_recursive<Point>(
    points: impl IntoIterator<Item = Point>,
) -> Circle2D<Point>
where
    Point: PartialEq + PointLike<f64, 2> + Copy,
{
    smallest_enclosing_circle_recursive_with_predicate::<Point, DefaultInCircle>(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::Itertools;
    use paste::paste;

    macro_rules! assert_equals_circles {
        ($circle1: expr, $circle2: expr) => {
            $circle1.equals($circle2)
        };
    }

    macro_rules! test_case {
        ($name: ident, $function: ident, $points: expr, $expected_circle_points: expr) => {
            paste! {
                #[test]
                fn [<test_$name>]() {
                    let points: Vec<[f64; 2]> = $points.to_vec();
                    let n = points.into_iter().count();
                    Itertools::permutations($points.into_iter(), n).for_each(|permutation: Vec<[f64; 2]>| {
                        let result = [<$function>]::<[f64; 2]>(
                            permutation,
                        );
                        assert_equals_circles!(result, &Circle2D::<[f64; 2]>::new(&$expected_circle_points));
                    });
                }
            }
        };
    }

    macro_rules! test_function {
        ($function: ident) => {
            paste! {
                mod [<test_$function>] {
                    use super::*;

                    test_case!(
                        collinear,
                        $function,
                        [[0., 0.], [1., 0.], [2., 0.]],
                        [[2., 0.], [0., 0.]]
                    );

                    test_case!(
                        duplicate,
                        $function,
                        [[0., 0.], [1., 0.], [1., 0.]],
                        [[1., 0.], [0., 0.]]
                    );

                    test_case!(
                        duplicate2,
                        $function,
                        [[1., 0.], [0., 0.], [1., 0.]],
                        [[0., 0.], [1., 0.]]
                    );

                    test_case!(
                        empty,
                        $function,
                        [],
                        []
                    );

                    test_case!(
                        single,
                        $function,
                        [[0., 0.]],
                        [[0., 0.]]
                    );

                     test_case!(
                        double,
                        $function,
                        [[0., 0.], [1., 0.]],
                        [[1.0, 0.], [0., 0.]]
                    );

                    test_case!(
                        double_duplicate,
                        $function,
                        [[1., 0.], [1., 0.]],
                        [[1.0, 0.]]
                    );

                    test_case!(
                        opposite_zero,
                        $function,
                        [[-1., 0.], [1., 0.]],
                        [[1.0, 0.], [-1., 0.]]
                    );

                    test_case!(
                        small,
                        $function,
                        [
                            [0., 0.],
                            [1e-12, 0.],
                            [0.5, 0.],
                            [1., 0.],
                            [1.1, 0.],
                            [1.5, 0.],
                            [2. - 1e-12, 0.],
                            [2., 0.],
                        ],
                        [[2.0, 0.], [0., 0.]]
                    );

                    test_case!(
                        small2,
                        $function,
                        [
                            [1e-12, 0.],
                            [0.5, 0.],
                            [1., 0.],
                            [1.1, 0.],
                            [1.5, 0.],
                            [0., 0.],
                            [2. - 1e-12, 0.],
                            [2., 0.],
                        ],
                        [[2.0, 0.], [0., 0.]]
                    );

                    test_case!(
                        small3,
                        $function,
                        [
                            [0., 0.],
                            [1e-12, 0.],
                            [0.5, 0.],
                            [1., 0.],
                            [1.1, 0.],
                            [1.5, 0.],
                            [2. - 1e-12, 0.],
                            [2., 0.],
                        ],
                        [[2.0, 0.], [0., 0.]]
                    );

                    test_case!(
                        cocircular,
                        $function,
                        [[1., 0.], [0., 1.], [-1., 0.], [0., -1.]],
                        [[-1., 0.], [1., 0.]]
                    );

                     test_case!(
                        multiple,
                        $function,
                        [
                            [-1., -1.],
                            [-1., -1.],
                            [0., 0.],
                            [0., 0.],
                            [1., 1.],
                            [1., 1.],
                        ],
                        [[1., 1.], [-1., -1.]]
                    );

                    test_case!(
                        triangle,
                        $function,
                        [[0., 0.], [1., 0.], [1., 1.], [1., 1.]],
                        [[1., 1.], [0., 0.]]
                    );
                }
            }
        };
    }

    test_function!(smallest_enclosing_circle);
    test_function!(smallest_enclosing_circle_recursive);
}
