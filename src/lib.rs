//! Iterative and recursive two-dimensional implementations of Welzl's algorithm for computing the smallest enclosing circle.
//!
//! The implementations in this crate solve the smallest enclosing circle problem, also known as smallest-circle problem, minimum covering circle problem, or bounding circle problem.
//! Welzl's algorithm solves this problem in expected O(n) runtime.
//! The original algorithm was formulated as recursive program, which leads to call stack overflow for larger problem sizes.
//! Thus, the iterative implementation in this crate should be preferred.
//! However, the recursive version is provided for demonstration purposes.
//!
//!
//! The implementation is based on the following work:
//!
//! Welzl, E. (1991). Smallest enclosing disks (balls and ellipsoids).
//! In New results and new trends in computer science (pp. 359-370).
//! Springer, Berlin, Heidelberg.
//! 
//! # A Note on Custom Predicates
//!
//! # Examples
//!
//! ```
//! use smallest_enclosing_circle::smallest_enclosing_circle;
//!
//! // Input: Four corner points of square box of unit size
//! let points = Vec::from([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
//! let circle = smallest_enclosing_circle(points.into_iter());
//! println!("Circle: {:?}", circle);
//! // Circle: Three([0.0, 1.0], [1.0, 1.0], [1.0, 0.0], false);
//! println!("Center: {:?}", circle.center());
//! // Center: Some([0.5, 0.5])
//! println!("Radius: {:?}", circle.radius());
//! // Radius: 0.7071067811865476
//! ```

pub mod smallest_enclosing_circle;
pub mod circle;
pub mod geometry;
pub mod predicates;

pub use self::smallest_enclosing_circle::{smallest_enclosing_circle, smallest_enclosing_circle_with_predicate};
pub use self::circle::{Circle2D};
//pub use self::smallest_enclosing_circle::smallest_enclosing_circle_recursive;
//pub use self::smallest_enclosing_circle::Circle2D;
