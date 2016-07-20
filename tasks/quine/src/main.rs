fn main() {
    let x = "fn main() {\n    let x = ";
    let y = "print!(\"{}{:?};\n    let y = {:?};\n    {}\", x, x, y, y)\n}\n";
    print!("{}{:?};
    let y = {:?};
    {}", x, x, y, y)
}
