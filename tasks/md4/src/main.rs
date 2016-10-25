// MD4, based on RFC 1186 and RFC 1320.
//
// https://www.ietf.org/rfc/rfc1186.txt
// https://tools.ietf.org/html/rfc1320
//

use std::fmt::Write;
use std::mem;

// Let not(X) denote the bit-wise complement of X.
// Let X v Y denote the bit-wise OR of X and Y.
// Let X xor Y denote the bit-wise XOR of X and Y.
// Let XY denote the bit-wise AND of X and Y.

// f(X,Y,Z)  =  XY v not(X)Z
fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

// g(X,Y,Z)  =  XY v XZ v YZ
fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

// h(X,Y,Z)  =  X xor Y xor Z
fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

// Round 1 macro
// Let [A B C D i s] denote the operation
//   A = (A + f(B,C,D) + X[i]) <<< s
macro_rules! md4round1 {
    ( $a:expr, $b:expr, $c:expr, $d:expr, $i:expr, $s:expr, $x:expr) => {
        {
            // Rust defaults to non-overflowing arithmetic, so we need to specify wrapping add.
            $a = ($a.wrapping_add( f($b, $c, $d) ).wrapping_add( $x[$i] ) ).rotate_left($s);
        }
    };
}

// Round 2 macro
// Let [A B C D i s] denote the operation
//   A = (A + g(B,C,D) + X[i] + 5A827999) <<< s .
macro_rules! md4round2 {
    ( $a:expr, $b:expr, $c:expr, $d:expr, $i:expr, $s:expr, $x:expr) => {
        {
            $a = ($a.wrapping_add( g($b, $c, $d))
                        .wrapping_add($x[$i])
                        .wrapping_add(0x5a827999_u32))
                    .rotate_left($s);
        }
    };
}

// Round 3 macro
// Let [A B C D i s] denote the operation
//   A = (A + h(B,C,D) + X[i] + 6ED9EBA1) <<< s .
macro_rules! md4round3 {
    ( $a:expr, $b:expr, $c:expr, $d:expr, $i:expr, $s:expr, $x:expr) => {
        {
            $a = ($a.wrapping_add(h($b, $c, $d))
                        .wrapping_add($x[$i])
                        .wrapping_add(0x6ed9eba1_u32))
                    .rotate_left($s);
        }
    };
}

fn convert_byte_vec_to_u32(mut bytes: Vec<u8>) -> Vec<u32> {

    bytes.shrink_to_fit();
    let num_bytes = bytes.len();
    let num_words = num_bytes / 4;
    unsafe {
        let words = Vec::from_raw_parts(bytes.as_mut_ptr() as *mut u32, num_words, num_words);
        mem::forget(bytes);
        words
    }
}

