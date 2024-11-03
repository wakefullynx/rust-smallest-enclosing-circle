# Smallest Enclosing Circle

Iterative and recursive two-dimensional implementations of Welzl's algorithm for computing the smallest enclosing circle.

**Live Demo:** [wakefullynx.dev/smallest-enclosing-circle-demo/](https://wakefullynx.dev/smallest-enclosing-circle-demo/)

**Crate:** [crates.io](https://crates.io/crates/smallest-enclosing-circle)

**Documentation:** [docs.rs](https://docs.rs/smallest-enclosing-circle/latest/smallest_enclosing_circle/)

The implementations in this crate solve the smallest enclosing circle problem, also known as smallest-circle problem, minimum covering circle problem, or bounding circle problem.
Welzl's algorithm solves this problem in expected O(n) runtime.
The original algorithm was formulated as recursive program, which leads to call stack overflow for larger problem sizes.
Thus, the iterative implementation in this crate should be preferred.
However, the recursive version is provided for demonstration purposes.

*Please note that the expected runtime only holds for randomized inputs (i.e., you may want to shuffle your input stream in advance).*

The implementation is based on the following work(s):

Welzl, E. (1991). Smallest enclosing disks (balls and ellipsoids).
In New results and new trends in computer science (pp. 359-370).
Springer, Berlin, Heidelberg.

## Examples

```rust
use smallest_enclosing_circle::smallest_enclosing_circle;

// Input: Four corner points of square box of unit size
let points = Vec::from([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]);
let circle = smallest_enclosing_circle(points.into_iter());
println!("Circle: {:?}", circle);
// Circle: Three([0.0, 1.0], [1.0, 1.0], [1.0, 0.0], false);
println!("Center: {:?}", circle.center());
// Center: Some([0.5, 0.5])
println!("Radius: {:?}", circle.radius());
// Radius: 0.7071067811865476
```

## Related

An equivalent [TypeScript](https://github.com/wakefullynx/ts-smallest-circle) implementation is available on [GitHub](https://github.com/wakefullynx/ts-smallest-circle) and [npm](https://www.npmjs.com/package/smallest-circle).
