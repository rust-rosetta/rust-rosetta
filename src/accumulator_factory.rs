// Implements http://rosettacode.org/wiki/Accumulator_factory
#![feature(overloaded_calls)]
pub struct G<T, U> {
    n: T,
}

impl<T: Add<U, T> + Clone, U> FnMut<(U,), T> for G<T, U> {
    extern "rust-call" fn call_mut(&mut self, (i,):(U,)) -> T {
        self.n = self.n + i;
        self.n.clone()
    }
}

pub fn accum<T: Add<T, U> + Clone, U>(n: T) -> G<T, U> {
    G { n: n }
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
