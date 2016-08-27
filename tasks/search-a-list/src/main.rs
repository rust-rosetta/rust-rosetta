fn main() {
    let haystack = vec!["Zig", "Zag", "Wally", "Ronald", "Bush", "Krusty", "Charlie", "Bush",
                        "Boz", "Zag"];

    println!("First occurence of 'Bush' at {:?}",
             haystack.iter().position(|s| *s == "Bush"));
    println!("Last occurence of 'Bush' at {:?}",
             haystack.iter().rposition(|s| *s == "Bush"));
    println!("First occurence of 'Rob' at {:?}",
             haystack.iter().position(|s| *s == "Rob"));
}
