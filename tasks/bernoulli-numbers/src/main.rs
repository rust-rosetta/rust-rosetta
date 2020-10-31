use std::cmp::max;
use std::env;
use std::process;

use bernoulli_numbers::Context;

fn help() {
    println!("Usage: bernoulli_numbers <up_to>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut up_to: usize = 60;

    match args.len() {
        1 => {}
        2 => {
            up_to = args[1].parse::<usize>().unwrap();
        }
        _ => {
            help();
            process::exit(0);
        }
    }

    let context = Context::new();
    // Collect the solutions by using the Context iterator
    // (this is not as fast as calling the optimized function directly).
    let res = context.take(up_to + 1).collect::<Vec<_>>();
    let width = res
        .iter()
        .fold(0, |a, r| max(a, r.value.numer().to_string().len()));

    for r in res.iter().filter(|r| r.index % 2 == 0) {
        println!(
            "B({:>2}) = {:>2$} / {denom}",
            r.index,
            r.value.numer(),
            width,
            denom = r.value.denom()
        );
    }
}
