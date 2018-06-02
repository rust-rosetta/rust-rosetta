// Author : Rahul Sharma
// Github : github.com/creativcoder

// The rest of the `Pattern` and `split` APIs could make this much more powerful
fn count_sub_string(src: &str, target: &str) -> usize {
    src.split(target).count() - 1
}

fn main() {
    let text = "this is three of the four";
    let sub_str = "th";
    println!("{:?}", count_sub_string(text, sub_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing() {
        assert_eq!(count_sub_string("adssdadaadaddaadad", "ad"), 6);
    }

    #[test]
    fn none() {
        assert_eq!(count_sub_string("rustisawesome", "zz"), 0);
    }
}
