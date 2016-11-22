/// Returns the longest common subsequence of a and b.
fn longest_common_subsequence(a: &str, b: &str) -> String {
    let a_chars: Vec<_> = a.chars().collect();
    let b_chars: Vec<_> = b.chars().collect();

    let mut lengths = vec![vec![0; b_chars.len() + 1]; a_chars.len() + 1];

    for (i, a_char) in a_chars.iter().enumerate() {
        for (j, b_char) in b_chars.iter().enumerate() {
            if a_char == b_char {
                lengths[i + 1][j + 1] = lengths[i][j] + 1;
            } else {
                lengths[i + 1][j + 1] = std::cmp::max(lengths[i + 1][j], lengths[i][j + 1]);
            }
        }
    }

    let mut lcs = Vec::new();
    let mut i = a_chars.len();
    let mut j = b_chars.len();

    while i != 0 && j != 0 {
        if lengths[i][j] == lengths[i - 1][j] {
            i -= 1;
        } else if lengths[i][j] == lengths[i][j - 1] {
            j -= 1;
        } else {
            lcs.push(a_chars[i - 1]);
            i -= 1;
            j -= 1;
        }
    }

    lcs.into_iter().rev().collect()
}

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

#[test]
fn test_unicode() {
    assert_eq!(longest_common_subsequence("самолетов была отмечена в Японском",
                                          "отмечена в"),
               "отмечена в");
}
