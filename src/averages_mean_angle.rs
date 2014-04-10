// http://rosettacode.org/wiki/Averages/Mean_angle

use std::num::Float;

fn mean_angle(angles: &[f64]) -> f64 {
    let mean_cos = angles.iter().map(|&x| (x * Float::pi() / 180.).cos())
                                .fold(0., |a, b| a + b) / (angles.len() as f64);
    let mean_sin = angles.iter().map(|&x| (x * Float::pi() / 180.).sin())
                                .fold(0., |a, b| a + b) / (angles.len() as f64);

    mean_sin.atan2(&mean_cos) * 180. / Float::pi()
}

fn main() {
    let set1 = [350., 10.];
    let set2 = [90., 180., 270., 360.];
    let set3 = [10., 20., 30.];

    println!("Mean angle of first set is {} degrees", mean_angle(set1));
    println!("Mean angle of second set is {} degrees", mean_angle(set2));
    println!("Mean angle of third set is {} degrees", mean_angle(set3));
}
