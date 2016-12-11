fn seconds_to_compound(secs: u32) -> String {
    let part = |comps: &mut String, c: &str, one: u32, secs: &mut u32| {
        if *secs >= one {
            let div = *secs / one;
            comps.push_str(&(div.to_string() + c));
            *secs -= one * div;
            if *secs > 0 {
                comps.push_str(", ");
            }
        }
    };

    let mut secs = secs;
    let mut comps = String::new();
    part(&mut comps, " wk", 60 * 60 * 24 * 7, &mut secs);
    part(&mut comps, " d", 60 * 60 * 24, &mut secs);
    part(&mut comps, " hr", 60 * 60, &mut secs);
    part(&mut comps, " min", 60, &mut secs);
    part(&mut comps, " sec", 1, &mut secs);
    comps
}

#[test]
fn hours_and_seconds() {
    assert_eq!(seconds_to_compound(7259), "2 hr, 59 sec");
}

#[test]
fn one_day() {
    assert_eq!(seconds_to_compound(86400), "1 d");
}

#[test]
fn six_million_seconds() {
    assert_eq!(seconds_to_compound(6000000), "9 wk, 6 d, 10 hr, 40 min");
}

fn main() {
    println!("7,259 seconds = {}", seconds_to_compound(7259));
    println!("86,400 seconds = {}", seconds_to_compound(86400));
    println!("6,000,000 seconds = {}", seconds_to_compound(6000000));
}
