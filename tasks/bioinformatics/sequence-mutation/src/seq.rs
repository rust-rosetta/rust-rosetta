use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

pub struct Seq<'a> {
    alphabet: Vec<&'a str>,
    seq: Vec<&'a str>,
    base_distr: Uniform<usize>,
    pos_distr: Uniform<usize>,
}

impl Seq<'_> {
    pub fn new(alphabet: Vec<&str>, len: usize) -> Seq {
        let base_distr = Uniform::new_inclusive(0, alphabet.len() - 1);
        let pos_distr = Uniform::new_inclusive(0, len - 1);

        let seq = (0..len)
            .map(|_| alphabet[thread_rng().sample(base_distr)])
            .collect();

        Seq {
            alphabet,
            base_distr,
            pos_distr,
            seq,
        }
    }

    pub fn insert(&mut self) {
        let pos = thread_rng().sample(self.pos_distr);
        let base = self.alphabet[thread_rng().sample(self.base_distr)];

        println!("Inserting {} at position {}", base, pos);

        self.seq.insert(pos, base);
    }

    pub fn delete(&mut self) {
        let pos = thread_rng().sample(self.pos_distr);

        println!("Deleting {} at position {}", self.seq[pos], pos);

        self.seq.remove(pos);
    }

    pub fn swap(&mut self) {
        let pos = thread_rng().sample(self.pos_distr);
        let cur_base = self.seq[pos];
        let new_base = self.alphabet[thread_rng().sample(self.base_distr)];

        println!(
            "Replacing {} at position {} with {}",
            cur_base, pos, new_base
        );

        self.seq[pos] = new_base;
    }
}

impl Display for Seq<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let pretty: String = self
            .seq
            .chunks(60)
            .map(|bases| format!("{}\n", bases.join("")))
            .collect();

        let counts_hm = self
            .seq
            .iter()
            .fold(HashMap::<&str, usize>::new(), |mut m, base| {
                *m.entry(base).or_insert(0) += 1;
                m
            });

        let mut counts_vec: Vec<(&str, usize)> = counts_hm.into_iter().collect();

        counts_vec.sort_by(|a, b| a.0.cmp(b.0));

        let counts_string =
            counts_vec
                .iter()
                .fold(String::new(), |mut counts_string, (base, count)| {
                    counts_string += &format!("{} = {}\n", base, count);
                    counts_string
                });

        write!(
            f,
            "Seq:\n{}\n\nLength: {}\n\nCounts:\n{}",
            pretty,
            self.seq.len(),
            counts_string
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_consists_of_alphabet_only() {
        let alphabet = vec!["A", "B"];
        let seq = Seq::new(alphabet.clone(), 5);

        let consists_of_alphabet_only = seq.seq.iter().all(|s| alphabet.contains(s));

        assert!(consists_of_alphabet_only);
    }

    #[test]
    fn length_is_equal_to_given_value() {
        let seq_1 = Seq::new(vec!["A", "B", "C"], 15);
        let seq_2 = Seq::new(vec!["X", "Y", "Z"], 1);
        let seq_3 = Seq::new(vec!["X"], 10);

        assert_eq!(seq_1.seq.len(), 15);
        assert_eq!(seq_2.seq.len(), 1);
        assert_eq!(seq_3.seq.len(), 10);
    }

    #[test]
    fn insertion_increases_length() {
        let mut seq = Seq::new(vec!["A", "B", "C"], 15);
        let prev_length = seq.seq.len();

        seq.insert();

        assert_eq!(seq.seq.len(), prev_length + 1);
    }

    #[test]
    fn deletion_decreases_length() {
        let mut seq = Seq::new(vec!["A", "B", "C"], 20);
        let prev_length = seq.seq.len();

        seq.delete();

        assert_eq!(seq.seq.len(), prev_length - 1);
    }

    #[test]
    #[should_panic]
    fn cannot_make_deletion_on_empty_sequence() {
        let mut seq = Seq::new(vec!["X", "Y", "Z"], 1);

        seq.delete();
        seq.delete();

        assert_eq!(seq.seq.len(), 0);
    }

    #[test]
    fn swapping_does_not_change_length() {
        let mut seq = Seq::new(vec!["X", "Y", "Z"], 1);
        let prev_length = seq.seq.len();

        seq.swap();

        assert_eq!(seq.seq.len(), prev_length);
    }
}
