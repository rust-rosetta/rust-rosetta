extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

/// Decodes a base58-encoded string into an array of bytes.
fn decode_base58(address: &str) -> Result<Vec<u8>, &'static str> {
    const ALPHABET: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    const BASE: usize = 58;

    let mut result = vec![0; 25];

    for c in address.chars() {
        let mut value = match ALPHABET.find(c) {
            Some(index) => index,
            None => return Err("invalid character"),
        };

        for byte in &mut result {
            value += BASE * (*byte as usize);
            *byte = (value % 256) as u8;
            value /= 256;
        }

        if value > 0 {
            return Err("address too long");
        }
    }

    Ok(result.iter().rev().map(|&byte| byte as u8).collect())
}

/// Hashed the input with the SHA-256 algorithm twice, and returns the output.
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

/// Validates a bitcoin address.
///
/// Returns `Ok` if the address validates.
/// Returns `Err` with the reason if the address is invalid.
fn validate(address: &str) -> Result<(), &'static str> {
    let bytes = try!(decode_base58(address));

    // A bitcoin address encodes 25 bytes:
    if bytes.len() != 25 {
        return Err("invalid length");
    }

    // The first byte is the version number, which will be zero for this task
    if bytes[0] != 0 {
        return Err("unknown version encountered");
    }

    // The next twenty bytes are a RIPEMD-160 digest, but you don't have to know that for this
    // task: you can consider them a pure arbitrary data
    let _ = &bytes[1..21];

    // The last four bytes are a checksum check. They are the first four bytes of a double SHA-256
    // digest of the previous 21 bytes.
    let checksum = &bytes[21..];
    let result = double_sha256(&bytes[..21]);
    if &result[..4] != checksum {
        return Err("checksum did not validate");
    }

    Ok(())
}

fn main() {
    use std::env;

    let result = match env::args().nth(2) {
        Some(address) => validate(&address),
        None => Err("no address supplied"),
    };

    match result {
        Ok(()) => println!("address is valid."),
        Err(reason) => println!("address is invalid: {}", reason),
    }
}

#[test]
fn test_valid() {
    assert!(validate("1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62i").is_ok());
}

#[test]
fn test_invalid() {
    assert!(validate("1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62j").unwrap_err() ==
            "checksum did not validate")
}
