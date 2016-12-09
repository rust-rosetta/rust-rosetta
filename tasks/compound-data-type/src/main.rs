//! There are three kinds of `structs` in Rust, two of which would be suitable to represent a
//! point.

/// C-like struct.
///
/// Defines a generic struct where x and y can be of any type `T`.
struct Point<T> {
    x: T,
    y: T,
}

/// Tuple struct
///
/// These are basically just named tuples.
struct TuplePoint<T>(T, T);

fn main() {
    let p1 = Point { x: 1.0, y: 2.5 };    // p is of type Point<f64>
    println!("{}, {}", p1.x, p1.y);

    let p2 = TuplePoint(1.0, 2.5);
    println!("{}, {}", p2.0, p2.1);

    // A plain tuple may also be used.
    let p3 = (1.0, 2.5);
    println!("{}, {}", p3.0, p3.1)
}
