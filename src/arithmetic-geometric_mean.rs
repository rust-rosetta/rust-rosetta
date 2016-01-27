// http://rosettacode.org/wiki/Arithmetic-geometric_mean

//! Accepts two command-line arguments

fn main() {
    let mut args = std::env::args();

    let x = args.nth(1).expect("First argument not specified.").parse::<f32>().unwrap();
    let y = args.next().expect("Second argument not specified.").parse::<f32>().unwrap();

    let result = agm(x, y);
    println!("The arithmetic-geometric mean is {}", result);
}

fn agm(x: f32, y: f32) -> f32 {
    let e: f32 = 0.000001;
    let mut a = x;
    let mut g = y;
    let mut a1: f32;
    let mut g1: f32;

    if a * g < 0f32 {
        panic!("The arithmetric-geometric mean is undefined for numbers less than zero!");
    } else {
        loop {
            a1 = (a + g) / 2.;
            g1 = (a * g).sqrt();
            a = a1;
            g = g1;
            if (a - g).abs() < e {
                return a;
            }
        }
    }
}
