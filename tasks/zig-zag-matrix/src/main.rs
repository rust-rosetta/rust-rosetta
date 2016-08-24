//! Implements with the sorting indexes algorithm explained in the [discussion page].
//!
//! [discussion page]:http://rosettacode.org/wiki/Talk:Zig-zag_matrix
use std::iter::repeat;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Debug, PartialEq, Eq)]
struct SortIndex {
    x: usize,
    y: usize,
}

impl SortIndex {
    fn new(x: usize, y: usize) -> SortIndex {
        SortIndex { x: x, y: y }
    }
}

impl PartialOrd for SortIndex {
    fn partial_cmp(&self, other: &SortIndex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SortIndex {
    fn cmp(&self, other: &SortIndex) -> Ordering {
        let lower = if self.x + self.y == other.x + other.y {
            if (self.x + self.y) % 2 == 0 {
                self.x < other.x
            } else {
                self.y < other.y
            }
        } else {
            (self.x + self.y) < (other.x + other.y)
        };

        if lower {
            Less
        } else if self == other {
            Equal
        } else {
            Greater
        }
    }
}

fn zigzag(n: usize) -> Vec<Vec<usize>> {
    let mut l: Vec<SortIndex> = (0..n * n).map(|i| SortIndex::new(i % n, i / n)).collect();
    l.sort();

    let init_vec = vec![0; n];
    let mut result: Vec<Vec<usize>> = repeat(init_vec).take(n).collect();
    for (i, &SortIndex { x, y }) in l.iter().enumerate() {
        result[y][x] = i
    }
    result
}

fn main() {
    println!("{:?}", zigzag(5));
}

#[test]
fn result() {
    let exp = vec![vec![0, 1, 5, 6, 14],
                   vec![2, 4, 7, 13, 15],
                   vec![3, 8, 12, 16, 21],
                   vec![9, 11, 17, 20, 22],
                   vec![10, 18, 19, 23, 24]];
    assert_eq!(zigzag(5), exp);
}
