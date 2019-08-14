use std::error::Error;
/// Create a program that downloads the time from this URL: http://tycho.usno.navy.mil/cgi-bin/timer.pl
/// and then prints the current UTC time by extracting just the UTC time from the web page's HTML.
use std::io::prelude::*;

fn get_page_html(url: &str) -> Result<String, Box<dyn Error>> {
    let mut response = reqwest::get(url)?;
    let mut response_text = String::new();
    response.read_to_string(&mut response_text)?;
    Ok(response_text)
}
fn main() -> Result<(), Box<dyn Error>> {
    let raw_html = get_page_html("http://tycho.usno.navy.mil/cgi-bin/timer.pl")?;
    // parse the response body by <BR> tags
    let mut lines = raw_html
        .split("<BR>")
        .filter(|line| match line.find("UTC") {
            Some(_) => true,
            None => false,
        })
        .map(|line| line.trim_end_matches("UTC\t\tUniversal Time\n").trim());

    if let Some(timestamp) = lines.next() {
        print!("{}", timestamp);
    } else {
        eprint!("Error: Could not parse URL for getting current time.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_page_html};
    use std::error::Error;

    #[test]
    fn test_get_html() {
        assert_eq!(Error::new("Something"), get_page_html("bad-url"));
    }
}
