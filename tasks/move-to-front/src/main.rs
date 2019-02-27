fn main() {
    let examples = vec!["broood", "bananaaa", "hiphophiphop"];
    for example in examples {
        let encoded = encode(example);
        let decoded = decode(&encoded);
        println!(
            "{} encodes to {:?} decodes to {}",
            example, encoded, decoded
        );
    }
}

fn get_symbols() -> Vec<u8> {
    (b'a'..b'z').collect()
}

fn encode(input: &str) -> Vec<usize> {
    input
        .as_bytes()
        .iter()
        .fold((Vec::new(), get_symbols()), |(mut o, mut s), x| {
            let i = s.iter().position(|c| c == x).unwrap();
            let c = s.remove(i);
            s.insert(0, c);
            o.push(i);
            (o, s)
        })
        .0
}

fn decode(input: &[usize]) -> String {
    input
        .iter()
        .fold((Vec::new(), get_symbols()), |(mut o, mut s), x| {
            o.push(s[*x]);
            let c = s.remove(*x);
            s.insert(0, c);
            (o, s)
        })
        .0
        .into_iter()
        .map(|c| c as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn correct_encode() {
        let encoded = encode("broood");
        assert_eq!(encoded, vec![1, 17, 15, 0, 0, 5]);
    }

    #[test]
    fn correct_decode() {
        let decoded = decode(&[1, 17, 15, 0, 0, 5]);
        assert_eq!(decoded, "broood");
    }
}
