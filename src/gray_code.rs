// Implements http://rosettacode.org/wiki/Gray_code

// Encode an uint
fn gray_encode(integer: uint) -> uint {
    (integer >> 1) ^ integer
}

// Decode an uint
fn gray_decode(integer: uint) -> uint {
    match integer {
        0 => 0,
        _ => integer ^ gray_decode(integer >> 1)
    }
}

#[cfg(not(test))]
fn main() {
    for i in range(0u,32u) {
        println!("{:2} {:0>5t} {:0>5t} {:2}", i, i, gray_encode(i),
            gray_decode(i));
    }
}

#[test]
fn test_coherence() {
    assert!(range(0u, 1000).all(|x| gray_decode(gray_encode(x)) == x));
}