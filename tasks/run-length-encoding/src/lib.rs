use std::iter::repeat;

pub const INPUT: &'static str =
    r"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";

pub fn encode(value: &str) -> String {
    let mut ret = String::new();
    let mut chars = value.chars();

    let (mut count, mut cur) = (1, chars.next());
    if cur.is_none() {
        return ret;
    }

    for chr in chars {
        if cur == Some(chr) {
            count += 1
        } else {
            ret.push_str(&(count.to_string())[..]);
            ret.push(cur.unwrap());
            count = 1;
            cur = Some(chr);
        }
    }
    ret.push_str(&(count.to_string())[..]);
    ret.push(cur.unwrap());
    ret
}

pub fn decode(value: &str) -> Result<String, String> {
    let mut result = String::new();
    if value.is_empty() {
        return Ok(result);
    }

    let mut start = 0;

    for (i, c) in value.char_indices() {
        if c.is_numeric() {
            continue;
        }
        if i == start {
            return Err(format!("expected digit, found {}", c));
        }

        let ret_s = &value[start..i];
        let ret = ret_s.parse::<usize>().unwrap();

        let repeated: String = repeat(c).take(ret).collect();
        start = i + 1;

        result.push_str(&repeated[..]);
    }
    Ok(result)
}

#[test]
fn test_encode_decode() {
    assert_eq!(decode(&encode(INPUT)[..]).unwrap(), INPUT);
    assert_eq!(decode("a"), Err("expected digit, found a".to_string()));
}
