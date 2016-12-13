fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    for &year in &[1900, 1995, 1996, 1999, 2000, 2001] {
        println!("{} {} a leap year",
                 year,
                 if is_leap_year(year) {
                     "is"
                 } else {
                     "is not"
                 });
    }
}

#[test]
fn test_regular_common_year() {
    assert_eq!(is_leap_year(2014), false);
}

#[test]
fn test_regular_leap_year() {
    assert_eq!(is_leap_year(2012), true);
}

#[test]
fn test_century_common_year() {
    assert_eq!(is_leap_year(1900), false);
}

#[test]
fn test_400_leap_year() {
    assert_eq!(is_leap_year(2000), true);
}
