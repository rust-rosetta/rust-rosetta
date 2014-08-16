// http://rosettacode.org/wiki/Taxicab_numbers
// not_tested

use std::collections::PriorityQueue;
use std::num::pow;

/// A type to represent a pair-sum of cubes.
/// value = a^3 + b^3
#[deriving(Eq, Ord)]
struct SumCubes {
    a: u64,
    b: u64,
    value: u64
}

impl SumCubes {
    fn new(a: u64, b: u64) -> SumCubes {
        SumCubes{ a: a, b: b, value: pow(a, 3) + pow(b, 3) }
    }
}

impl PartialEq for SumCubes {
    fn eq(&self, other: &SumCubes) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for SumCubes {
    fn partial_cmp(&self, other: &SumCubes) -> Option<Ordering> {
        // Comparison is reversed to make PriorityQueue behave like a min-heap
        other.value.partial_cmp(&self.value)
    }
}

fn main() {

    // Go through all pair-sums of cubes in increasing order

    // Use a priority queue to do it efficiently
    let mut pq = PriorityQueue::new();

    // Start with 1^3 + 1^3
    pq.push(SumCubes::new(1, 1));

    let mut at: uint = 1;           // The number of the result we're currently at
    let mut ways = Vec::new();      // All the ways we can express the current
                                    // value as a sum of cubes
    ways.push(SumCubes::new(0, 0)); // Just a sentinel value

    loop {
        let cur = pq.pop().unwrap();

        if cur.value == ways[0].value {
            // If the current sum is the same as the last one,
            // then we found another way to express the current value
            ways.push(cur);
        } else {
            if ways.len() > 1 {
                // If we can express the last value in more than one ways
                if (1 <= at && at <= 25) || (2000 <= at && at <= 2006) {

                    // Then output it
                    print!("{:>4u}:{:>10u}", at, ways[0].value);
                    for &SumCubes{ a, b, value } in ways.iter() {
                        print!(" = {:>4u}^3 + {:>4u}^3", a, b);
                    }

                    print!("\n");
                }

                if at == 2006 {
                    break;
                }

                at += 1;
            }

            ways.clear();
            ways.push(cur);
        }

        // Populate the priority queue with more sums
        pq.push(SumCubes::new(cur.a+1, cur.b));
        if cur.a == cur.b {
            pq.push(SumCubes::new(cur.a+1, cur.b+1));
        }
    }
}

