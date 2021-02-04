use std::f64::consts::PI;

#[derive(Debug, PartialEq, Eq)]
struct Time {
    h: u8,
    m: u8,
    s: u8,
}

impl Time {
    /// Create a Time from equivalent radian measure
    fn from_radians(mut rads: f64) -> Time {
        rads %= 2.0 * PI;
        if rads < 0.0 {
            rads += 2.0 * PI
        }
        Time {
            h: (rads * 12.0 / PI) as u8,
            m: ((rads * 720.0 / PI) % 60.0) as u8,
            s: ((rads * 43200.0 / PI) % 60.0).round() as u8,
        }
    }

    /// Create a Time from H/M/S
    fn from_parts(h: u8, m: u8, s: u8) -> Result<Time, ()> {
        if h > 23 || m > 59 || s > 59 {
            return Err(());
        }
        Ok(Time { h, m, s })
    }

    /// Return time as measure in radians
    fn as_radians(&self) -> f64 {
        ((self.h as f64 / 12.0) + (self.m as f64 / 720.0) + (self.s as f64 / 43200.0)) * PI
    }
}

/// Compute the mean time from a slice of times
fn mean_time(times: &[Time]) -> Time {
    // compute sum of sines and cosines
    let (ss, sc) = times
        .iter()
        .map(Time::as_radians)
        .map(|a| (a.sin(), a.cos()))
        .fold((0.0, 0.0), |(ss, sc), (s, c)| (ss + s, sc + c));
    // scaling does not matter for atan2, meaning we do not have to divide sums by len
    Time::from_radians(ss.atan2(sc))
}

fn main() {
    let times = [
        Time::from_parts(23, 00, 17).unwrap(),
        Time::from_parts(23, 40, 20).unwrap(),
        Time::from_parts(00, 12, 45).unwrap(),
        Time::from_parts(00, 17, 19).unwrap(),
    ];

    let mean = mean_time(&times);

    println!("{:02}:{:02}:{:02}", mean.h, mean.m, mean.s);
}

#[test]
fn test_time_eq() {
    // simple tests
    assert_eq!(
        Time::from_parts(00, 00, 00).unwrap(),
        Time::from_radians(0.0)
    );
    assert_eq!(
        Time::from_parts(12, 00, 00).unwrap(),
        Time::from_radians(PI)
    );
    assert_eq!(
        Time::from_parts(18, 00, 00).unwrap(),
        Time::from_radians(3.0 * PI / 2.0)
    );

    // targeted tests
    assert_eq!(
        Time::from_parts(23, 00, 17).unwrap(),
        Time::from_radians(6.022622194267266)
    );
    assert_eq!(
        Time::from_parts(23, 40, 20).unwrap(),
        Time::from_radians(6.197373285623199)
    );
    assert_eq!(
        Time::from_parts(00, 12, 45).unwrap(),
        Time::from_radians(0.05563236990731926)
    );
    assert_eq!(
        Time::from_parts(00, 17, 19).unwrap(),
        Time::from_radians(0.07555821220092118)
    );
    assert_eq!(
        Time::from_parts(23, 47, 23).unwrap(),
        Time::from_radians(6.228134713689599)
    );
}
