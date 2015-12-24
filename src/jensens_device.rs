// http://rosettacode.org/wiki/Jensen's_Device

fn harmonic_sum<F>(lo:usize, hi:usize, term: F ) -> f32
    where F : Fn(f32) -> f32 {
    (lo..hi+1).fold(0.0, |acc, item| acc + term(item as f32))
}

fn main() {
    println!("{}", harmonic_sum(1, 100, |i| 1.0 / i) );
}

#[test]
fn test_harm_sum() {
    let term = |i| 1.0 / i;
    assert_eq!( harmonic_sum(1, 100, &term), 5.187378);
    assert_eq!( harmonic_sum(1, 50, &term), 4.4992056);
    assert_eq!( harmonic_sum(1, 1000, &term), 7.4854784);
    assert_eq!( harmonic_sum(1, 2, &term), 1.5);
}



