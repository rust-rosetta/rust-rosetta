//Iterator "Successors"
fn main() {
    std::iter::successors(Some((1u128, 0)), |&(a, b)| a.checked_add(b).map(|s| (b, s)))
        .for_each(|(_, u)| println!("{}", u));
}
