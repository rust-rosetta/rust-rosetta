#![feature(unboxed_closures)]
use std::ops::Add;

pub struct G<T, U> {
    n: T,
}

macro_rules! add_impl(
    ($($t:ty)*) => ($(
        impl FnMut<($t,), $t> for G<$t, $t> {
            extern "rust-call" fn call_mut(&mut self, (i,):($t,)) -> $t {
                self.n = self.n + i;
                self.n
            }
        }
    )*)
);

add_impl!(isize usize u8 u16 u32 u64 i8 i16 i32 i64 f32 f64);

pub fn accum<T: Add<T, Output=U>, U>(n: T) -> G<T, U> {
    G { n: n }
}

#[cfg(not(test))]
pub fn main() {
    println!("{:?}", accumulate());
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
