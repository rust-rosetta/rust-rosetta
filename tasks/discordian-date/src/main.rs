extern crate chrono;

use chrono::*;

const SEASONS: [&'static str; 5] =
    ["Chaos", "Discord", "Confusion", "Bureaucracy", "The Aftermath"];
const WEEKDAYS: [&'static str; 5] =
    ["Sweetmorn", "Boomtime", "Pungenday", "Prickle-Prickle", "Setting Orange"];
const YEAR_OFFSET: i32 = 1166;

fn discordian_day(day: usize, leap: bool) -> String {
    if day == 59 && leap {
        return String::from("St. Tib's Day");
    }
    let offset = if day > 59 && leap {
        1
    } else {
        0
    };
    let day_off = day - offset;
    let day_num = day_off % 73 + 1;
    let season = SEASONS[day_off / 73];
    let weekday = WEEKDAYS[day_off % 5];
    String::from(format!("{}, {} {}", weekday, season, day_num))
}

fn discordian_date<T: Datelike>(date: T) -> String {
    let dday = discordian_day(date.ordinal0() as usize, is_leap_year(date.year()));
    let year = date.year() + YEAR_OFFSET;
    String::from(format!("{}, YOLD {}", dday, year))
}

// implementation from https://en.wikipedia.org/wiki/Leap_year#Algorithm
#[cfg_attr(feature="clippy", allow(if_same_then_else, needless_bool))]
fn is_leap_year(year: i32) -> bool {
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}

fn main() {
    // sample date
    let utc = Utc::now();
    println!("{} in the Discordian Calendar is:", utc);
    println!("{}", discordian_date(utc));
    let local: DateTime<Local> = Local::now();
    println!("{} in the Discordian Calendar is:", local);
    println!("{}", discordian_date(local));
}

#[cfg(test)]
mod tests {
    use super::discordian_date;
    use chrono::*;

    #[test]
    fn curse_of_greyface_test() {
        let dt = Utc.ymd(-1166, 1, 1);
        assert_eq!("Sweetmorn, Chaos 1, YOLD 0", discordian_date(dt));
    }

    #[test]
    fn before_leap_day_test() {
        let dt = Utc.ymd(2016, 2, 28);
        assert_eq!("Prickle-Prickle, Chaos 59, YOLD 3182", discordian_date(dt));
    }

    #[test]
    fn leap_day_test() {
        let dt = Utc.ymd(2016, 2, 29);
        assert_eq!("St. Tib's Day, YOLD 3182", discordian_date(dt));
    }

    #[test]
    fn after_leap_day_test() {
        let dt = Utc.ymd(2016, 3, 1);
        assert_eq!("Setting Orange, Chaos 60, YOLD 3182", discordian_date(dt));
    }

    #[test]
    fn before_not_leap_day_test() {
        let dt = Utc.ymd(2015, 2, 28);
        assert_eq!("Prickle-Prickle, Chaos 59, YOLD 3181", discordian_date(dt));
    }

    #[test]
    fn not_leap_day_test() {
        let dt = Utc.ymd(2015, 3, 1);
        assert_eq!("Setting Orange, Chaos 60, YOLD 3181", discordian_date(dt));
    }

    #[test]
    fn birthday_test() {
        let dt = Utc.ymd(1994, 5, 14);
        assert_eq!("Prickle-Prickle, Discord 61, YOLD 3160",
                   discordian_date(dt));
    }

    #[test]
    fn magna_carta_test() {
        let dt = Utc.ymd(1215, 6, 15);
        assert_eq!("Sweetmorn, Confusion 20, YOLD 2381", discordian_date(dt));
    }
}
