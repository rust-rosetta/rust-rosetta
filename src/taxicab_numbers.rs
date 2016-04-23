// http://rosettacode.org/wiki/Taxicab_numbers

use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// A type to represent a pair-sum of cubes.
/// value = a^3 + b^3
#[derive(Copy, Clone, PartialEq, Eq)]
struct SumCubes {
    a: u64,
    b: u64,
    value: u64,
}

impl SumCubes {
    fn new(a: u64, b: u64) -> SumCubes {
        SumCubes {
            value: a.pow(3) + b.pow(3),
            a: a,
            b: b,
        }
    }
}

impl PartialOrd for SumCubes {
    fn partial_cmp(&self, other: &SumCubes) -> Option<Ordering> {
        // Comparison is reversed to make PriorityQueue behave like a min-heap
        (other.value, other.a, other.b).partial_cmp(&(self.value, self.a, self.b))
    }
}

impl Ord for SumCubes {
    fn cmp(&self, other: &SumCubes) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// An iterator through all Taxicab numbers
struct TaxicabNumbers {
    /// Use a BinaryHeap as a priority queue to iterate through sums of
    /// cubes efficiently in increasing order
    pq: BinaryHeap<SumCubes>,
}

impl TaxicabNumbers {
    fn new() -> TaxicabNumbers {
        let mut res = TaxicabNumbers { pq: BinaryHeap::new() };
        res.pq.push(SumCubes::new(1, 1)); // Start with 1^3 + 1^3
        res
    }
}

impl Default for TaxicabNumbers {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for TaxicabNumbers {
    type Item = Vec<SumCubes>;

    fn next(&mut self) -> Option<Vec<SumCubes>> {
        let mut ways = Vec::new();      // All the ways we can express the current
                                        // value as a sum of cubes
        ways.push(SumCubes::new(0, 0)); // Just a sentinel value

        loop {
            let nxt = self.pq.pop().unwrap();
            if nxt.value == ways[0].value {
                // If the next sum is the same as the current one,
                // then we found another way to express the current value
                ways.push(nxt);
            } else if ways.len() > 1 {
                // If we can express the current value in more than one ways,
                // then it's a Taxicab number
                self.pq.push(nxt);
                return Some(ways);
            } else {
                ways.clear();
                ways.push(nxt);
            }

            // Populate the priority queue with more sums
            self.pq.push(SumCubes::new(nxt.a + 1, nxt.b));
            if nxt.a == nxt.b {
                self.pq.push(SumCubes::new(nxt.a + 1, nxt.b + 1));
            }
        }
    }
}

fn main() {
    let numbers = TaxicabNumbers::new();
    for (at, ways) in numbers.take(2006)
        .enumerate()
        .filter(|&(at, _)| at + 1 <= 25 || at + 1 >= 2000) {
        print!("{:>4}:{:>10}", at + 1, ways[0].value);
        for &SumCubes { a, b, .. } in &ways {
            print!(" = {:>4}^3 + {:>4}^3", a, b);
        }
        print!("\n");
    }
}

#[test]
fn test_taxicab_numbers() {
    // A001235 on OEIS
    let seq = [1729u64, 4104, 13832, 20683, 32832, 39312, 40033, 46683, 64232, 65728, 110656,
               110808, 134379, 149389, 165464, 171288, 195841, 216027, 216125, 262656, 314496,
               320264, 327763, 373464, 402597, 439101, 443889, 513000, 513856, 515375, 525824,
               558441, 593047, 684019, 704977];

    for (&expected, ways) in seq.iter().zip(TaxicabNumbers::new()) {
        assert!(ways.len() > 1);
        for &SumCubes { value, .. } in &ways {
            assert_eq!(value, expected);
        }
    }
}
