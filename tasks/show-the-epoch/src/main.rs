use chrono::{TimeZone, Utc};

fn main() {
    let epoch = Utc.timestamp(0, 0);
    println!("{}", epoch.to_rfc3339());
}
