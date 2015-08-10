/// http://rosettacode.org/wiki/Convert_seconds_to_compound_duration#Rust
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
    part(&mut comps, "w", 60 * 60 * 24 * 7, &mut secs);
    part(&mut comps, "d", 60 * 60 * 24, &mut secs);
    part(&mut comps, "h", 60 * 60, &mut secs);
    part(&mut comps, "m", 60, &mut secs);
    part(&mut comps, "s", 1, &mut secs);
    comps
}

#[cfg(not(test))]
fn main() {
	assert_eq!("1w, 3d, 2h, 45m, 23s", seconds_to_compound(873923));
	assert_eq!("1w, 3d, 2h, 45m", seconds_to_compound(873923 - 23));
	assert_eq!("1w, 3d, 2h, 23s", seconds_to_compound(873923 - 45 * 60));
	assert_eq!("3d, 2h, 23s", seconds_to_compound(873923 - 45 * 60 - 60 * 60 * 24 * 7));
}
