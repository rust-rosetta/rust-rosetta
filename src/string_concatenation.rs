// http://rosettacode.org/wiki/String_concatenation

fn add_world(mut x: String) -> String {
    // world is a &'a[u8]
    let world = " world";
    // The call append() does a vec.push(other) to x.
    // Note that this call implies a move, so we cannot do an in-place append
    // and we cannot continue to use x outside of the body of this function.
    // It is not a copy operation though.
    x.push_str(world);
    x
}

#[cfg(not(test))]
fn main() {
    // The call to_string() turns a &[u8] into a Vec<u8>.
    // This is done because Vecs are growable but slices aren't.
    let hello = "hello".to_string();
    let hello_world = add_world(hello);
    println!("{}", hello_world);
}

#[test]
fn test_string_concat() {
    let super_hoverbear = "super hoverbear".to_string();
    let super_hoverbear_world = add_world(super_hoverbear);
    assert_eq!(super_hoverbear_world, "super hoverbear world".to_string());
}
