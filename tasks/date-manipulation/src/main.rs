use chrono::prelude::*;
use chrono::Duration;

fn main() {
    // Chrono allows parsing time zone abbreviations like "EST", but
    // their meaning is ignored due to a lack of standardization.
    //
    // This solution compromises by augmenting the parsed datetime
    // with the timezone using the IANA abbreviation.
    let ndt =
        NaiveDateTime::parse_from_str("March 7 2009 7:30pm EST", "%B %e %Y %l:%M%P %Z").unwrap();

    // add TZ manually
    let dt = chrono_tz::EST.from_local_datetime(&ndt).unwrap();
    println!("Date parsed: {:?}", dt);

    let new_date = dt + Duration::hours(12);
    println!("+12 hrs in EST: {:?}", new_date);
    println!(
        "+12 hrs in CET: {:?}",
        new_date.with_timezone(&chrono_tz::CET)
    );
}
