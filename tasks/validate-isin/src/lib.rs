extern crate luhn_test_of_credit_card_numbers;

use luhn_test_of_credit_card_numbers::luhn_test;

fn validate_isin(isin: &str) -> bool {
    if !isin.chars().all(|x| x.is_alphanumeric()) || isin.len() != 12 {
        return false;
    }
    if !isin[..2].chars().all(|x| x.is_alphabetic())
        || !isin[2..12].chars().all(|x| x.is_alphanumeric())
        || !isin.chars().last().unwrap().is_numeric()
    {
        return false;
    }

    let bytes = isin.as_bytes();

    let s2 = bytes
        .iter()
        .flat_map(|&c| {
            if c.is_ascii_digit() {
                vec![c]
            } else {
                (c + 10 - ('A' as u8)).to_string().into_bytes()
            }
        })
        .collect::<Vec<u8>>();

    let string = std::str::from_utf8(&s2).unwrap();
    let number = string.parse::<u64>().unwrap();

    return luhn_test(number as u64);
}

#[cfg(test)]
mod tests {
    use super::validate_isin;

    #[test]
    fn test_validate_isin() {
        assert_eq!(validate_isin("US0378331005"), true);
        assert_eq!(validate_isin("US0373831005"), false);
        assert_eq!(validate_isin("U50378331005"), false);
        assert_eq!(validate_isin("US03378331005"), false);
        assert_eq!(validate_isin("AU0000XVGZA3"), true);
        assert_eq!(validate_isin("AU0000VXGZA3"), true);
        assert_eq!(validate_isin("FR0000988040"), true);
    }
}
