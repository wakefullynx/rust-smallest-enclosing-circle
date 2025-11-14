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

pub mod algorithm;
pub mod circle;
pub mod geometry;
pub mod predicates;

pub use self::algorithm::{smallest_enclosing_circle, smallest_enclosing_circle_with_predicate};
pub use self::circle::{Circle2D};
