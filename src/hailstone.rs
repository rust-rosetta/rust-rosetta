// Implements http://rosettacode.org/wiki/Hailstone_sequence

// Define a struct which we can make into an interator.
struct Hailstone {
  current: int, // Accessible online only to itself.
  start:   int  // Accessible to everything.
}

// Define a constructor for the struct.
impl Hailstone {
  fn new(n: int) -> Hailstone {
    Hailstone { current: n, start: n }
  }
}

// Implement the hailstone iteration sequence.
impl Iterator<int> for Hailstone {
  // This gets called to fetch the next item of the iterator.
  fn next(&mut self) -> Option<int> {
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
      x if x % 2 == 1 => {
        // Got an odd.
        self.current = (3 * x) + 1;
        Some(current)
      },
      _               => None
    }
  }
}

fn main() {
  // Find the hailstone for 27.
  let two_seven: ~[int] = Hailstone::new(27).collect();
  println!("Testing: {}, Length: {}, Values: {}...{}", two_seven[0], two_seven.len(), two_seven.slice(0,4).to_str(), two_seven.slice(two_seven.len()-4, two_seven.len()).to_str());
  // Find the longest.
  let mut biggest = range(0, 100000).map(|x| Hailstone::new(x)).max_by(|&mut x| x.len()).unwrap();
  let size = biggest.len();
  println!("Largest: {}, Size: {}", biggest.start, size);
}