// Returns a 128-bit MD4 hash as an array of four 32-bit words.
// Based on RFC 1186 from https://www.ietf.org/rfc/rfc1186.txt
fn md4<T: Into<Vec<u8>>>(input: T) -> [u32; 4] {

    let mut bytes = input.into().to_vec();
    let initial_bit_len = (bytes.len() << 3) as u64;

    // Step 1. Append padding bits
    // Append one '1' bit, then append 0 ≤ k < 512 bits '0', such that the resulting message
    // length in bis is congruent to 448 (mod 512).
    // Since our message is in bytes, we use one byte with a set high-order bit (0x80) plus
    // a variable number of zero bytes.

    // Append zeros
    // Number of padding bytes needed is 448 bits (56 bytes) modulo 512 bits (64 bytes)
    bytes.push(0x80_u8);
    while (bytes.len() % 64) != 56 {
        bytes.push(0_u8);
    }

    // Everything after this operates on 32-bit words, so reinterpret the buffer.
    let mut w = convert_byte_vec_to_u32(bytes);

    // Step 2. Append length
    // A 64-bit representation of b (the length of the message before the padding bits were added)
    // is appended to the result of the previous step, low-order bytes first.
    w.push(initial_bit_len as u32); // Push low-order bytes first
    w.push((initial_bit_len >> 32) as u32);

    // Step 3. Initialize MD buffer
    let mut a = 0x67452301_u32;
    let mut b = 0xefcdab89_u32;
    let mut c = 0x98badcfe_u32;
    let mut d = 0x10325476_u32;

    // Step 4. Process message in 16-word blocks
    let n = w.len();
    for i in 0..n / 16 {

        // Select the next 512-bit (16-word) block to process.
        let x = &w[i * 16..i * 16 + 16];

        let aa = a;
        let bb = b;
        let cc = c;
        let dd = d;

        // [Round 1]
        md4round1!(a, b, c, d, 0, 3, x);  // [A B C D 0 3]
        md4round1!(d, a, b, c, 1, 7, x);  // [D A B C 1 7]
        md4round1!(c, d, a, b, 2, 11, x); // [C D A B 2 11]
        md4round1!(b, c, d, a, 3, 19, x); // [B C D A 3 19]
        md4round1!(a, b, c, d, 4, 3, x);  // [A B C D 4 3]
        md4round1!(d, a, b, c, 5, 7, x);  // [D A B C 5 7]
        md4round1!(c, d, a, b, 6, 11, x); // [C D A B 6 11]
        md4round1!(b, c, d, a, 7, 19, x); // [B C D A 7 19]
        md4round1!(a, b, c, d, 8, 3, x);  // [A B C D 8 3]
        md4round1!(d, a, b, c, 9, 7, x);  // [D A B C 9 7]
        md4round1!(c, d, a, b, 10, 11, x);// [C D A B 10 11]
        md4round1!(b, c, d, a, 11, 19, x);// [B C D A 11 19]
        md4round1!(a, b, c, d, 12, 3, x); // [A B C D 12 3]
        md4round1!(d, a, b, c, 13, 7, x); // [D A B C 13 7]
        md4round1!(c, d, a, b, 14, 11, x);// [C D A B 14 11]
        md4round1!(b, c, d, a, 15, 19, x);// [B C D A 15 19]

        // [Round 2]
        md4round2!(a, b, c, d, 0, 3, x);  //[A B C D 0  3]
        md4round2!(d, a, b, c, 4, 5, x);  //[D A B C 4  5]
        md4round2!(c, d, a, b, 8, 9, x);  //[C D A B 8  9]
        md4round2!(b, c, d, a, 12, 13, x);//[B C D A 12 13]
        md4round2!(a, b, c, d, 1, 3, x);  //[A B C D 1  3]
        md4round2!(d, a, b, c, 5, 5, x);  //[D A B C 5  5]
        md4round2!(c, d, a, b, 9, 9, x);  //[C D A B 9  9]
        md4round2!(b, c, d, a, 13, 13, x);//[B C D A 13 13]
        md4round2!(a, b, c, d, 2, 3, x);  //[A B C D 2  3]
        md4round2!(d, a, b, c, 6, 5, x);  //[D A B C 6  5]
        md4round2!(c, d, a, b, 10, 9, x); //[C D A B 10 9]
        md4round2!(b, c, d, a, 14, 13, x);//[B C D A 14 13]
        md4round2!(a, b, c, d, 3, 3, x);  //[A B C D 3  3]
        md4round2!(d, a, b, c, 7, 5, x);  //[D A B C 7  5]
        md4round2!(c, d, a, b, 11, 9, x); //[C D A B 11 9]
        md4round2!(b, c, d, a, 15, 13, x);//[B C D A 15 13]

        // [Round 3]
        md4round3!(a, b, c, d, 0, 3, x);  //[A B C D 0  3]
        md4round3!(d, a, b, c, 8, 9, x);  //[D A B C 8  9]
        md4round3!(c, d, a, b, 4, 11, x); //[C D A B 4  11]
        md4round3!(b, c, d, a, 12, 15, x);//[B C D A 12 15]
        md4round3!(a, b, c, d, 2, 3, x);  //[A B C D 2  3]
        md4round3!(d, a, b, c, 10, 9, x); //[D A B C 10 9]
        md4round3!(c, d, a, b, 6, 11, x); //[C D A B 6  11]
        md4round3!(b, c, d, a, 14, 15, x);//[B C D A 14 15]
        md4round3!(a, b, c, d, 1, 3, x);  //[A B C D 1  3]
        md4round3!(d, a, b, c, 9, 9, x);  //[D A B C 9  9]
        md4round3!(c, d, a, b, 5, 11, x); //[C D A B 5  11]
        md4round3!(b, c, d, a, 13, 15, x);//[B C D A 13 15]
        md4round3!(a, b, c, d, 3, 3, x);  //[A B C D 3  3]
        md4round3!(d, a, b, c, 11, 9, x); //[D A B C 11 9]
        md4round3!(c, d, a, b, 7, 11, x); //[C D A B 7  11]
        md4round3!(b, c, d, a, 15, 15, x);//[B C D A 15 15]

        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    // Step 5. Output
    // The message digest produced as output is A, B, C, D. That is, we begin with the low-order
    // byte of A, and end with the high-order byte of D.
    [u32::from_be(a), u32::from_be(b), u32::from_be(c), u32::from_be(d)]
}

fn digest_to_str(digest: &[u32]) -> String {
    let mut s = String::new();
    for &word in digest {
        write!(&mut s, "{:08x}", word).unwrap();
    }
    s
}

fn main() {
    let val = "Rosetta Code";
    println!("md4(\"{}\") = {}", val, digest_to_str(&md4(val)));
}

#[test]
fn test_rfc1320() {
    // The following test vectors are defined in RFC 1320 (The MD4 Message-Digest Algorithm).
    assert_eq!("31d6cfe0d16ae931b73c59d7e0c089c0", digest_to_str(&md4("")));
    assert_eq!("bde52cb31de33e46245e05fbdbd6fb24", digest_to_str(&md4("a")));

    assert_eq!("a448017aaf21d8525fc10ae87aa6729d",
               digest_to_str(&md4("abc")));

    assert_eq!("d9130a8164549fe818874806e1c7014b",
               digest_to_str(&md4("message digest")));

    assert_eq!("d79e1c308aa5bbcdeea8ed63df412da9",
               digest_to_str(&md4("abcdefghijklmnopqrstuvwxyz")));

    assert_eq!("043f8582f241db351ce627e153e7f0e4",
               digest_to_str(&md4("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\
                                   0123456789")));

    assert_eq!("e33b4ddc9c38f2199c3e7b164fcc0536",
               digest_to_str(&md4("12345678901234567890123456789012345678901234567890123456789\
                                   012345678901234567890")));


}
