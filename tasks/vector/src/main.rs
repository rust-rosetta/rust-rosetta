mod vector;

use std::f64::consts::PI;
use vector::Vector;

fn main() {
    println!("{:?}", Vector::new(4, 5));
    println!("{:.4}", Vector::from_polar(3.0, PI / 3 as f64));
    println!("{}", Vector::new(2, 3) + Vector::new(4, 6));
    println!("{:.4}", Vector::new(5.6, 1.3) - Vector::new(4.2, 6.1));
    println!("{:.4}", Vector::new(3.0, 4.2) * 2.3);
    println!("{:.4}", Vector::new(3.0, 4.2) / 2.3);
    println!("{}", Vector::new(3, 4) / 2);
}
