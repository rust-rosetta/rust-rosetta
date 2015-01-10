// Implements http://rosettacode.org/wiki/Loops/For

fn main() {
    for i in (1us..6) {
        for _ in (1us..i+1) {
            print!("*")
        }
        println!("")
    }
}
