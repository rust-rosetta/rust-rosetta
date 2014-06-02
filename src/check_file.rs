// Implements http://rosettacode.org/wiki/Check_that_file_exists
// not_tested

fn main() {
    let paths = ["input.txt", "docs"];
    for path in paths.iter().map(|&x| Path::new(x)) {
        if path.exists() {
            println!("{} exists.", path.display());
        } else {
            println!("{} does not exist.", path.display());
        }
    }
}
