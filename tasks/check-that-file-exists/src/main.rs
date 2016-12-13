use std::path::Path;

fn main() {
    let paths = ["input.txt", "docs"];
    for path in paths.iter().map(|&x| Path::new(x)) {
        let msg = if path.exists() {
            "exists"
        } else {
            "does not exist"
        };

        println!("{} {}.", path.display(), msg);
    }
}
