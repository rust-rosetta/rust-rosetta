extern crate rand;

use rand::Rng;

// Since you're probably already using the `rand` crate, you should just use
// `rand::Rng::shuffle`.
fn knuth_shuffle<T>(rng: &mut impl Rng, v: &mut [T]) {
    let l = v.len();

    for n in 0..l {
        let i = rng.gen_range(0, l - n);
        v.swap(i, l - n - 1);
    }
}

fn main() {
    let mut v: Vec<_> = (0..10).collect();
    let mut rng = rand::thread_rng();

    println!("before: {:?}", v);
    knuth_shuffle(&mut rng, &mut v);
    println!("after:  {:?}", v);
}
