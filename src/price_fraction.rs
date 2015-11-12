// http://rosettacode.org/wiki/Price_fraction

fn fix_price(num: f64) -> f64 {
    match num {
        0.96...1.00 => 1.00,
        0.91...0.96 => 0.98,
        0.86...0.91 => 0.94,
        0.81...0.86 => 0.90,
        0.76...0.81 => 0.86,
        0.71...0.76 => 0.82,
        0.66...0.71 => 0.78,
        0.61...0.66 => 0.74,
        0.56...0.61 => 0.70,
        0.51...0.56 => 0.66,
        0.46...0.51 => 0.62,
        0.41...0.46 => 0.58,
        0.36...0.41 => 0.54,
        0.31...0.36 => 0.50,
        0.26...0.31 => 0.44,
        0.21...0.26 => 0.38,
        0.16...0.21 => 0.32,
        0.11...0.16 => 0.26,
        0.06...0.11 => 0.18,
        0.00...0.06 => 0.10,
        // panics on invalid value
        _ => unreachable!(),
    }
}

fn main() {
    let mut n: f64 = 0.04;
    while n <= 1.00 {
        println!("{:.2} => {}", n, fix_price(n));
        n += 0.04;
    }
}

// typically this could be included in the match as those check for exhaustiveness already
// by explicitly listing all remaining ranges / values instead of a catch-all underscore (_)
// but f64::NaN, f64::INFINITY and f64::NEG_INFINITY can't be matched like this
#[test]
fn exhaustiveness_check() {
    let mut input_price = 0.;
    while input_price <= 1. {
        fix_price(input_price);
        input_price += 0.01;
    }
}
