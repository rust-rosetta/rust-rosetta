/// Encode an usize
fn gray_encode(integer: usize) -> usize {
    (integer >> 1) ^ integer
}

/// Decode an usize
fn gray_decode(integer: usize) -> usize {
    match integer {
        0 => 0,
        _ => integer ^ gray_decode(integer >> 1),
    }
}

fn main() {
    for i in 0..32 {
        println!("{:2} {:0>5} {:0>5} {:2}",
                 i,
                 i,
                 gray_encode(i),
                 gray_decode(i));
    }
}

#[test]
fn test_coherence() {
    assert!((0..1000).all(|x| gray_decode(gray_encode(x)) == x));
}
