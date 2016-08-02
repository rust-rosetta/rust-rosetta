extern crate url;

use url::Url;

fn print_url(url: Url) {
    println!("scheme:      {:?}", url.scheme());
    if let Some(ref host) = url.host() {
        println!("domain:      {:?}", host);
    }
    if let Some(ref port) = url.port() {
        println!("port:        {:?}", port);
    }
    println!("path:        {:?}", url.path());
    if let Some(ref query) = url.query() {
        println!("query:       {:?}", query);
    }
    if let Some(ref fragment) = url.fragment() {
        println!("fragment:    {:?}", fragment);
    }
}

fn main() {
    let urls = vec![
        "foo://example.com:8042/over/there?name=ferret#nose",
        "urn:example:animal:ferret:nose",
        "jdbc:mysql://test_user:ouupppssss@localhost:3306/sakila?profileSQL=true",
        "ftp://ftp.is.co.za/rfc/rfc1808.txt",
        "http://www.ietf.org/rfc/rfc2396.txt#header1",
        "ldap://[2001:db8::7]/c=GB?objectClass=one&objectClass=two",
        "mailto:John.Doe@example.com",
        "news:comp.infosystems.www.servers.unix",
        "tel:+1-816-555-1212",
        "telnet://192.0.2.16:80/",
        "urn:oasis:names:specification:docbook:dtd:xml:4.1.2",
        "ssh://alice@example.com",
        "https://bob:pass@example.com/place",
        "http://example.com/?a=1&b=2+2&c=3&c=4&d=%65%6e%63%6F%64%65%64",
    ];
    for url in urls {
        println!("URL: {}", url);
        print_url(Url::parse(url).unwrap());
        println!("");
    }
}
