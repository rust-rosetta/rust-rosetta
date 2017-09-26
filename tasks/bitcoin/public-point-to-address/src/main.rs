extern crate crypto;
extern crate hex;

use hex::FromHex;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ripemd160::Ripemd160;

static X: &'static str = "50863AD64A87AE8A2FE83C1AF1A8403CB53F53E486D8511DAD8A04887E5B2352";
static Y: &'static str = "2CD470243453A299FA9E77237716103ABC11A1DF38855ED6F2EE187E9C582BA6";
static ALPHABET: [char; 58] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D',
                               'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S',
                               'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                               'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
                               'u', 'v', 'w', 'x', 'y', 'z'];

fn base58_encode(bytes: &mut [u8]) -> String {
    let base = ALPHABET.len();
    if bytes.len() == 0 {
        return String::from("");
    }
    let mut output: Vec<u8> = Vec::new();
    let mut num: usize;
    for _ in 0..33 {
        num = 0;
        for byte in bytes.iter_mut() {
            num = num * 256 + *byte as usize;
            *byte = (num / base) as u8;
            num = num % base;
        }
        output.push(num as u8);
    }
    let mut string = String::new();
    for b in output.iter().rev() {
        string.push(ALPHABET[*b as usize]);
    }
    string
}

/// Hashes the input with the SHA-256 algorithm twice, and returns the output.
fn double_sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.input(bytes);
    let mut digest_1 = vec![0; 32];
    hasher.result(&mut digest_1);
    hasher.reset();

    hasher.input(&digest_1);
    let mut digest_2 = vec![0; 32];
    hasher.result(&mut digest_2);

    digest_2
}

fn point_to_address(x: &str, y: &str) -> String {
    let mut addrv: Vec<u8> = Vec::with_capacity(65);
    addrv.push(4u8);
    addrv.append(&mut <Vec<u8>>::from_hex(x).unwrap());
    addrv.append(&mut <Vec<u8>>::from_hex(y).unwrap());
    // create Sha256 hasher
    let mut sha256 = Sha256::new();
    sha256.input(&addrv);
    let mut sha_digest = vec![0; 32];
    sha256.result(&mut sha_digest);
    // create Ripemd object
    let mut ripemd = Ripemd160::new();
    ripemd.input(&sha_digest);
    // prepend 0
    let mut ripemd_digest = vec![0; 21];
    ripemd.result(&mut ripemd_digest[1..]);
    // calculate checksum of extended ripemd digest
    let checksum = double_sha256(&ripemd_digest);
    ripemd_digest.extend_from_slice(&checksum[..4]);
    base58_encode(&mut ripemd_digest)
}

fn main() {
    println!("{}", point_to_address(X, Y));
}

#[cfg(test)]
mod tests {
    use super::{X, Y, point_to_address};

    #[test]
    fn bitcoin_address() {
        assert_eq!(point_to_address(X, Y), "16UwLL9Risc3QfPqBUvKofHmBQ7wMtjvM");
    }
}
