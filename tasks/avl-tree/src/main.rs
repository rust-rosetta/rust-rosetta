#[macro_use]
extern crate structopt;
extern crate avl_tree;
extern crate rand;

use rand::distributions::Uniform;
use rand::Rng;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Number of nodes in the random tree
    #[structopt(short = "r", default_value = "100")]
    nodes: usize,

    /// Number of random inserts and deletes
    #[structopt(short = "n", default_value = "0")]
    operations: usize,
}

fn main() {
    let opt = Opt::from_args();

    let mut tree = avl_tree::random_bal_tree(opt.nodes as u32);
    let mut rng = rand::thread_rng();
    // `Uniform` rather than `gen_range`'s `Uniform::sample_single`
    let key_range = Uniform::new(-(opt.operations as i32) / 2, (opt.operations as i32) / 2);
    let value_range = Uniform::new(-1.0, 1.0);
    tree.insert_bal(0, rng.sample(value_range));
    for _ in 0..opt.operations {
        tree.insert_bal(rng.sample(key_range), rng.sample(value_range));
    }
    let (_, bals) = tree.gather_balances();
    assert!(*bals.iter().max().unwrap() < 2);
    assert!(*bals.iter().min().unwrap() > -2);

    println!(
        "AVL tree after ~{} random inserts and ~{} random deletes, starting with {} nodes:",
        opt.operations, opt.operations, opt.nodes
    );
    println!("{}", tree);
}
