fn strip_comments(s: &str) -> &str {
    let markers = ['#', ';'];
    s.find(&markers[..]).map_or(s, |i| &s[..i]).trim()
}

#[test]
fn test_strip_comments() {
    let inputs = [
        "apples, pears # and bananas",
        "apples, pears ; and bananas",
        "  apples, pears ",
    ];
    let output = "apples, pears";

    for &input in &inputs {
        assert_eq!(strip_comments(input), output)
    }
}

fn main() {
    let inputs = [
        "apples, pears # and bananas",
        "apples, pears ; and bananas",
        "  apples, pears ",
    ];

    for &input in &inputs {
        println!("Input: {}\nStripped: {}", input, strip_comments(input))
    }
}
