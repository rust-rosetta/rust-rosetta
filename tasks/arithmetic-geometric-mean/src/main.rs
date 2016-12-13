//! Accepts two command line arguments
//! cargo run --name agm arg1 arg2

use std::f32;

fn main() {
    let mut args = std::env::args();

    let x = args.next().unwrap().to_owned().parse::<f32>().unwrap();
    let y = args.next().unwrap().to_owned().parse::<f32>().unwrap();

    let result = agm(x, y);
    println!("The arithmetic-geometric mean is {}", result);
}

fn agm(x: f32, y: f32) -> f32 {
    let mut a = x;
    let mut g = y;
    let mut a1: f32;
    let mut g1: f32;

    if a * g < 0f32 {
        panic!("The arithmetic-geometric mean is undefined for numbers less than zero!");
    } else {
        loop {
            a1 = (a + g) / 2f32;
            g1 = (a * g).sqrt();
            a = a1;
            g = g1;
            if (a - g).abs() < f32::EPSILON {
                return a;
            }
        }
    }
}

#[test]
fn test_agm_12_6() {
    use std::f32;

    assert!((agm(12f32, 6f32) - 8.740746f32).abs() < f32::EPSILON);
}
