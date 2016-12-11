use std::collections::HashMap;

fn main() {
    // Note that `HashMap` does not preserve order. If this is important,
    // `std::collections::BTreeMap` is what you want.
    let mut olympic_medals = HashMap::new();
    olympic_medals.insert("United States", (1072, 859, 749));
    olympic_medals.insert("Soviet Union", (473, 376, 355));
    olympic_medals.insert("Great Britain", (246, 276, 284));
    olympic_medals.insert("Germany", (252, 260, 270));
    for (country, medals) in olympic_medals {
        println!("{} has had {} gold medals, {} silver medals, and {} bronze medals",
                 country,
                 medals.0,
                 medals.1,
                 medals.2);
    }
}
