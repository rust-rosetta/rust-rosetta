//! Author : Thibault Barbie
//!
//! A simple evolutionary algorithm written in Rust.

extern crate rand;

use rand::distributions::Uniform;
use rand::prelude::*;

fn main() {
    let target = "METHINKS IT IS LIKE A WEASEL";
    let copies = 100;
    let mutation_rate = 1.0 / 20.0; // = 0.05 = 5%

    let mut rng = SmallRng::from_entropy();

    // Generate first sentence, mutating each character
    let start = mutate(&mut rng, target, 1.0); // = 100%

    println!("{}", target);
    println!("{}", start);

    evolve(&mut rng, target, start, copies, mutation_rate);
}

/// Evolution algorithm
///
/// Evolves `parent` to match `target`.  Returns the number of evolutions performed.
fn evolve<R: Rng>(
    rng: &mut R,
    target: &str,
    mut parent: String,
    copies: usize,
    mutation_rate: f64,
) -> usize {
    let mut counter = 0;
    let mut parent_fitness = target.len() + 1;

    loop {
        counter += 1;

        let (best_fitness, best_sentence) = (0..copies)
            .map(|_| {
                // Copy and mutate a new sentence.
                let sentence = mutate(rng, &parent, mutation_rate);
                // Find the fitness of the new mutation
                (fitness(target, &sentence), sentence)
            })
            .min_by_key(|&(f, _)| f) // find the closest mutation to the target
            .unwrap(); // fails if `copies == 0`

        // If the best mutation of this generation is better than `parent` then "the fittest
        // survives" and the next parent becomes the best of this generation.
        if best_fitness < parent_fitness {
            parent = best_sentence;
            parent_fitness = best_fitness;
            println!(
                "{} : generation {} with fitness {}",
                parent, counter, best_fitness
            );

            if best_fitness == 0 {
                return counter;
            }
        }
    }
}

/// Computes the fitness of a sentence against a target string, returning the number of
/// incorrect characters.
fn fitness(target: &str, sentence: &str) -> usize {
    sentence
        .chars()
        .zip(target.chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count()
}

/// Mutation algorithm.
///
/// It mutates each character of a string, according to a `mutation_rate`.
fn mutate<R: Rng>(rng: &mut R, sentence: &str, mutation_rate: f64) -> String {
    let maybe_mutate = |c| {
        if rng.gen_bool(mutation_rate) {
            random_char(rng)
        } else {
            c
        }
    };
    sentence.chars().map(maybe_mutate).collect()
}

/// Generates a random letter or space.
fn random_char<R: Rng>(rng: &mut R) -> char {
    // Something similar to `rand::distributions::Alphanumeric` might be faster.

    // Returns a value in the range [A, Z] + an extra slot for the space character.
    // `Uniform` rather than `gen_range`'s `Uniform::sample_single` for speed
    let range = Uniform::new_inclusive(b'A', b'Z' + 1);
    match rng.sample(range) {
        c if c == b'Z' + 1 => ' ', // the `char` after 'Z'
        c => c as char,
    }
}
