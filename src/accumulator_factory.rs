use std::ops::Add;

pub fn accum<'a, T>(mut n: T) -> Box<FnMut(T) -> T + 'a>
    where T: 'a + Add<T, Output=T> + Copy
{
    Box::new(move |i: T| { n = n + i; n })
}

#[cfg(not(test))]
pub fn main() {
    println!("{}", accumulate());
}

#[test]
pub fn test() {
    assert_eq!(8.3, accumulate());
}

fn accumulate() -> f32 {
    // Deviation: works with all types implementing addition, but not a mixture
    // of types (it is possible to handle mixed types, but would require type
    // switching at the moment).
    let mut g = accum(1f32);
    g(5.);
    accum(3i32);
    g(2.3)
}
