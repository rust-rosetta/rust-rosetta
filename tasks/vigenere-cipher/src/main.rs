const ASCII_A: u8 = b'A';

fn main() {
    let msg = "Beware the Jabberwock, my son! The jaws that bite, the claws that catch!";
    let key = "VIGENERECIPHER";

    let enc = vigenere(msg, key, true);
    let dec = vigenere(&enc, key, false);

    println!("msg: {}", msg);
    println!("key: {}", key);
    println!("enc: {}", enc);
    println!("dec: {}", dec);
}

fn vigenere(plaintext: &str, key: &str, encrypt: bool) -> String {
    let plaintext_bytes = to_sanitized_bytes(plaintext);
    let key_bytes = to_sanitized_bytes(key);
    let key_len = key_bytes.len();
    let mut output = String::with_capacity(plaintext_bytes.len());

    for (i, byte) in plaintext_bytes.iter().enumerate() {
        let c = *byte;
        let b = key_bytes[i % key_len];

        let output_byte = if encrypt {
            enc_byte(c, b)
        } else {
            dec_byte(c, b)
        };

        output.push(output_byte as char);
    }
    output
}

fn to_sanitized_bytes(string: &str) -> Vec<u8> {
    string.chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase() as u8)
        .collect::<Vec<u8>>()
}

fn enc_byte(m: u8, k: u8) -> u8 {
    ASCII_A + (m.wrapping_add(k).wrapping_sub(2 * (ASCII_A))) % 26
}

fn dec_byte(c: u8, k: u8) -> u8 {
    ASCII_A + (c.wrapping_sub(k).wrapping_add(26)) % 26
}

#[test]
fn test_enc_dec() {
    let plaintext = "Beware the Jabberwock, my son! The jaws that bite, the claws that catch!";
    let key = "VIGENERECIPHER";

    let enc = vigenere(plaintext, key, true);
    assert_eq!("WMCEEIKLGRPIFVMEUGXQPWQVIOIAVEYXUEKFKBTALVXTGAFXYEVKPAGY",
               enc);
    let dec = vigenere(&enc, key, false);
    assert_eq!("BEWARETHEJABBERWOCKMYSONTHEJAWSTHATBITETHECLAWSTHATCATCH",
               dec);
}

#[test]
fn test_equal_len_key_and_plaintext() {
    let plaintext = "VIGENERECIPHER";
    let key = "REHPICERENEGIV";
    // to be sure nobody breaks this test
    assert_eq!(plaintext.len(), key.len());

    let enc = vigenere(plaintext, key, true);
    assert_eq!("MMNTVGVVGVTNMM", enc);
    let dec = vigenere(&enc, key, false);
    assert_eq!(plaintext, dec);
}

#[test]
fn test_empty_string_enc_dec() {
    let enc = vigenere("", "", true);
    assert_eq!("", enc);
    let dec = vigenere("", "", false);
    assert_eq!("", dec);
}
