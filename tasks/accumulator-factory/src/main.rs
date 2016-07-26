use std::ops::Add;

fn accum<'a, T>(mut n: T) -> Box<FnMut(T) -> T + 'a>
    where T: 'a + Add<T, Output = T> + Copy
{
    Box::new(move |i: T| {
        n = n + i;
        n
    })
}

fn main() {
    println!("{}", accumulate());
}

#[test]
fn test() {
    use std::f32;

    assert!((8.3 - accumulate()).abs() < f32::EPSILON);
}

/// Deviation: works with all types implementing addition, but not a mixture
/// of types (it is possible to handle mixed types, but would require type
/// switching at the moment).
fn accumulate() -> f32 {
    let mut g = accum(1f32);
    g(5.);
    accum(3i32);
    g(2.3)
}
