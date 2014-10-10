// http://rosettacode.org/wiki/Run-length_encoding
static INPUT: &'static str = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";

// Needed so look-and-say_sequence compiles cleanly, because it
// uses this code as a library
#[allow(dead_code)]
#[cfg(not(test))]
fn main() {
    let enc = encode(INPUT);
    println!("encoded {}", enc);

    let dec = decode(enc.as_slice());
    println!("decoded {}", dec.unwrap());
}

pub fn encode(value: &str) -> String {
    let mut ret = String::new();
    let mut chars = value.chars();

    let (mut count, mut cur) = (1u, chars.next());
    if cur.is_none() { return ret }

    for chr in chars {
        if cur == Some(chr) { count += 1 }
        else {
                ret.push_str(count.to_string().as_slice());
                ret.push(cur.unwrap());
                count=1u;
                cur=Some(chr);
        }
    }
    ret.push_str(count.to_string().as_slice());
    ret.push(cur.unwrap());
    ret
}

pub fn decode(value: &str) -> Result<String, String> {
    let mut result = String::new();
    if value.is_empty() { return Ok(result) }

    let mut start = 0;

    for (i, c) in value.char_indices() {
        if c.is_digit() { continue }
        if i==start { return Err(format!("expected digit, found {}", c)) }

        let ret_s = value.slice(start, i);
        let ret : uint = from_str(ret_s).unwrap();

        let repeated = String::from_char(ret, c);
        start = i + 1;

        result.push_str(repeated.as_slice());
    }
    Ok(result)
}

#[test]
fn test_encode_decode() {
    assert_eq!(decode(encode(INPUT).as_slice()).unwrap(), INPUT.to_string());
    assert_eq!(decode("a"), Err("expected digit, found a".to_string()));
}
