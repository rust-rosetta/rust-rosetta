// http://rosettacode.org/wiki/Towers_of_Hanoi

fn play(n: i32, from: i32, to: i32, via: i32) {
    if n > 0 {
        play(n - 1, from, via, to);
        println!("Move disk from pole {} to pole {}", from, to);
        play(n - 1, via, to, from);
    }
}

fn main() {
    play(4, 1, 2, 3);
}
