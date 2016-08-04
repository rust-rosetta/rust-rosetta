extern crate rand;

use rand::Rng;

fn knuth_shuffle<T>(v: &mut [T]) {
    let mut rng = rand::thread_rng();
    let l = v.len();

    for n in 0..l {
        let i = rng.gen_range(0, l - n);
        v.swap(i, l - n - 1);
    }
}

fn main() {
    let mut v: Vec<_> = (0..10).collect();

    println!("before: {:?}", v);
    knuth_shuffle(&mut v);
    println!("after:  {:?}", v);
}
