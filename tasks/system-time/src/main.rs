extern crate time;
use time::{at, get_time, strftime};

fn main() {
    // Prints the current time as a timespec containing the seconds
    // and nanoseconds since 1970-01-01T00:00:00Z.
    let time_ts = get_time();
    println!("seconds: {} nanoseconds: {}", time_ts.sec, time_ts.nsec);

    // Convert the timespec to a broken-down time value Tm
    // Could also use "let time_tm = now();" to get directly
    let time_tm = at(time_ts);

    // Display time formatted according to the asctime format in ISO
    // C, in the local timezone, eg "Wed Oct 29 22:26:17 2014"
    println!("ctime: {}", time_tm.ctime());

    // Display time formatted according to RFC 822,
    // eg "Wed, 29 Oct 2014 22:26:17"
    println!("rfc822: {}", time_tm.rfc822());

    // Display time formatted according to RFC 3339/ISO8601
    // eg "2014-10-29T22:26:17+07:00"
    println!("rfc3339: {}", time_tm.rfc3339());

    // Display time in a custom format (eg "22:26:17") using strftime
    println!("Custom: {}", strftime("%H:%M:%S", &time_tm).unwrap());
}
