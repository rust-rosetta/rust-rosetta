// Implements http://rosettacode.org/wiki/Balanced_brackets

trait Balanced {
    /// Returns true if the brackets are balanced
    fn is_balanced(&self) -> bool;
}

impl<'a> Balanced for &'a str {
    fn is_balanced(&self) -> bool {
        let mut count = 0i;

        for bracket in self.chars() {
            let change = match bracket {
                '[' => 1,
                ']' => -1,
                _ => panic!("Strings should only contain brackets")
            };

            count += change;
            if count < 0 { return false; }
        }

        count == 0
    }
}

// For convenience this delegates to its slice form
impl Balanced for String {
    fn is_balanced(&self) -> bool { self.as_slice().is_balanced() }
}

/// Generates random brackets
#[cfg(not(test))]
fn generate_brackets(num: usize) -> String {
    use std::rand::random;

    (0..num).map(|_| if random() { '[' } else { ']' }).collect()
}

#[cfg(not(test))]
fn main() {
    for i in range (0us, 10) {
        let brackets = generate_brackets(i);

        println!("{:?}    {:?}", brackets, brackets.is_balanced())
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
