use std::io;
struct SundialCalculation {
    hour_angle: f64,
    hour_line_angle: f64,
}

fn get_input(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
    println!("{}", prompt);
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input)?;
    Ok(input.trim().parse::<f64>()?)
}

fn calculate_sundial(hour: i8, lat: f64, lng: f64, meridian: f64) -> SundialCalculation {
    let diff = lng - meridian;
    let hour_angle = f64::from(hour) * 15. - diff;
    let hour_line_angle = (hour_angle.to_radians().tan() * lat.to_radians().sin())
        .atan()
        .to_degrees();

    SundialCalculation {
        hour_angle,
        hour_line_angle,
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lat = get_input("Enter latitude       => ")?;
    let lng = get_input("Enter longitude      => ")?;
    let meridian = get_input("Enter legal meridian => ")?;
    let diff = lng - meridian;

    let sine_lat = lat.to_radians().sin();
    println!("Sine of latitude: {:.5}", sine_lat);
    println!("Diff longitude: {}", diff);

    println!("  Hrs Angle   Hour Line Angle");
    (-6..=6).for_each(|hour| {
        let sd = calculate_sundial(hour, lat, lng, meridian);
        println!(
            "{:>3}{} {:>5}   {:>+15.5}",
            if hour == 0 { 12 } else { (hour + 12) % 12 },
            if hour <= 6 { "pm" } else { "am" },
            sd.hour_angle,
            sd.hour_line_angle
        );
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_calculate_sundial() {
        let result = calculate_sundial(-6, -4.95, -150.5, -150.);
        assert_approx_eq!(result.hour_angle, -89.5);
        assert_approx_eq!(result.hour_line_angle, 84.224832601);
    }
}
