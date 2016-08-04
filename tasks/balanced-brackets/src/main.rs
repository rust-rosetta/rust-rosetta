extern crate rand;

trait Balanced {
    /// Returns true if the brackets are balanced
    fn is_balanced(&self) -> bool;
}

impl<'a> Balanced for str {
    fn is_balanced(&self) -> bool {
        let mut count = 0;

        for bracket in self.chars() {
            let change = match bracket {
                '[' => 1,
                ']' => -1,
                _ => panic!("Strings should only contain brackets"),
            };

            count += change;
            if count < 0 {
                return false;
            }
        }

        count == 0
    }
}

/// Generates random brackets
fn generate_brackets(num: usize) -> String {
    use rand::random;

    (0..num)
        .map(|_| {
            if random() {
                '['
            } else {
                ']'
            }
        })
        .collect()
}

fn main() {
    for i in 0..10 {
        let brackets = generate_brackets(i);

        println!("{}    {}", brackets, brackets.is_balanced())
    }
}

#[test]
fn test_empty_string() {
    assert!("".is_balanced());
}

#[test]
fn test_wrong_brackets() {
    assert!(!"][".is_balanced());
    assert!(!"][][".is_balanced());
    assert!(!"[]][[]".is_balanced());
}

#[test]
fn test_good_brackets() {
    assert!("[]".is_balanced());
    assert!("[][]".is_balanced());
    assert!("[[][]]".is_balanced());
}
