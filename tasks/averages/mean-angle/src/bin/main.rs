use std::f64;

// the macro is from
// http://stackoverflow.com/questions/30856285/assert-eq-with-floating-
// point-numbers-and-delta
fn mean_angle(angles: &[f64]) -> f64 {
    let length: f64 = angles.len() as f64;
    let cos_mean: f64 = angles.iter().fold(0.0, |sum, i| sum + i.to_radians().cos()) / length;
    let sin_mean: f64 = angles.iter().fold(0.0, |sum, i| sum + i.to_radians().sin()) / length;
    (sin_mean).atan2(cos_mean).to_degrees()
}

fn main() {
    let angles1 = [350.0_f64, 10.0];
    let angles2 = [90.0_f64, 180.0, 270.0, 360.0];
    let angles3 = [10.0_f64, 20.0, 30.0];
    println!("Mean Angle for {:?}  is {:.5} degrees",
             &angles1,
             mean_angle(&angles1));
    println!("Mean Angle for {:?}  is {:.5} degrees",
             &angles2,
             mean_angle(&angles2));
    println!("Mean Angle for {:?}  is {:.5} degrees",
             &angles3,
             mean_angle(&angles3));
}

macro_rules! assert_diff{
    ($x: expr,$y : expr, $diff :expr)=>{
        if ( $x - $y ).abs() > $diff {
            panic!("floating point difference is to big {}", $x - $y );
        }
    }
}

#[test]
fn calculate() {
    let angles1 = [350.0_f64, 10.0];
    let angles2 = [90.0_f64, 180.0, 270.0, 360.0];
    let angles3 = [10.0_f64, 20.0, 30.0];
    assert_diff!(0.0, mean_angle(&angles1), 0.001);
    assert_diff!(-90.0, mean_angle(&angles2), 0.001);
    assert_diff!(20.0, mean_angle(&angles3), 0.001);
}
