use rand::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

pub struct Seq<'a> {
    alphabet: Vec<&'a str>,
    distr: rand::distributions::Uniform<usize>,
    pos_distr: rand::distributions::Uniform<usize>,
    seq: Vec<&'a str>,
}

impl Display for Seq<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let pretty: String = self
            .seq
            .iter()
            .enumerate()
            .map(|(i, nt)| {
                if (i + 1) % 60 == 0 {
                    format!("{}\n", nt)
                } else {
                    nt.to_string()
                }
            })
            .collect();

        let counts_hm = self
            .seq
            .iter()
            .fold(HashMap::<&str, usize>::new(), |mut m, nt| {
                *m.entry(nt).or_default() += 1;
                m
            });

        let mut counts_vec: Vec<(&str, usize)> = counts_hm.into_iter().collect();
        counts_vec.sort_by(|a, b| a.0.cmp(&b.0));
        let counts_string =
            counts_vec
                .iter()
                .fold(String::new(), |mut counts_string, (nt, count)| {
                    counts_string += &format!("{} = {}\n", nt, count);
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

impl Seq<'_> {
    pub fn new(alphabet: Vec<&str>, len: usize) -> Seq {
        let distr = rand::distributions::Uniform::new_inclusive(0, alphabet.len() - 1);
        let pos_distr = rand::distributions::Uniform::new_inclusive(0, len - 1);

        let seq: Vec<&str> = (0..len)
            .map(|_| alphabet[thread_rng().sample(distr)])
            .collect();
        Seq {
            alphabet,
            distr,
            pos_distr,
            seq,
        }
    }

    pub fn insert(&mut self) {
        let pos = thread_rng().sample(self.pos_distr);
        let nt = self.alphabet[thread_rng().sample(self.distr)];
        println!("Inserting {} at position {}", nt, pos);
        self.seq.insert(pos, nt);
    }

    pub fn delete(&mut self) {
        let pos = thread_rng().sample(self.pos_distr);
        println!("Deleting {} at position {}", self.seq[pos], pos);
        self.seq.remove(pos);
    }

    pub fn swap(&mut self) {
        let pos = thread_rng().sample(self.pos_distr);
        let cur_nt = self.seq[pos];
        let new_nt = self.alphabet[thread_rng().sample(self.distr)];
        println!("Replacing {} at position {} with {}", cur_nt, pos, new_nt);
        self.seq[pos] = new_nt;
    }
}

fn main() {
    let mut seq = Seq::new(vec!["A", "C", "T", "G"], 200);
    println!("Initial sequnce:\n{}", seq);

    let mut_distr = rand::distributions::Uniform::new_inclusive(0, 2);

    for _ in 0..10 {
        let mutation = thread_rng().sample(mut_distr);

        if mutation == 0 {
            seq.insert()
        } else if mutation == 1 {
            seq.delete()
        } else {
            seq.swap()
        }
    }

    println!("\nMutated sequence:\n{}", seq);
}
