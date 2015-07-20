// http://rosettacode.org/wiki/Equilibrium_index
extern crate num;

use num::traits::Zero;

fn equilibrium_indices(v: &[i32]) -> Vec<usize> {
    let mut right = v.iter().fold(0, |a, &b| a + b);
    let mut left = i32::zero();

    v.iter().enumerate().fold(vec![], |mut out, (i, &el)| {
        // NOTE: -= and += doesn't work on left/right and el for some reason.
        right = right - el;
        if left == right {
            out.push(i);
        }
        left = left + el;

        out
    })
}

#[cfg(not(test))]
fn main() {
    let v = [-7i32, 1, 5, 2, -4, 3, 0];
    let indices = equilibrium_indices(&v);
    println!("Equilibrium indices for {:?} are: {:?}", v, indices);
}

#[test]
fn test_equilibrium_indices() {
    let v = &[-7i32, 1, 5, 2, -4, 3, 0];
    assert_eq!(equilibrium_indices(v), vec![3, 6]);
}
