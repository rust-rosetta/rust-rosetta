fn lower_case_alphabet() -> Box<Iterator<Item = char>> {
    let ascii_iter = (0..26).map(|x| (x + b'a') as char);
    Box::new(ascii_iter)
}

fn main() {
    println!("{:?}", lower_case_alphabet().collect::<Vec<_>>());
}

#[test]
fn test_alphabet() {
    let expected = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
                        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    assert_eq!(expected, lower_case_alphabet().collect::<Vec<_>>());
}
