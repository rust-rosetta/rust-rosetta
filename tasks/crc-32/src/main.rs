fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    println!("{:X}", crc(s.as_bytes()));
}

fn crc(bytes: &[u8]) -> u32 {
    // Store the CRC of all possible 256 one byte values in table
    let mut table: [u32; 256] = [0; 256];
    for (i, value) in table.iter_mut().enumerate() {
        let mut word = i as u32;
        for _ in 0..8 {
            if word & 1 == 1 {
                word = (word >> 1) ^ 0xedb88320
            } else {
                word >>= 1;
            }
        }
        *value = word;
    }

    let mut crc: u32 = 0xffffffff;
    for byte in bytes {
        crc = table[(crc as u8 ^ *byte) as usize] ^ (crc >> 8);
    }
    crc ^ 0xffffffff
}

#[test]
fn test() {
    let s = "The quick brown fox jumps over the lazy dog";
    assert_eq!(crc(s.as_bytes()), 0x414FA339);
}
