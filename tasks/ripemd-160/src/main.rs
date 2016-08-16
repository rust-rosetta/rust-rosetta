extern crate crypto;

use crypto::ripemd160;
use crypto::digest::Digest;

fn ripemd_str(string: &str) -> String {
    let mut ripemd = ripemd160::Ripemd160::new();
    ripemd.input_str(string);
    ripemd.result_str()
}

fn main() {
    println!("{}", ripemd_str("Rosetta Code"));
}

#[cfg(test)]
mod tests {
    use super::ripemd_str;

    #[test]
    fn ripemd160() {
        assert_eq!(ripemd_str("Rosetta Code"),
                   "b3be159860842cebaa7174c8fff0aa9e50a5199f");
        assert_eq!(ripemd_str(""), "9c1185a5c5e9fc54612808977ee8f548b2258d31");
        assert_eq!(ripemd_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
                   "b0e20b6e3116640286ed3a87a5713079b21f5189");
    }
}
