struct Hailstone {
  priv current: uint
}

impl Hailstone {
  fn new(start: uint) -> Hailstone {
    Hailstone { current: start }
  }
}

impl Iterator<uint> for Hailstone {
  fn next(&mut self) -> Option<uint> {
    let current = self.current;
    match current {
      0               => None,
      1               => {
        self.current = 0;
        Some(1)
      },
      x if x % 2 == 0 => {
        self.current = x / 2;
        Some(current)
      },
      x if x % 2 == 1 => {
        self.current = (3 * x) + 1;
        Some(current)
      },
      _               => None
    }
  }
}

fn check_hailstone(n: uint) {
  let mut hail = Hailstone::new(n);
  let result: ~[uint] = hail.collect();
  println!("Testing: {}, Length: {}, Values: {}...{}", n, result.len(), result.slice(0,4).to_str(), result.slice(result.len()-4, result.len()).to_str())
}

fn main() {
  check_hailstone(27);
}
