// Implements http://rosettacode.org/wiki/Longest_common_subsequence
#![feature(collections)]


/// Returns the longest common subsequence of a and b.
fn longest_common_subsequence(a: &str, b: &str) -> String {
    let mut lengths = vec![vec![0; b.len() + 1]; a.len() + 1];
    
    for i in 0..a.len() {
        for j in 0..b.len() {
            if a.char_at(i) == b.char_at(j) {
                lengths[i + 1][j + 1] = lengths[i][j] + 1;
            } else {
                lengths[i + 1][j + 1] = std::cmp::max(
                        lengths[i + 1][j], lengths[i][j + 1]);
            }
        }
    }

    let mut lcs = String::new();
    let mut i = a.len();
    let mut j = b.len();

    while i != 0 && j != 0 {
        if lengths[i][j] == lengths[i - 1][j] {
            i -= 1;
        } else if lengths[i][j] == lengths[i][j - 1] {
            j -= 1;
        } else {
            lcs.push(a.char_at(i - 1));
            i -= 1;
            j -= 1;
        }
    }

    lcs.chars().rev().collect()
}


// Demonstration code
#[cfg(not(test))]
fn main() {
    println!("{}", longest_common_subsequence("abc", "abcd"));
    println!("{}", longest_common_subsequence("ABCDGH", "AEDFHR"));
    println!("{}", longest_common_subsequence("AGGTAB", "GXTXAYB"));
    println!("{}", longest_common_subsequence("", "abcdefg"));
    println!("{}", longest_common_subsequence("abc", ""));
    println!("{}", longest_common_subsequence("abcdefg", "abc"));
    println!("{}", longest_common_subsequence("aaaa", "aaaaaa"));
}


#[test]
fn test_longest_common_subsequence() {
    assert_eq!(longest_common_subsequence("abc", "abcdefg"), "abc");
    assert_eq!(longest_common_subsequence("ABCDGH", "AEDFHR"), "ADH");
    assert_eq!(longest_common_subsequence("AGGTAB", "GXTXAYB"), "GTAB");
    assert_eq!(longest_common_subsequence("", "abcdefg"), "");
    assert_eq!(longest_common_subsequence("abc", ""), "");
    assert_eq!(longest_common_subsequence("abcdefg", "abc"), "abc");
    assert_eq!(longest_common_subsequence("aaaa", "aaaaaa"), "aaaa");
}
