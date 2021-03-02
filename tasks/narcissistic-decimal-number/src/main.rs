fn is_narcissistic(x: u32) -> bool {
    let digits: Vec<u32> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits
        .iter()
        .map(|d| d.pow(digits.len() as u32))
        .sum::<u32>()
        == x
}

fn main() {
    let mut counter = 0;
    let mut i = 0;
    while counter < 25 {
        if is_narcissistic(i) {
            println!("{}", i);
            counter += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::is_narcissistic;
    #[test]
    fn test_is_narc() {
        let input = 0;
        let output = is_narcissistic(input);
        assert_eq!(output, true);

        let input = 1;
        let output = is_narcissistic(input);
        assert_eq!(output, true);

        let input = 153;
        let output = is_narcissistic(input);
        assert_eq!(output, true);
    }
    #[test]
    fn test_not_is_narc() {
        let input = 10;
        let output = is_narcissistic(input);
        assert_eq!(output, false);

        let input = 999999999;
        let output = is_narcissistic(input);
        assert_eq!(output, false);
    }
}
