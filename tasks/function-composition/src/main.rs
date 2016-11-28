fn main() {
    use std::f32::consts;

    // the two functions we will compose:
    let f = |x: u32| x.to_string();
    let g = |x: f32| x as u32;

    // their composition
    let comp = compose(f, g);

    println!("{:?}", (*comp)(consts::PI));
}

fn compose<'a, F, G, A, B, C>(f: F, g: G) -> Box<Fn(A) -> C + 'a>
    where G: Fn(A) -> B + 'a,
          F: Fn(B) -> C + 'a
{
    Box::new(move |a: A| f(g(a)))
}

#[test]
fn test_compose() {
    fn inc(x: usize) -> usize {
        x + 1
    }
    fn mul(x: usize) -> usize {
        x * 3
    }

    let comp = compose(inc, mul);
    assert_eq!((*comp)(3), 10);
}
