//! This is a partial solution and only implements trapezium integration.

fn trapezium_integral<F>(f: F, range: std::ops::Range<f64>, n_steps: u32) -> f64
    where F: Fn(f64) -> f64
{
    let step_size = (range.end - range.start) / n_steps as f64;

    let mut integral = (f(range.start) + f(range.end)) / 2.;
    let mut pos = range.start + step_size;
    while pos < range.end {
        integral += f(pos);
        pos += step_size;
    }
    integral * step_size
}

fn main() {
    println!("{}", trapezium_integral(|x| x.powi(3), 0.0..1.0, 100));
    println!("{}", trapezium_integral(|x| 1.0 / x, 1.0..100.0, 1000));
    println!("{}", trapezium_integral(|x| x, 0.0..5000.0, 5_000_000));
    println!("{}", trapezium_integral(|x| x, 0.0..6000.0, 6_000_000));
}
