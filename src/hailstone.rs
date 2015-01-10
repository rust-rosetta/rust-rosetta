// Implements http://rosettacode.org/wiki/Hailstone_sequence
#![feature(associated_types)]

// Define a struct which stores the state for the iterator.
struct Hailstone {
    next: uint, // Accessible only to the current module.
    pub start: uint  // Publically accessible.
}

impl Hailstone {
  // Define a constructor for the struct.
    fn new(n: uint) -> Hailstone {
        Hailstone { next: n, start: n }
    }
}

// Implement the hailstone iteration sequence.
impl Iterator for Hailstone {
    type Item = uint;
    // This gets called to fetch the next item of the iterator.
    fn next(&mut self) -> Option<uint> {
        // We need to cache the current value.
        let current = self.next;
        // And then calculate the 'next'
        self.next = match current {
            0 => {
                // Resets the iterator.
                self.next = self.start;
                return None
            },
            // At the end, yield 1 and roll over next time called.
            1 => 0,
            // Got an even.
            x if x % 2 == 0 => x / 2,
            // All remaining numbers are odd.
            x => (3 * x) + 1,
        };

        Some(current)
    }
}

/// Returns the start number and length of the longest hailstone sequence up to `limit`
fn biggest_hailstone(limit: usize) -> (usize, usize) {
    (0us..limit).map(|n| (n, Hailstone::new(n).count()))
                    .max_by(|&(_, count)| count)
                    .unwrap()
}

#[cfg(not(test))]
fn main() {
    // Find the hailstone for 27.
    let two_seven = Hailstone::new(27).collect::<Vec<uint>>();
    let ts_len = two_seven.len();
    println!("Testing: {}, Length: {}, Values: {:?}...{:?}",
            two_seven[0],
            ts_len,
            two_seven.slice(0, 4),
            two_seven.slice(ts_len - 4, ts_len));

    // Find the longest.
    let (biggest, length) = biggest_hailstone(100000);
    println!("Largest: {}, Size: {}", biggest, length);
}

#[test]
fn test_27() {
    let seq = Hailstone::new(27).collect::<Vec<uint>>();

    assert_eq!(seq.slice(0, 4), [27, 82, 41, 124]);
    assert_eq!(seq.slice(seq.len() - 4, seq.len()), [8, 4, 2, 1]);
}

#[test]
fn test_biggest() {
    let (biggest, length) = biggest_hailstone(100000);
    assert_eq!(biggest, 77031);
    assert_eq!(length, 351);
}
