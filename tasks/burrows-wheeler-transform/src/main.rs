use core::cmp::Ordering;

const STX: char = '\u{0002}';
const ETX: char = '\u{0003}';

pub fn special_cmp(lhs: &str, rhs: &str) -> Ordering {
    let mut iter1 = lhs.chars();
    let mut iter2 = rhs.chars();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    let is_lhs_alnum = lhs.is_alphanumeric();
                    let is_rhs_alnum = rhs.is_alphanumeric();

                    let result = if is_lhs_alnum == is_rhs_alnum {
                        lhs.cmp(&rhs)
                    } else if is_lhs_alnum {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };

                    return result;
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return lhs.cmp(&rhs),
        }
    }
}

fn burrows_wheeler_transform(input: &str) -> String {
    let mut table: Vec<String> = vec![];

    // add markers for the start and end
    let input_string = format!("{}{}{}", STX, input, ETX);

    // create all possible rotations
    for (i, _) in input_string.char_indices() {
        table.push(format!(
            "{}{}",
            &input_string[input_string.len() - 1 - i..],
            &input_string[0..input_string.len() - 1 - i]
        ));
    }

    // sort rows alphabetically
    table.sort_unstable_by(|lhs, rhs| special_cmp(&lhs, &rhs));

    // return the last column
    table
        .iter()
        .map(|s| s.chars().nth_back(0).unwrap())
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>()
}

fn inverse_burrows_wheeler_transform(input: &str) -> String {
    let mut table: Vec<String> = vec![String::new(); input.len()];
    for _ in 0..input.len() {
        // insert the charatcers of the encoded input as a first column for each row
        for (j, s) in table.iter_mut().enumerate() {
            *s = format!("{}{}", input.chars().nth(j).unwrap(), s);
        }

        // sort rows alphabetically
        table.sort_unstable_by(|lhs, rhs| special_cmp(&lhs, &rhs));
    }

    // return the row which has the end marker at the last position
    table
        .into_iter()
        .filter(|s| s.chars().nth_back(0).unwrap().eq(&ETX))
        .collect::<String>()
        // remove start and markers
        .replace(STX, "")
        .replace(ETX, "")
}

fn main() {
    let input = [
        "banana",
        "SIX.MIXED.PIXIES.SIFT.SIXTY.PIXIE.DUST.BOXES",
        "TO BE OR NOT TO BE OR WANT TO BE OR NOT?",
    ];
    for s in input.iter() {
        let bwt = burrows_wheeler_transform(s);
        let ibwt = inverse_burrows_wheeler_transform(&bwt);
        println!("Input: {}", s);
        println!("\tBWT: {}", bwt.replace(STX, "^").replace(ETX, "|"));
        println!("\tInverse BWT: {}", ibwt);
    }
}

#[cfg(test)]
mod tests {
    use super::{burrows_wheeler_transform, inverse_burrows_wheeler_transform};
    #[test]
    fn test_bwt() {
        let input = "banana";
        let output = burrows_wheeler_transform(input);
        assert_eq!("bnn\u{0002}aa\u{0003}a", output);
    }

    #[test]
    fn test_ibwt() {
        let input = "bnn\u{0002}aa\u{0003}a";
        let output = inverse_burrows_wheeler_transform(input);
        assert_eq!("banana", output);
    }
}
