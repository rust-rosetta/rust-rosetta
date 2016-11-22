//! An iterable version.

/// Define a struct which stores the state for the iterator.
struct HofstadterQ {
    next: usize,
    memoize_vec: Vec<usize>,
}

impl HofstadterQ {
    /// Define a constructor for the struct.
    fn new() -> HofstadterQ {
        HofstadterQ {
            next: 1,
            memoize_vec: vec![1],
        }
    }
}

impl Default for HofstadterQ {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement the hofstadter q iteration sequence.
impl Iterator for HofstadterQ {
    type Item = usize;

    /// This gets called to fetch the next item of the iterator.
    fn next(&mut self) -> Option<usize> {
        // Cache the current value.
        self.memoize_vec.push(self.next);
        // And then calculate the 'next'.
        // First, make the four recursive calls.
        let current: usize = self.memoize_vec.len();
        let rec_call_1: usize = self.memoize_vec[current - 1];
        let rec_call_2: usize = self.memoize_vec[current - 2];
        let rec_call_3: usize = self.memoize_vec[current - rec_call_1];
        let rec_call_4: usize = self.memoize_vec[current - rec_call_2];
        // Then update self.next and return it.
        self.next = rec_call_3 + rec_call_4;
        Some(self.next)
    }
}

fn main() {
    // Set up the iterable.
    let hof: HofstadterQ = HofstadterQ::new();
    // The number of terms we want from the iterator.
    let upto: usize = 1000;
    // Create the iterator.
    let mut it = hof.take(upto - 2);
    // Print the base values.
    println!("H(1) = 1");
    println!("H(2) = 1");
    // Print the rest of the sequence.
    for i in 3..1 + upto {
        println!("H({}) = {}", i, it.next().unwrap());
    }
}

#[test]
fn test_first_ten() {
    let hofstadter_q = HofstadterQ::new().take(10).collect::<Vec<_>>();
    // Test that the first ten values are as expected
    // The first two values are hardcoded, so no need to test those.
    let hofstadter_q_expected = vec![2, 3, 3, 4, 5, 5, 6, 6, 6, 8];
    assert_eq!(hofstadter_q_expected, hofstadter_q);
}

#[test]
fn test_thousandth() {
    // Set up the iterable.
    let hof: HofstadterQ = HofstadterQ::new();
    // The number of terms we want from the iterator.
    let upto: usize = 1000;
    // Create the iterator.
    let mut it = hof.take(upto - 2);
    let expected: usize = 502;
    // Test that the upto-th term is as expected.
    for _ in 3..upto {
        it.next();
    }
    assert_eq!(expected, it.next().unwrap());
}
