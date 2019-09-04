use ripemd160::{Digest, Ripemd160};

/// Create a lowercase hexadecimal string using the
/// RIPEMD160 hashing algorithm
fn ripemd160(text: &str) -> String {
    // create a lowercase hexadecimal string
    // using the shortand for the format macro
    // https://doc.rust-lang.org/std/fmt/trait.LowerHex.html
    format!("{:x}", Ripemd160::digest(text.as_bytes()))
}

fn main() {
    println!("{}", ripemd160("Rosetta Code"));
}

#[cfg(test)]
mod tests {
    use super::ripemd160;

    #[test]
    fn test_ripemd160() {
        assert_eq!(
            ripemd160("Rosetta Code"),
            "b3be159860842cebaa7174c8fff0aa9e50a5199f"
        );
        assert_eq!(ripemd160(""), "9c1185a5c5e9fc54612808977ee8f548b2258d31");
        assert_eq!(
            ripemd160("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            "b0e20b6e3116640286ed3a87a5713079b21f5189"
        );
    }
}
