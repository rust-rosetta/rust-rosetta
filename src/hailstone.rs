// Implements http://rosettacode.org/wiki/Hailstone_sequence

// Define a struct which we can make into an interator.
struct Hailstone {
  current: uint, // Accessible online only to itself.
  start:   uint  // Accessible to everything.
}

// Define a constructor for the struct.
impl Hailstone {
  fn new(n: uint) -> Hailstone {
    Hailstone { current: n, start: n }
  }
}

// Implement the hailstone iteration sequence.
impl Iterator<uint> for Hailstone {
  // This gets called to fetch the next item of the iterator.
  fn next(&mut self) -> Option<uint> {
    let current = self.current;    // We need to cache the current value.
    match current {
      0               => {
        // Resets the iterator.
        self.current = self.start;
        None
      },
      1               => {
        // At the end, yield 1 and roll over next time called.
        self.current = 0;
        Some(1)
      },
      x if x % 2 == 0 => {
        // Got an even.
        self.current = x / 2;
        Some(current)
      },
      x               => {
        // All remaining numbers are odd.
        self.current = (3 * x) + 1;
        Some(current)
      },
    }
  }
}

// Returns the number and length of the longest hailstone sequence up to `limit`
fn biggest_hailstone(limit: uint) -> (uint, uint) {
    let mut biggest = range(0u, limit).map(Hailstone::new)
                                      .max_by(|&mut x| x.len())
                                      .unwrap();
    (biggest.start, biggest.len())
}

#[cfg(not(test))]
fn main() {
  // Find the hailstone for 27.
  let two_seven: Vec<uint> = Hailstone::new(27).collect();
  let ts_len = two_seven.len();
  println!("Testing: {}, Length: {}, Values: {}...{}",
          two_seven.get(0),
          ts_len,
          two_seven.slice(0, 4),
          two_seven.slice(ts_len - 4, ts_len));

  // Find the longest.
  let (biggest, length) = biggest_hailstone(100000);
  println!("Largest: {}, Size: {}", biggest, length);
}

#[test]
fn test_27() {
    let seq: Vec<uint> = Hailstone::new(27).collect();

    assert_eq!(seq.slice(0, 4), &[27, 82, 41, 124]);
    assert_eq!(seq.slice(seq.len() - 4, seq.len()), &[8, 4, 2, 1]);
}

#[test]
fn test_biggest() {
    let (biggest, length) = biggest_hailstone(100000);
    assert_eq!(biggest, 77031);
    assert_eq!(length, 351);
}
