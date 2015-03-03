// http://rosettacode.org/wiki/Arithmetric-geometric_mean
// Accepts two command line arguments
// cargo run --name agm arg1 arg2

#![allow(unused_features)] // feature(os) is used only in main
#![feature(std_misc)]
#![feature(os)]
#![feature(collections)]
#![feature(core)]

use std::num::Float;

#[cfg(not(test))]
fn main () {
    use std::borrow::ToOwned;
    let mut args = std::env::args();

    let x = args.next().unwrap().to_owned().parse::<f32>().unwrap();
    let y = args.next().unwrap().to_owned().parse::<f32>().unwrap();

    let result = agm(x,y);
    println!("The arithmetic-geometric mean is {}", result);
}

fn abs<T: Float>(n: T) -> T {
    if n < Float::zero() { -n } else { n }
}

fn agm (x: f32, y: f32) -> f32 {
    let e: f32 = 0.000001;
    let mut a = x;
    let mut g = y;
    let mut a1: f32;
    let mut g1: f32;

    if a * g < 0f32 {
        panic!("The arithmetric-geometric mean is undefined for numbers less than zero!");
    }
    else {
        loop {
            a1 = (a + g) / 2f32;
            g1 = (a * g).sqrt();
            a = a1;
            g = g1;
            if abs( a - g) < e {  return a; }
        }
    }

}

#[test]
fn test_agm_12_6() {
    assert! ( agm(12f32,6f32) == 8.740746f32 );
}
