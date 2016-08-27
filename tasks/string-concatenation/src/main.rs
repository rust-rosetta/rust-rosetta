fn add_world(mut x: String) -> String {
    // world is a &'a[u8]
    let world = " world";
    x.push_str(world);
    x
}

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
