extern crate permutohedron;
extern crate rand;

use std::cmp::{min, Ordering};
use std::env;
use std::str;

use rand::{thread_rng, Rng};

const WORDS: &[&str] = &["abracadabra", "seesaw", "elk", "grrrrrr", "up", "a"];

#[derive(Eq)]
struct Solution {
    original: String,
    shuffled: String,
    score: usize,
}

// Ordering trait implementations are only needed for the permutations method
impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Solution) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Solution) -> bool {
        self.score == other.score
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Solution) -> Ordering {
        self.score.cmp(&other.score)
    }
}

fn _help() {
    println!("Usage: best_shuffle <word1> <word2> ...");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let words = match args.len() {
        1 => WORDS.iter().map(|s| s.to_string()).collect(),
        _ => args[1..].to_vec(),
    };

    let solutions = words.iter().map(|w| best_shuffle(w));

    for s in solutions {
        println!("{}, {}, ({})", s.original, s.shuffled, s.score);
    }
}

// Implementation iterating over all permutations
fn _best_shuffle_perm(w: &str) -> Solution {
    let w = String::from(w);

    let mut soln = Solution {
        original: w.clone(),
        shuffled: w.clone(),
        score: w.len(),
    };
    let w_bytes: Vec<u8> = w.clone().into_bytes();
    let mut permutocopy = w_bytes.clone();
    let mut permutations = permutohedron::Heap::new(&mut permutocopy);
    while let Some(p) = permutations.next_permutation() {
        let hamm = hamming(&w_bytes, p);
        soln = min(
            soln,
            Solution {
                original: w.clone(),
                shuffled: String::from(str::from_utf8(p).unwrap()),
                score: hamm,
            },
        );
        // Accept the solution if score 0 found
        if hamm == 0 {
            break;
        }
    }
    soln
}

// Quadratic implementation
fn best_shuffle(w: &str) -> Solution {
    let w = String::from(w);

    let w_bytes: Vec<u8> = w.clone().into_bytes();
    let mut shuffled_bytes: Vec<u8> = w.clone().into_bytes();

    // Shuffle once
    let sh: &mut [u8] = shuffled_bytes.as_mut_slice();
    thread_rng().shuffle(sh);

    // Swap wherever it doesn't decrease the score
    for i in 0..sh.len() {
        for j in 0..sh.len() {
            if (i == j) | (sh[i] == w_bytes[j]) | (sh[j] == w_bytes[i]) | (sh[i] == sh[j]) {
                continue;
            }
            sh.swap(i, j);
            break;
        }
    }

    let res = String::from(str::from_utf8(sh).unwrap());
    let res_bytes: Vec<u8> = res.clone().into_bytes();
    Solution {
        original: w.clone(),
        shuffled: res,
        score: hamming(&w_bytes, &res_bytes),
    }
}

fn hamming(w0: &[u8], w1: &[u8]) -> usize {
    w0.iter().zip(w1.iter()).filter(|&(a, b)| a == b).count()
}

#[cfg(test)]
mod tests {
    use super::{_best_shuffle_perm, best_shuffle};

    #[test]
    fn test_best_shuffle_perm() {
        let mut s0 = _best_shuffle_perm("seesaw");
        assert_eq!(s0.score, 0);

        s0 = _best_shuffle_perm("elk");
        assert_eq!(s0.score, 0);

        s0 = _best_shuffle_perm("grrrrrr");
        assert_eq!(s0.score, 5);

        s0 = _best_shuffle_perm("up");
        assert_eq!(s0.shuffled, "pu");
        assert_eq!(s0.score, 0);

        s0 = _best_shuffle_perm("a");
        assert_eq!(s0.shuffled, "a");
        assert_eq!(s0.score, 1);
    }

    #[test]
    fn test_best_shuffle() {
        let mut s0 = best_shuffle("abracadabra");
        assert_eq!(s0.score, 0);

        s0 = best_shuffle("seesaw");
        assert_eq!(s0.score, 0);

        s0 = best_shuffle("elk");
        assert_eq!(s0.score, 0);

        s0 = best_shuffle("grrrrrr");
        assert_eq!(s0.score, 5);

        s0 = best_shuffle("up");
        assert_eq!(s0.shuffled, "pu");
        assert_eq!(s0.score, 0);

        s0 = best_shuffle("a");
        assert_eq!(s0.shuffled, "a");
        assert_eq!(s0.score, 1);
    }
}
