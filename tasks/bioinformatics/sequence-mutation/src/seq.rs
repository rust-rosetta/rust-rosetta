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
