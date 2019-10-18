fn main() -> Result<(), std::num::ParseFloatError> {
    let args = std::env::args();
    if args.len() == 2 {
        let k: f64 = args.skip(1).next().unwrap().parse()?;
        if k < 0.0 {
            println!("{:2.2} K is below absolute zero", k);
        } else {
            println!(
                "K {:2.2}\nC {:2.2}\nF {:2.2}\nR {:2.2}",
                k,
                kelvin_to_celsius(k),
                kelvin_to_fahrenheit(k),
                kelvin_to_rankine(k)
            );
        }
    }
    Ok(())
}

fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    k * 1.8 - 459.67
}

fn kelvin_to_rankine(k: f64) -> f64 {
    k * 1.8
}

#[test]
fn test() {
    let k = 21.0;
    assert_eq!(kelvin_to_celsius(k), -252.14999999999998);
    assert_eq!(kelvin_to_fahrenheit(k), -421.87);
    assert_eq!(kelvin_to_rankine(k), 37.800000000000004);
}
