use std::error::Error;
/// Create a program that downloads the time from this URL: http://tycho.usno.navy.mil/cgi-bin/timer.pl
/// and then prints the current UTC time by extracting just the UTC time from the web page's HTML.

fn extract_timestamp(raw_html: &str) -> Option<String> {
    // parse the response body by <BR> tags
    let mut lines = raw_html
        .split("<BR>")
        .filter(|line| match line.find("UTC") {
            Some(_) => true,
            None => false,
        })
        .map(|line| {
            line.trim()
                .replace("UTC\t\tUniversal Time", "")
                .trim()
                .to_string()
        });
    lines.next()
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw_html = reqwest::get("http://tycho.usno.navy.mil/cgi-bin/timer.pl")?.text()?;
    if let Some(timestamp) = extract_timestamp(&raw_html) {
        print!("{}", timestamp);
    } else {
        eprint!("Error: Could not parse URL for getting current time.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extract_timestamp;

    #[test]
    fn test_extract_timestamp() {
        let body = r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final"//EN>
            <html>
            <body>
            <TITLE>What time is it?</TITLE>
            <H2> US Naval Observatory Master Clock Time</H2> <H3><PRE>
            <BR>Aug. 12, 23:55:08 UTC		Universal Time
            <BR>Aug. 12, 07:55:08 PM EDT		Eastern Time
            <BR>Aug. 12, 06:55:08 PM CDT		Central Time
            <BR>Aug. 12, 05:55:08 PM MDT		Mountain Time
            <BR>Aug. 12, 04:55:08 PM PDT		Pacific Time
            <BR>Aug. 12, 03:55:08 PM AKDT	Alaska Time
            <BR>Aug. 12, 01:55:08 PM HAST	Hawaii-Aleutian Time
            </PRE></H3><P><A HREF="http://www.usno.navy.mil"> US Naval Observatory</A>

            </body></html>
            "#;
        assert_eq!(extract_timestamp(&body).unwrap(), "Aug. 12, 23:55:08");
    }
}
