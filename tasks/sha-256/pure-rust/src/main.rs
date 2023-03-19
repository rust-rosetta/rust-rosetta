const HASH_VALUES: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const INPUT: &str = "Rosetta code";

fn main() {
    let mut bytes = INPUT.as_bytes().to_vec();

    let mut hash_values = HASH_VALUES.to_vec();

    let input_len = bytes.len(); // Bytes
    let input_len_byte = (input_len * 8).to_be_bytes(); // Bits

    let padding = ((64 * ((input_len + 72) / 64)) - input_len) - 9; // Bytes

    bytes.push(128);
    bytes.append(&mut vec![0; padding]);
    bytes.extend(input_len_byte);

    for byte_chunk in bytes.chunks(64) {
        let mut working_hash = hash_values.clone();

        let mut joined_bytes: Vec<u32> = byte_chunk
            .chunks(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        joined_bytes.append(&mut vec![0; 48]);

        // Message loop

        for i in 16..64 {
            let chunk_index_1 = joined_bytes[i - 15];
            let chunk_index_2 = joined_bytes[i - 2];

            let sigma_1 = chunk_index_1.rotate_right(7)
                ^ chunk_index_1.rotate_right(18)
                ^ (chunk_index_1 >> 3);
            let sigma_2 = chunk_index_2.rotate_right(17)
                ^ chunk_index_2.rotate_right(19)
                ^ (chunk_index_2 >> 10);

            joined_bytes[i] = joined_bytes[i - 16]
                .wrapping_add(sigma_1.wrapping_add(joined_bytes[i - 7].wrapping_add(sigma_2)));
        }

        // Compression loop

        for i in 0..64 {
            let sigma_1 = working_hash[4].rotate_right(6)
                ^ working_hash[4].rotate_right(11)
                ^ working_hash[4].rotate_right(25);
            let choice =
                (working_hash[4] & working_hash[5]) ^ ((!working_hash[4]) & working_hash[6]);
            let temp_1 = working_hash[7].wrapping_add(sigma_1.wrapping_add(
                choice.wrapping_add(ROUND_CONSTANTS[i].wrapping_add(joined_bytes[i])),
            ));

            let sigma_0 = working_hash[0].rotate_right(2)
                ^ working_hash[0].rotate_right(13)
                ^ working_hash[0].rotate_right(22);
            let majority = (working_hash[0] & working_hash[1])
                ^ (working_hash[0] & working_hash[2])
                ^ (working_hash[1] & working_hash[2]);
            let temp_2 = sigma_0.wrapping_add(majority);

            working_hash.pop();
            working_hash.insert(0, temp_1.wrapping_add(temp_2));
            working_hash[4] = working_hash[4].wrapping_add(temp_1);
        }

        hash_values = hash_values
            .iter()
            .zip(working_hash)
            .map(|(hv1, hv2)| hv1.wrapping_add(hv2))
            .collect();
    }

    let output: String = hash_values
        .iter()
        .map(|val| format!("{:x}", val))
        .collect::<Vec<String>>()
        .join("");

    assert_eq!(
        output,
        "764faf5c61ac315f1497f9dfa542713965b785e5cc2f707d6468d7d1124cdfcf"
    );

    println!("{}", output);
}
