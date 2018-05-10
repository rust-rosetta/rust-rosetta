// http://rosettacode.org/wiki/MD5/Implementation

//! Ported from C - Simple MD5 implementation
//! on Wikipedia: <https://en.wikipedia.org/wiki/MD5>

#![feature(iterator_step_by)]

use std::fmt::{Debug, Formatter, Result};
use std::num::Wrapping as wr;

fn main() {
    let inputs = [
        &b"a"[..],
        &b"abc"[..],
        &b"message digest"[..],
        &b"abcdefghijklmnopqrstuvwxyz"[..],
        &b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"[..],
        &b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"[..],
    ];

    for &input in &inputs {
        println!("{:?}", md5(input));
    }
}

/// Constants are the integer part of the sines of integers (in radians) * 2^32.
const K: [u32; 64] = [
    0xd76a_a478,
    0xe8c7_b756,
    0x2420_70db,
    0xc1bd_ceee,
    0xf57c_0faf,
    0x4787_c62a,
    0xa830_4613,
    0xfd46_9501,
    0x6980_98d8,
    0x8b44_f7af,
    0xffff_5bb1,
    0x895c_d7be,
    0x6b90_1122,
    0xfd98_7193,
    0xa679_438e,
    0x49b4_0821,
    0xf61e_2562,
    0xc040_b340,
    0x265e_5a51,
    0xe9b6_c7aa,
    0xd62f_105d,
    0x0244_1453,
    0xd8a1_e681,
    0xe7d3_fbc8,
    0x21e1_cde6,
    0xc337_07d6,
    0xf4d5_0d87,
    0x455a_14ed,
    0xa9e3_e905,
    0xfcef_a3f8,
    0x676f_02d9,
    0x8d2a_4c8a,
    0xfffa_3942,
    0x8771_f681,
    0x6d9d_6122,
    0xfde5_380c,
    0xa4be_ea44,
    0x4bde_cfa9,
    0xf6bb_4b60,
    0xbebf_bc70,
    0x289b_7ec6,
    0xeaa1_27fa,
    0xd4ef_3085,
    0x0488_1d05,
    0xd9d4_d039,
    0xe6db_99e5,
    0x1fa2_7cf8,
    0xc4ac_5665,
    0xf429_2244,
    0x432a_ff97,
    0xab94_23a7,
    0xfc93_a039,
    0x655b_59c3,
    0x8f0c_cc92,
    0xffef_f47d,
    0x8584_5dd1,
    0x6fa8_7e4f,
    0xfe2c_e6e0,
    0xa301_4314,
    0x4e08_11a1,
    0xf753_7e82,
    0xbd3a_f235,
    0x2ad7_d2bb,
    0xeb86_d391,
];

/// R specifies the per-round shift amounts
const R: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

/// "newtype" for [u8; 16] to specify it's a MD5 hash
struct MD5([u8; 16]);

impl Debug for MD5 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let MD5(md5) = *self;
        for b in &md5 {
            write!(f, "{:02x}", *b)?;
        }
        Ok(())
    }
}

fn to_bytes(val: u64) -> [u8; 8] {
    let mut tmp: [u8; 8] = [0u8; 8];
    for (i, byte) in tmp.iter_mut().enumerate() {
        *byte = (val >> (8 * i)) as u8;
    }
    tmp
}

#[cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
fn md5(initial_msg: &[u8]) -> MD5 {
    let initial_len = initial_msg.len() as u64;

    // These vars will contain the hash
    let mut h: [wr<u32>; 4] = [
        wr(0x6745_2301),
        wr(0xefcd_ab89),
        wr(0x98ba_dcfe),
        wr(0x1032_5476),
    ];

    // Pre-processing:
    // append "1" bit to message
    // append "0" bits until message length in bits ≡ 448 (mod 512)
    // append length mod (2^64) to message
    let mut new_len = initial_len;
    while new_len % (512 / 8) != 448 / 8 {
        new_len += 1;
    }

    let mut msg = initial_msg.to_vec();
    msg.push(0x80u8); // append the "1" bit; most significant bit is "first"

    for _ in (initial_len + 1)..new_len {
        msg.push(0); // append "0" bits
    }

    // append the len in bits at the end of the buffer.
    msg.extend_from_slice(&to_bytes(initial_len << 3));

    assert_eq!(msg.len() % 64, 0);

    let mut w: [u32; 16] = [0u32; 16];
    // Process the message in successive 512-bit chunks:
    // for each 512-bit chunk of message:
    for offset in (0u64..new_len).step_by(512 / 8) {
        // break chunk into sixteen 32-bit words w[j], 0 ≤ j ≤ 15
        for i in 0u32..16 {
            let j = i as usize * 4 + offset as usize;
            w[i as usize] = u32::from(msg[j])
                | u32::from(msg[j + 1]) << 8
                | u32::from(msg[j + 2]) << 16
                | u32::from(msg[j + 3]) << 24;
        }

        // Initialize hash value for this chunk:
        let (mut a, mut b, mut c, mut d) = (h[0], h[1], h[2], h[3]);

        // Main loop:
        for ind in 0..64 {
            let (f, g) = match ind {
                i @ 0...15 => ((b & c) | ((!b) & d) /* f */, i), //g
                i @ 16...31 => ((d & b) | ((!d) & c), (5 * i + 1) % 16),
                i @ 32...47 => (b ^ c ^ d, (3 * i + 5) % 16),
                i => (c ^ (b | (!d)), (7 * i) % 16),
            };

            let temp = d;
            d = c;
            c = b;
            b += wr((a + f + wr(K[ind]) + wr(w[g])).0.rotate_left(R[ind]));
            a = temp;
        }

        // Add this chunk's hash to result so far:
        h[0] += a;
        h[1] += b;
        h[2] += c;
        h[3] += d;
    }
    drop(msg); // cleanup, msg is freed

    // var char digest[16] := h0 append h1 append h2 append h3 //(Output is in little-endian)
    let mut digest = [0u8; 16];
    for (i, s) in h.iter().enumerate() {
        digest[i * 4] = (*s).0 as u8;
        digest[i * 4 + 1] = (*s >> 8).0 as u8;
        digest[i * 4 + 2] = (*s >> 16).0 as u8;
        digest[i * 4 + 3] = (*s >> 24).0 as u8;
    }
    MD5(digest)
}

#[test]
fn helper_fns() {
    assert_eq!(64, 8u32.rotate_left(3));

    let exp: [u8; 8] = [64u8, 226, 1, 0, 0, 0, 0, 0];
    assert!(to_bytes(123456) == exp);
}

#[test]
fn known_hashes() {
    let in_out: &[(&[u8], &str)] = &[
        (b"", "d41d8cd98f00b204e9800998ecf8427e"),
        (b"a", "0cc175b9c0f1b6a831c399e269772661"),
        (b"abc", "900150983cd24fb0d6963f7d28e17f72"),
        (b"message digest", "f96b697d7cb7938d525a2f31aaf161d0"),
        (
            b"abcdefghijklmnopqrstuvwxyz",
            "c3fcd3d76192e4007dfb496cca67e13b",
        ),
        (
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "d174ab98d277d9f5a5611c2c9f419d9f",
        ),
        (
            b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
            "57edf4a22be3c955ac49da2e2107b67a",
        ),
    ];

    for &(i, o) in in_out {
        let m = md5(i);
        assert_eq!(format!("{:?}", m), o.to_string());
    }
}
