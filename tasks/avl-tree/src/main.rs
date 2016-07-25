extern crate avl_tree;
extern crate getopts;
extern crate rand;

use std::env;

use getopts::Options;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut r_nodes = 100;
    let mut n = 0;

    let mut opts = Options::new();
    opts.optflag("?", "help", "Print this help menu");
    opts.optopt("r", "", "Number of nodes in random tree", "<int>");
    opts.optopt("n", "", "Number of random inserts and deletes", "<int>");

    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("?") {
        print_usage(&program, opts);
        return;
    }
    match matches.opt_str("r") {
        None => {}
        Some(x) => r_nodes = x.parse::<usize>().unwrap(),
    };
    match matches.opt_str("n") {
        None => {}
        Some(x) => n = x.parse::<usize>().unwrap(),
    };

    let mut tree = avl_tree::random_bal_tree(r_nodes as u32);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        tree.insert_bal(rng.gen_range(-(r_nodes as i32) / 2, (r_nodes as i32) / 2),
                        rng.gen_range(-1f32, 1f32));
        tree.delete_bal(rng.gen_range(-(r_nodes as i32) / 2, (r_nodes as i32) / 2));
    }
    let res = tree.gather_balances();
    let (_, bals) = res;
    assert!(bals.iter().max().unwrap() < &2);
    assert!(bals.iter().min().unwrap() > &-2);

    println!("AVL tree after ~{} random inserts and ~{} random deletes, starting with {} nodes:",
             n,
             n,
             r_nodes);
    println!("{}", tree);

}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
