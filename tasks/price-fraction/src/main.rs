fn fix_price(num: f64) -> f64 {
    let cents = (num.fract() * 100_f64) as u16;

    match cents {
        96...100 => 1.00,
        91...95 => 0.98,
        86...90 => 0.94,
        81...85 => 0.90,
        76...80 => 0.86,
        71...75 => 0.82,
        66...70 => 0.78,
        61...65 => 0.74,
        56...60 => 0.70,
        51...55 => 0.66,
        46...50 => 0.62,
        41...45 => 0.58,
        36...40 => 0.54,
        31...35 => 0.50,
        26...30 => 0.44,
        21...25 => 0.38,
        16...20 => 0.32,
        11...15 => 0.26,
        6...10 => 0.18,
        0...5 => 0.10,
        // panics on invalid value
        _ => panic!("price out of range"),
    }
}

fn main() {
    let mut n: f64 = 0.04;
    while n <= 1.00 {
        println!("{:.2} => {:.2}", n, fix_price(n));
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
