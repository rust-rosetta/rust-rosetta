// Implements http://rosettacode.org/wiki/Averages/Mean_angle
#![feature(core)]

use std::num::Float;
use std::f64::consts::PI;

fn mean_angle(angles: &[f64]) -> f64 {
    let (sum_cos, sum_sin) = angles.iter()
                                   .map(|&x| x * PI / 180.)
                                   // We map into a tuple of sin and cos so we
                                   // only need to iterate the slice once.
                                   .map(|x| (x.cos(), x.sin()))
                                   .fold((0., 0.),
                                        |(sc, ss), (c, s)| (sc + c, ss + s));

    let mean_cos = sum_cos / angles.len() as f64;
    let mean_sin = sum_sin / angles.len() as f64;

    mean_sin.atan2(mean_cos) * 180. / PI
}

#[cfg(not(test))]
fn main() {
    let set1 = &[350., 10.];
    let set2 = &[90., 180., 270., 360.];
    let set3 = &[10., 20., 30.];

    println!("Mean angle of first set is {} degrees", mean_angle(set1));
    println!("Mean angle of second set is {} degrees", mean_angle(set2));
    println!("Mean angle of third set is {} degrees", mean_angle(set3));
}

#[test]
fn basic_tests() {
    let set1 = [350., 10.];
    let set2 = [90., 180., 270., 360.];
    let set3 = [10., 20., 30.];

    // We need to round the numbers
    assert_eq!(mean_angle(&set1).round(), 0.);
    assert_eq!(mean_angle(&set2).round(), -90.);
    assert_eq!(mean_angle(&set3).round(), 20.);
}
