//! This crate solves the smallest enclosing circle problem, also known as smallest-circle problem, minimum covering circle problem, or bounding circle problem.
//! Welzl's algorithm solves this problem in expected `O(n)` runtime.
//! The original algorithm was formulated as a recursive program, which leads to a call stack overflow for larger problem sizes.
//! Thus, the iterative implementation in this crate should be preferred.
//! However, the recursive version is provided for demonstration purposes.
//!
//! *Please note that the expected runtime only holds for randomized inputs (i.e., you may want to shuffle your input stream in advance).*
//! 
//! The main functionality of this crate is the [`smallest_enclosing_circle`] function.
//!
//! The implementation is based on the following work:
//!
//! Welzl, E. (1991). Smallest enclosing disks (balls and ellipsoids).
//! In New results and new trends in computer science (pp. 359-370).
//! Springer, Berlin, Heidelberg.
//!
//! # Examples
//!
//! ```
//! use smallest_enclosing_circle::smallest_enclosing_circle;
//! use smallest_enclosing_circle::predicates::in_circle::DefaultInCircle;
//!
//! // Input: Four corner points of square box of unit size
//! let circle = smallest_enclosing_circle([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
//! assert_eq!(circle.center(), Some([0.5, 0.5]));
//! assert_eq!(circle.radius(), Some(f64::sqrt(2.) / 2.));
//! ```
//! 
//! # A Note on Custom Predicates
//! 
//! Some of the methods in this crate come in two flavors: with a simple interface (e.g., [`smallest_enclosing_circle`]), and with the possibility to supply your own predicates (e.g., [`smallest_enclosing_circle_with_predicate`]).
//! This crates uses the [`predicates::orientation::Orientation`] and [`predicates::in_circle::InCircle`] predicates, which you could implement in your own way. 
//! A possible use case would be that you include the functionality in this crate in, e.g., a higher level algorithm and you need both parts to make the exact same geometric decisions.
//! However, if you don't specify your own predicates, then the default implementation is used, based on [`geometry_predicates`] crate, which is already a very reasonable choice.

pub mod algorithm;
pub mod circle;
pub mod geometry;
pub mod predicates;

pub use self::algorithm::{smallest_enclosing_circle, smallest_enclosing_circle_with_predicate};
pub use self::circle::{Circle2D};
