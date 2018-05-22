extern crate avl_tree;
extern crate getopts;
extern crate rand;

use std::env;

use getopts::Options;
use rand::distributions::Uniform;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("?", "help", "Print this help menu");
    opts.optopt("r", "", "Number of nodes in random tree", "<int>");
    opts.optopt("n", "", "Number of random inserts and deletes", "<int>");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("?") {
        let program = &args[0];
        print_usage(program, &opts);
        return;
    }
    let r_nodes = match matches.opt_str("r") {
        None => 100,
        Some(x) => x.parse::<usize>().unwrap(),
    };
    let n = match matches.opt_str("n") {
        None => 0,
        Some(x) => x.parse::<usize>().unwrap(),
    };

    let mut tree = avl_tree::random_bal_tree(r_nodes as u32);
    let mut rng = rand::thread_rng();
    // `Uniform` rather than `gen_range`'s `Uniform::sample_single`
    let key_range = Uniform::new(-(n as i32) / 2, (n as i32) / 2);
    let value_range = Uniform::new(-1.0, 1.0);
    tree.insert_bal(0, rng.sample(value_range));
    for _ in 0..n {
        tree.insert_bal(rng.sample(key_range), rng.sample(value_range));
    }
    let (_, bals) = tree.gather_balances();
    assert!(*bals.iter().max().unwrap() < 2);
    assert!(*bals.iter().min().unwrap() > -2);

    println!(
        "AVL tree after ~{} random inserts and ~{} random deletes, starting with {} nodes:",
        n, n, r_nodes
    );
    println!("{}", tree);
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
