extern crate num;

use num::One;

use std::ops::Add;

struct RangeFinder<'a, T: 'a> {
    index: usize,
    length: usize,
    arr: &'a [T],
}

impl<'a, T> Iterator for RangeFinder<'a, T>
where
    T: PartialEq + Add<T, Output = T> + Copy + One,
{
    type Item = (T, Option<T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.length {
            return None;
        }
        let lo = self.index;
        while self.index < self.length - 1
            && self.arr[self.index + 1] == self.arr[self.index] + T::one()
        {
            self.index += 1
        }
        let hi = self.index;
        self.index += 1;
        if hi - lo > 1 {
            Some((self.arr[lo], Some(self.arr[hi])))
        } else {
            if hi - lo == 1 {
                self.index -= 1
            }
            Some((self.arr[lo], None))
        }
    }
}

impl<'a, T> RangeFinder<'a, T> {
    fn new(a: &'a [T]) -> Self {
        RangeFinder {
            index: 0,
            arr: a,
            length: a.len(),
        }
    }
}

fn main() {
    let n = &[
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    for (i, (lo, hi)) in RangeFinder::new(n).enumerate() {
        if i > 0 {
            print!(", ")
        }
        print!("{}", lo);
        if hi.is_some() {
            print!("-{}", hi.unwrap())
        }
    }
    println!();
}

#[test]
fn test_solution() {
    let n: &[u8] = &[
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    let actual: Vec<(u8, Option<u8>)> = RangeFinder::new(n).collect();
    let expected = &[
        (0, Some(2)),
        (4, None),
        (6, Some(8)),
        (11, None),
        (12, None),
        (14, Some(25)),
        (27, Some(33)),
        (35, Some(39)),
    ];

    assert_eq!(&actual, expected);
}
