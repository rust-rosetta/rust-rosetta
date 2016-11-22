use std::cmp;

fn jaro(str1: &str, str2: &str) -> f64 {
    // lengths of both strings
    let str1_len: usize = str1.len();
    let str2_len: usize = str2.len();

    // if both strings are empty return 1
    // if only one of the strings is empty return 0
    if str1_len == 0 {
        if str2_len == 0 {
            return 1.0;
        } else {
            return 0.0;
        }
    }

    // max distance between two chars to be considered matching
    let match_distance: isize = cmp::max(str1_len, str2_len) as isize / 2 - 1;

    // mutable vectors of bools that signify if that char in the matching string has a match
    let mut str1_matches: Vec<bool> = vec![false; str1_len];
    let mut str2_matches: Vec<bool> = vec![false; str2_len];

    // number of matches and transpositions
    let mut matches: f64 = 0.0;
    let mut transpositions: f64 = 0.0;

    // find the matches
    for (i, str1_match) in str1_matches.iter_mut().enumerate() {
        // cast new variable i_isize for clarity
        let i_isize = i as isize;
        // start and end take into account the match distance
        let start: usize = cmp::max(0, i_isize - match_distance) as usize;
        let end: usize = cmp::min(i_isize + match_distance + 1, str2_len as isize) as usize;

        for (k, str2_match) in str2_matches.iter_mut().enumerate().take(end).skip(start) {
            // if str2 already has a match continue
            if *str2_match {
                continue;
            }
            // if str1 at i and str2 at k are not equal
            if str1.chars().nth(i).unwrap() != str2.chars().nth(k).unwrap() {
                continue;
            }
            // otherwise assume there is a match
            *str1_match = true;
            *str2_match = true;
            matches += 1.0;
            break;
        }
    }

    // if there are no matches return 0
    if matches == 0.0 {
        return 0.0;
    }

    // count transpositions
    let mut k = 0;
    for (i, str1_match) in str1_matches.iter().enumerate() {
        // if there are no matches in str1 continue
        if !str1_match {
            continue;
        }
        // while there is no match in str2 increment k
        while !str2_matches[k] {
            k += 1;
        }
        // increment transpositions
        if str1.chars().nth(i).unwrap() != str2.chars().nth(k).unwrap() {
            transpositions += 1.0;
        }
        k += 1;
    }

    // deallocate variables no longer used
    drop(k);
    drop(str1_matches);
    drop(str2_matches);
    drop(match_distance);

    // divide the number of transpositions by two as per the algorithm specs
    transpositions /= 2.0;

    // return the Jaro distance
    ((matches / str1_len as f64) + (matches / str2_len as f64) +
     ((matches - transpositions) / matches)) / 3.0
}

fn main() {
    println!("{}", jaro("MARTHA", "MARHTA"));
    println!("{}", jaro("DIXON", "DICKSONX"));
    println!("{}", jaro("JELLYFISH", "SMELLYFISH"));
}

#[test]
fn test_jaro() {
    use std::f64;

    assert!((jaro("MARTHA", "MARHTA") - 0.9444444444444445).abs() < f64::EPSILON);
    assert!((jaro("DIXON", "DICKSONX") - 0.7666666666666666).abs() < f64::EPSILON);
    assert!((jaro("JELLYFISH", "SMELLYFISH") - 0.8962962962962964).abs() < f64::EPSILON);
}
