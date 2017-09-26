//! includes the XOR version of the encryption scheme
#![feature(iterator_step_by)]
use std::num::Wrapping as w;

const MSG: &'static str = "a Top Secret secret";
const KEY: &'static str = "this is my secret key";

fn main() {
    let mut isaac = Isaac::new();
    isaac.seed(KEY, true);
    let encr = isaac.vernam(MSG.as_bytes());

    println!("msg: {}", MSG);
    println!("key: {}", KEY);
    print!("XOR: ");
    for a in &encr {
        print!("{:02X}", *a);
    }

    let mut isaac = Isaac::new();
    isaac.seed(KEY, true);
    let decr = isaac.vernam(&encr[..]);

    print!("\nXOR dcr: ");
    println!("{}", String::from_utf8(decr).unwrap())
}

macro_rules! mix_v(
   ($a:expr) => (
   {
       $a[0] = $a[0] ^ $a[1] << 11; $a[3] = $a[3] + $a[0]; $a[1] = $a[1] + $a[2];
       $a[1] = $a[1] ^ $a[2] >> 2; $a[4] = $a[4] + $a[1]; $a[2] = $a[2] + $a[3];
       $a[2] = $a[2] ^ $a[3] << 8; $a[5] = $a[5] + $a[2]; $a[3] = $a[3] + $a[4];
       $a[3] = $a[3] ^ $a[4] >> 16; $a[6] = $a[6] + $a[3]; $a[4] = $a[4] + $a[5];
       $a[4] = $a[4] ^ $a[5] << 10; $a[7] = $a[7] + $a[4]; $a[5] = $a[5] + $a[6];
       $a[5] = $a[5] ^ $a[6] >> 4; $a[0] = $a[0] + $a[5]; $a[6] = $a[6] + $a[7];
       $a[6] = $a[6] ^ $a[7] << 8; $a[1] = $a[1] + $a[6]; $a[7] = $a[7] + $a[0];
       $a[7] = $a[7] ^ $a[0] >> 9; $a[2] = $a[2] + $a[7]; $a[0] = $a[0] + $a[1];
   } );
);

struct Isaac {
    mm: [w<u32>; 256],
    aa: w<u32>,
    bb: w<u32>,
    cc: w<u32>,
    rand_rsl: [w<u32>; 256],
    rand_cnt: u32,
}

impl Isaac {
    fn new() -> Isaac {
        Isaac {
            mm: [w(0u32); 256],
            aa: w(0),
            bb: w(0),
            cc: w(0),
            rand_rsl: [w(0u32); 256],
            rand_cnt: 0,
        }
    }

    fn isaac(&mut self) {
        self.cc = self.cc + w(1);
        self.bb = self.bb + self.cc;

        for i in 0..256 {
            let w(x) = self.mm[i];
            match i % 4 {
                0 => self.aa = self.aa ^ self.aa << 13,
                1 => self.aa = self.aa ^ self.aa >> 6,
                2 => self.aa = self.aa ^ self.aa << 2,
                3 => self.aa = self.aa ^ self.aa >> 16,
                _ => unreachable!(),
            }

            self.aa = self.mm[((i + 128) % 256) as usize] + self.aa;
            let w(y) = self.mm[((x >> 2) % 256) as usize] + self.aa + self.bb;
            self.bb = self.mm[((y >> 10) % 256) as usize] + w(x);
            self.rand_rsl[i] = self.bb;
        }

        self.rand_cnt = 0;
    }

    fn rand_init(&mut self, flag: bool) {
        let mut a_v = [w(0x9e3779b9u32); 8];

        for _ in 0..4 {
            // scramble it
            mix_v!(a_v);
        }

        for i in (0..256).step_by(8) {
            // fill in mm[] with messy stuff
            if flag {
                // use all the information in the seed
                for (j, value) in a_v.iter_mut().enumerate().take(8) {
                    *value += self.rand_rsl[i + j];
                }
            }
            mix_v!(a_v);
            for (j, value) in a_v.iter().enumerate().take(8) {
                self.mm[i + j] = *value;
            }
        }

        if flag {
            // do a second pass to make all of the seed affect all of mm
            for i in (0..256).step_by(8) {
                for (j, value) in a_v.iter_mut().enumerate().take(8) {
                    *value += self.mm[i + j];
                }
                mix_v!(a_v);
                for (j, value) in a_v.iter().enumerate().take(8) {
                    self.mm[i + j] = *value;
                }
            }
        }

        self.isaac();       // fill in the first set of results
        self.rand_cnt = 0;  // prepare to use the first set of results
    }

    /// Get a random 32-bit value
    fn i_random(&mut self) -> u32 {
        let r = self.rand_rsl[self.rand_cnt as usize];
        self.rand_cnt += 1;
        if self.rand_cnt > 255 {
            self.isaac();
            self.rand_cnt = 0;
        }
        r.0
    }

    /// Seed ISAAC with a string
    fn seed(&mut self, seed: &str, flag: bool) {
        for i in 0..256 {
            self.mm[i] = w(0);
        }
        for i in 0..256 {
            self.rand_rsl[i] = w(0);
        }

        for i in 0..seed.len() {
            self.rand_rsl[i] = w(seed.as_bytes()[i] as u32);
        }
        // initialize ISAAC with seed
        self.rand_init(flag);
    }

    /// Get a random character in printable ASCII range
    fn i_rand_ascii(&mut self) -> u8 {
        (self.i_random() % 95 + 32) as u8
    }

    /// XOR message
    fn vernam(&mut self, msg: &[u8]) -> Vec<u8> {
        msg.iter()
            .map(|&b| (self.i_rand_ascii() ^ b))
            .collect::<Vec<u8>>()
    }
}

impl Default for Isaac {
    fn default() -> Self {
        Isaac::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Isaac, MSG, KEY};
    const ENCRIPTED: [u8; 19] = [0x1C, 0x06, 0x36, 0x19, 0x0B, 0x12, 0x60, 0x23, 0x3B, 0x35, 0x12,
                                 0x5F, 0x1E, 0x1D, 0x0E, 0x2F, 0x4C, 0x54, 0x22];

    #[test]
    fn encrypt() {
        let mut isaac = Isaac::new();
        isaac.seed(KEY, true);
        let encr = isaac.vernam(MSG.as_bytes());

        for (a, b) in encr.iter().zip(ENCRIPTED.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn decrypt() {
        let expected = MSG;

        let mut isaac = Isaac::new();
        isaac.seed(KEY, true);
        let decr = isaac.vernam(&ENCRIPTED[..]);

        for (&a, b) in decr.iter().zip(expected.bytes()) {
            assert_eq!(a, b);
        }
    }
}
