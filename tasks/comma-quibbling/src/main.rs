fn quibble(seq: &[&str]) -> String {
    match seq.len() {
        0 => "{}".to_string(),
        1 => format!("{{{}}}", seq[0]),
        _ => {
            format!("{{{} and {}}}",
                    seq[..seq.len() - 1].join(", "),
                    seq.last().unwrap())
        }
    }
}

fn main() {
    println!("{}", quibble(&[]));
    println!("{}", quibble(&["ABC"]));
    println!("{}", quibble(&["ABC", "DEF"]));
    println!("{}", quibble(&["ABC", "DEF", "G", "H"]));
}

#[test]
fn output() {
    assert_eq!(quibble(&[]), "{}");
    assert_eq!(quibble(&["ABC"]), "{ABC}");
    assert_eq!(quibble(&["ABC", "DEF"]), "{ABC and DEF}");
    assert_eq!(quibble(&["ABC", "DEF", "G", "H"]), "{ABC, DEF, G and H}");
}
