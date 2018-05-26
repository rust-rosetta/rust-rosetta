extern crate chrono;
use chrono::prelude::*;

/// Months with 31 days
const LONGMONTHS: [u32; 7] = [1, 3, 5, 7, 8, 10, 12];

/// Get all the tuples (year, month) in wich there is five Fridays, five Saturdays and five Sundays
/// between the years start and end (inclusive).
fn five_weekends(start: i32, end: i32) -> Vec<(i32, u32)> {
    let mut out = vec![];

    for year in start..=end {
        for month in LONGMONTHS.iter() {
            // Five weekends if a 31-days month starts with a Friday.
            if Local.ymd(year, *month, 1).weekday() == Weekday::Fri {
                out.push((year, *month));
            }
        }
    }

    out
}

fn main() {
    let out = five_weekends(1900, 2100);

    let len = out.len();
    println!(
        "There are {} months of which the first and last five are:",
        len
    );
    for (y, m) in &out[..5] {
        println!("\t{} / {}", y, m);
    }
    println!("...");
    for (y, m) in &out[(len - 5..)] {
        println!("\t{} / {}", y, m);
    }
}

#[test]
fn test() {
    let out = five_weekends(1900, 2100);
    assert_eq!(out.len(), 201);
}
