use std::error;
use std::io;

fn get_sum(input: String) -> Result<i32, Box<dyn error::Error>> {
    let numbers = input
        .split_whitespace()
        .map(|number| number.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    if numbers.len() == 2 {
        Ok(numbers.iter().sum())
    } else {
        Err("Please enter 2 integers".into())
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let sum = get_sum(input)?;
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn integer_sum_test() {
        let input = "2 2".to_string();
        let output = get_sum(input);
        assert_eq!(output.unwrap(), 4);

        let input = "3 2".to_string();
        let output = get_sum(input);
        assert_eq!(output.unwrap(), 5);

        let input = "79 -1".to_string();
        let output = get_sum(input);
        assert_eq!(output.unwrap(), 78);

        let input = "-5 10".to_string();
        let output = get_sum(input);
        assert_eq!(output.unwrap(), 5);

        let input = "1000 -1000".to_string();
        let output = get_sum(input);
        assert_eq!(output.unwrap(), 0);
    }

    #[test]
    fn bad_parsing_test() {
        let input = "2 1T".to_string();
        let output = get_sum(input);
        assert_eq!(output.is_err(), true);

        let input = "2 2.4".to_string();
        let output = get_sum(input);
        assert_eq!(output.is_err(), true);
    }

    #[test]
    fn bad_length_test() {
        let input = "2 1 1".to_string();
        let output = get_sum(input);
        assert_eq!(output.is_err(), true);

        let input = "2".to_string();
        let output = get_sum(input);
        assert_eq!(output.is_err(), true);

        let input = "".to_string();
        let output = get_sum(input);
        assert_eq!(output.is_err(), true);
    }
}
