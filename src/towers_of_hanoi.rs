// http://rosettacode.org/wiki/Towers_of_Hanoi
// not_tested

fn play(n: int, from: int, to: int, via: int) {
    if n > 0 {
        play(n - 1, from, via, to);
        println!("Move disk from pole {:d} to pole {:d}", from, to);
        play(n - 1, via, to, from);
    }
}

fn main() {
    play(4, 1, 2, 3);
}
