// Author : Rahul Sharma
// Github : github.com/creativcoder

fn count_sub_string(src: &str, target: &str) -> usize {
    let mut buff = src.to_string();
    if buff.contains(target) {
        buff = buff.replace(target, "");
    }

    (src.len() - buff.len()) / target.len()
}

#[test]
fn test_one() {
    assert_eq!(count_sub_string("adssdadaadaddaadad", "ad"), 6);
}

#[test]
fn test_two() {
    assert_eq!(count_sub_string("rustisawesome", "zz"), 0);
}

fn main() {
    let text = "this is three of the four";
    let sub_str = "th";
    println!("{:?}", count_sub_string(text, sub_str));
}
