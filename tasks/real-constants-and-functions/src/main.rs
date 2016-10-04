use std::f64::consts::*;

#[cfg_attr(feature = "clippy", allow(float_cmp))]
fn main() {
    // e (base of the natural logarithm)
    let mut x = E;
    // π
    x += PI;
    // square root
    x = x.sqrt();
    // logarithm (any base allowed)
    x = x.ln();
    // ceiling (smallest integer not less than this number--not the same as round up)
    x = x.ceil();
    // exponential (ex)
    x = x.exp();
    // absolute value (a.k.a. "magnitude")
    x = x.abs();
    // floor (largest integer less than or equal to this number--not the same as truncate or int)
    x = x.floor();
    // power (xy)
    x = x.powf(x);

    assert_eq!(x, 4.0);
}
