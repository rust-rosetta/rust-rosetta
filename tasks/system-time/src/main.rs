use std::time::{SystemTime, UNIX_EPOCH};

use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::macros::format_description;
use time::OffsetDateTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The standard library contains rudimentary functions for determining the system time.
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH)?;

    // Prints the current time as a timespec containing the seconds
    // and nanoseconds since 1970-01-01T00:00:00Z.
    println!(
        "seconds: {} nanoseconds: {}",
        duration_since_epoch.as_secs(),
        duration_since_epoch.subsec_nanos()
    );

    // To format the system time, we'll need to use an extern crate such as `time` or `chrono`.
    let now = OffsetDateTime::from(now);

    // Display time formatted according to RFC 2822,
    // eg "Wed, 29 Oct 2014 22:26:17 -0600"
    println!("rfc2822: {}", now.format(&Rfc2822)?);

    // Display time formatted according to RFC 3339/ISO8601
    // eg "2014-10-29T22:26:17+07:00"
    println!("rfc3339: {}", now.format(&Rfc3339)?);

    // Display time in a custom format (eg "22:26:17") using strftime
    println!(
        "Custom: {}",
        now.format(&format_description!("[hour]:[minute]:[second]"))?
    );

    Ok(())
}
