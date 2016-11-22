#[cfg_attr(feature="clippy", allow(needless_range_loop))]
fn levenshtein_distance(word1: &str, word2: &str) -> usize {
    let word1_length = word1.len() + 1;
    let word2_length = word2.len() + 1;

    let mut matrix = vec![vec![0]];

    for i in 1..word1_length {
        matrix[0].push(i);
    }
    for j in 1..word2_length {
        matrix.push(vec![j]);
    }

    for j in 1..word2_length {
        for i in 1..word1_length {
            let x: usize = if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                matrix[j - 1][i - 1]
            } else {
                let min_distance = [matrix[j][i - 1], matrix[j - 1][i], matrix[j - 1][i - 1]];
                *min_distance.iter().min().unwrap() + 1
            };

            matrix[j].push(x);
        }
    }

    matrix[word2_length - 1][word1_length - 1]
}

fn main() {
    println!("{}", levenshtein_distance("kitten", "sitting"));
    println!("{}", levenshtein_distance("saturday", "sunday"));
    println!("{}", levenshtein_distance("rosettacode", "raisethysword"));
}

#[test]
fn test_levenshtein_distance() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
}
