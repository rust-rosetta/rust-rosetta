fn main() {
    // only same types can be compared
    // String and String or &str and &str
    // exception: strict equality and inequality also work on &str and String
    let a: &str = "abc";
    let b: String = "Bac".to_owned();

    // Strings are coerced to &str when borrowed and needed
    if a == b {
        println!("The strings are equal")
    }
    if a != b {
        println!("The strings are not equal")
    }
    if a > &b {
        println!("The first string is lexically after the second")
    }
    if a < &b {
        println!("The first string is lexically before the second")
    }
    if a >= &b {
        println!("The first string is not lexically before the second")
    }
    if a <= &b {
        println!("The first string is not lexically after the second")
    }

    // case-insensitives:
    // everything else, create owned Strings, then compare as above
    let a2 = a.to_ascii_uppercase();
    let b2 = b.to_ascii_uppercase();

    // equality
    // this avoids new allocations
    if a.eq_ignore_ascii_case(&b) {
        println!("Both strings are equal when ignoring case")
    }

    if a2 == b2 {
        println!("The strings are equal")
    }
    if a2 != b2 {
        println!("The strings are not equal")
    }
    if a2 > b2 {
        println!("The first string is lexically after the second")
    }
    if a2 < b2 {
        println!("The first string is lexically before the second")
    }
    if a2 >= b2 {
        println!("The first string is not lexically before the second")
    }
    if a2 <= b2 {
        println!("The first string is not lexically after the second")
    }
}
