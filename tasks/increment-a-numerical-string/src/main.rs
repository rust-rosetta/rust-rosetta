use std::num::ParseIntError;

fn increment(input: &str) -> Result<String, ParseIntError> {
    input.parse::<i64>().map(|num| (num + 1).to_string())
}

fn main() {
    let s = "-1";
    let s2 = increment(s).unwrap();
    println!("{:?}", s2);
}

#[cfg(test)]
mod tests {
    use super::increment;

    #[test]
    fn numeric() {
        assert_eq!(increment("47").unwrap(), "48".to_owned());
    }


    #[test]
    fn not_numeric() {
        assert!(increment("abc").is_err());
    }
}
