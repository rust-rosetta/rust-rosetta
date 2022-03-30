use rand::{distributions::Uniform, thread_rng, Rng};

use sequence_mutation::Seq;

fn main() {
    let mut seq = Seq::new(vec!["A", "C", "T", "G"], 200);

    println!("Initial sequnce:\n{}", seq);

    let mut_distr = Uniform::new_inclusive(0, 2);

    for _ in 0..10 {
        let mutation = thread_rng().sample(mut_distr);

        match mutation {
            0 => seq.insert(),
            1 => seq.delete(),
            _ => seq.swap(),
        }
    }

    println!("\nMutated sequence:\n{}", seq);
}
