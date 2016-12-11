use std::usize;
use std::collections::VecDeque;
use std::iter::repeat;

/// Returns the value of a 2D vector given a pair of indexes.
/// Returns the default value if indices are out of bounds.
fn get_val(mat: &[Vec<usize>], r: usize, c: usize, default: usize) -> usize {
    match mat.get(r) {
        Some(col) => {
            match col.get(c) {
                Some(v) => *v,
                None => default,
            }
        }
        None => default,
    }
}

/// Implementation of the [Needleman–Wunsch algorithm], with modification
/// to the scoring method to only allow positive ints.
///
/// [Needleman-Wunsch algorithm]: http://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm
#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn levenshtein_distance(s1: &str, s2: &str) -> (usize, String, String) {
    let l1 = s1.len() + 1;
    let l2 = s2.len() + 1;

    let mut mat: Vec<Vec<usize>> = repeat(repeat(0).take(l2).collect())
        .take(l1)
        .collect();
    for row in 0..l1 {
        mat[row][0] = row;
    }
    for col in 0..l2 {
        mat[0][col] = col;
    }
    for row in 1..l1 {
        for col in 1..l2 {
            mat[row][col] = if s1.chars().nth(row - 1).unwrap() ==
                               s2.chars().nth(col - 1).unwrap() {
                mat[row - 1][col - 1]
            } else {
                let vals = [mat[row - 1][col] + 1,
                            mat[row][col - 1] + 1,
                            mat[row - 1][col - 1] + 1];
                *vals.iter().min().unwrap()
            }
        }
    }
    let mut res1: VecDeque<char> = VecDeque::new();
    let mut res2: VecDeque<char> = VecDeque::new();
    let mut cur_row = l1 - 1;
    let mut cur_col = l2 - 1;
    while cur_row > 0 || cur_col > 0 {
        let ins = get_val(&mat, cur_row, cur_col - 1, usize::MAX);
        let del = get_val(&mat, cur_row - 1, cur_col, usize::MAX);
        let sub = get_val(&mat, cur_row - 1, cur_col - 1, usize::MAX);
        let min_val = [sub, ins, del];
        let min_val = min_val.into_iter().min().unwrap();
        if *min_val == sub {
            cur_row -= 1;
            cur_col -= 1;
            res1.push_front(s1.chars().nth(cur_row).unwrap());
            res2.push_front(s2.chars().nth(cur_col).unwrap());
        } else if *min_val == ins {
            cur_col -= 1;
            res1.push_front('-');
            res2.push_front(s1.chars().nth(cur_col).unwrap());
        } else if *min_val == del {
            cur_row -= 1;
            res1.push_front(s1.chars().nth(cur_row).unwrap());
            res2.push_front('-');
        }
    }
    let aligned1: String = res1.into_iter().collect();
    let aligned2: String = res2.into_iter().collect();
    let lev_dist = mat[l1 - 1][l2 - 1];

    (lev_dist, aligned1, aligned2)
}

fn main() {
    let (s1, s2) = ("rosettacode", "raisethysword");
    let (lev_dist, aligned1, aligned2) = levenshtein_distance(s1, s2);
    println!("Words are: {}, {}", s1, s2);
    println!("Levenshtein Distance: {}", lev_dist);
    println!("{}", aligned1);
    println!("{}", aligned2);

}

#[test]
fn test_lev_distance() {
    let test_results = vec![("sunday", "saturday", (3, "s--unday", "sunurday")),
                            ("sitting", "kitten", (3, "sitting", "kitten-")),
                            ("test", "test", (0, "test", "test"))];
    for (word1, word2, dist) in test_results {
        let (d, s1, s2) = levenshtein_distance(word1, word2);
        assert_eq!((d, &s1[..], &s2[..]), dist);
    }
}
