//! Error handling.

use std::io;

use reqwest;

error_chain! {
    foreign_links {
        Io(io::Error) #[doc = "I/O Error."];

        Http(reqwest::Error) #[doc = "Network error."];
    }
}
