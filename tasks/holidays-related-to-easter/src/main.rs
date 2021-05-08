use std::ops::Add;

use chrono::{prelude::*, Duration};

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
fn get_easter_day(year: u32) -> chrono::NaiveDate {
    let k = (f64::from(year) / 100.).floor();
    let d = (19 * (year % 19)
        + ((15 - ((13. + 8. * k) / 25.).floor() as u32 + k as u32 - (k / 4.).floor() as u32) % 30))
        % 30;
    let e =
        (2 * (year % 4) + 4 * (year % 7) + 6 * d + ((4 + k as u32 - (k / 4.).floor() as u32) % 7))
            % 7;

    let (month, day) = match d {
        29 if e == 6 => (4, 19),
        28 if e == 6 => (4, 18),
        _ if d + e < 10 => (3, 22 + d + e),
        _ => (4, d + e - 9),
    };

    NaiveDate::from_ymd(year as i32, month, day)
}
fn main() {
    let holidays = vec![
        ("Easter", Duration::days(0)),
        ("Ascension", Duration::days(39)),
        ("Pentecost", Duration::days(49)),
        ("Trinity", Duration::days(56)),
        ("Corpus Christi", Duration::days(60)),
    ];
    for year in (400..=2100).step_by(100).chain(2010..=2020) {
        print!("{}: ", year);
        for (name, offset) in &holidays {
            print!(
                "{}: {}, ",
                name,
                get_easter_day(year).add(*offset).format("%a %d %h")
            );
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_easter_day() {
        assert_eq!(NaiveDate::from_ymd(1777, 3, 30), get_easter_day(1777));
        assert_eq!(NaiveDate::from_ymd(2021, 4, 4), get_easter_day(2021));
    }
}
