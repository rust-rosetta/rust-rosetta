fn fix_price(num: f64) -> f64 {
    let cents = (num.fract() * 100_f64) as u16;

    match cents {
        96...101 => 1.00,
        91...96 => 0.98,
        86...91 => 0.94,
        81...86 => 0.90,
        76...81 => 0.86,
        71...76 => 0.82,
        66...71 => 0.78,
        61...66 => 0.74,
        56...61 => 0.70,
        51...56 => 0.66,
        46...51 => 0.62,
        41...46 => 0.58,
        36...41 => 0.54,
        31...36 => 0.50,
        26...31 => 0.44,
        21...26 => 0.38,
        16...21 => 0.32,
        11...16 => 0.26,
        06...11 => 0.18,
        00...06 => 0.10,
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
