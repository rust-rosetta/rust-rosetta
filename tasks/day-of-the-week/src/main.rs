extern crate chrono;

use chrono::*;

fn main() {
    for i in 2008..2121 {
        let dt = Utc.ymd(i, 12, 25);
        if dt.weekday() == Weekday::Sun {
            println!("{} is a Sunday.", dt.format("%Y-%m-%d"));
        }
    }
}
