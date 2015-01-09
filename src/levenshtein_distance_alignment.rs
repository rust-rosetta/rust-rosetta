// http://rosettacode.org/wiki/Levenshtein_distance/Alignment
use std::uint;
use std::collections::DList;
use std::iter::repeat;

enum Operation { Insert, Delete, Match, }

// Returns the value of a 2D vector given a pair of indexes.
// Returns the default value if indices are out of bounds.
fn get_val(mat: &Vec<Vec<uint>>, r: uint, c: uint, default: uint) -> uint {
    match mat.get(r) {
        Some(col) => {
            match col.get(c) { Some(v) => *v, None => default, }
        }
        None => default,
    }
}
// Implementation of the Needleman–Wunsch algorithm, with modification
// to the scoring method to only allow positive ints.
//
// http://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm
fn levenshtein_distance(s1: &str, s2: &str) -> (uint, String, String) {
    let l1 = s1.len() + 1;
    let l2 = s2.len() + 1;

    let mut mat: Vec<Vec<uint>> = repeat( 
		repeat(0u).take(l2).collect()
		).take(l1).collect();
    for row in range(0u, l1) { mat[row][0] = row; }
    for col in range(0u, l2) { mat[0][col] = col; }
    for row in range(1u, l1) {
        for col in range(1u, l2) {
            mat[row][col] =
                if s1.char_at(row - 1) == s2.char_at(col - 1) {
                    mat[row - 1][col - 1]
                } else {
                    let vals =
                        [mat[row - 1][col] + 1, mat[row][col - 1] + 1,
                         mat[row - 1][col - 1] + 1];
                    *vals.iter().min().unwrap()
                }
        }
    }
    let mut res1: DList<char> = DList::new();
    let mut res2: DList<char> = DList::new();
    let mut cur_row = l1 - 1;
    let mut cur_col = l2 - 1;
    while cur_row > 0 || cur_col > 0 {
        let ins = get_val(&mat, cur_row, cur_col - 1, uint::MAX);
        let del = get_val(&mat, cur_row - 1, cur_col, uint::MAX);
        let sub = get_val(&mat, cur_row - 1, cur_col - 1, uint::MAX);
        let vals =
            vec!(( sub , Operation::Match ) , ( ins , Operation::Insert ) , ( del , Operation::Delete ));
        match vals.into_iter().min_by(|&(x, _)| { x }).unwrap() {
            (_, Operation::Insert) => {
                cur_col -= 1;
                res1.push_front('-');
                res2.push_front(s1.char_at(cur_col));
            }
            (_, Operation::Delete) => {
                cur_row -= 1;
                res1.push_front(s1.char_at(cur_row));
                res2.push_front('-');
            }
            (_, Operation::Match) => {
                cur_row -= 1;
                cur_col -= 1;
                res1.push_front(s1.char_at(cur_row));
                res2.push_front(s2.char_at(cur_col));
            }
        }
    }
    let aligned1: String = res1.into_iter().collect();
    let aligned2: String = res2.into_iter().collect();
    let lev_dist = mat[l1 - 1][l2 - 1];
    
    return (lev_dist, aligned1, aligned2);
}

#[cfg(not(test))]
fn main() { 
    let (s1, s2) = ("rosettacode", "raisethysword"); 
    let (lev_dist, aligned1, aligned2) = levenshtein_distance(s1, s2); 
    println!("Words are: {}, {}" , s1 , s2);
    println!("Levenshtein Distance: {}" , lev_dist);
    println!("{:?}" , aligned1);
    println!("{:?}" , aligned2);

}

#[test]
fn test_lev_distance() {
    let test_results =
        vec!(( "sunday" , "saturday" , (3, "s--unday".to_string(), "sunurday".to_string()))  , 
            ( "sitting" , "kitten" , (3, "sitting".to_string(), "kitten-".to_string())) , 
            ("test" , "test" , (0, "test".to_string(), "test".to_string()) ));
    for (word1, word2, dist) in test_results.into_iter() {
        assert_eq!(levenshtein_distance ( word1 , word2 ) , dist);
    }
}
