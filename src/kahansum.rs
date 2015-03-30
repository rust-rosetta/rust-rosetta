// Implements http://rosettacode.org/wiki/Kahan_summation
#![feature(std_misc)]
#![feature(collections)]

use std::num::Float;
use std::f32;

fn find_max(lst: &[f32]) -> Option<f32> {
    if lst.is_empty() { return None }
    let max = lst.iter().fold(f32::NEG_INFINITY,
                              |a, &b| Float::max(a, b));
    Some(max)
}

fn with_bits(val: f32, digits: usize) -> f32 {
    let num = std::f32::to_str_digits(val, digits);
    num.parse::<f32>().unwrap()
}

fn kahan_sum(lst: &[f32]) -> Option<f32> {
    let mut sum = 0.0f32;
    let mut c = 0.0f32;
    for i in lst {
        let y = *i - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    Some(with_bits(sum, 1))
}


fn all_sums(vec: &[f32]) -> Vec<f32> {
    let mut res = Vec::new();
    let mut perms = vec.permutations();
    loop {
        let v = perms.next();
        match v {
            Some(_v) =>  {
                let mut sum = 0.0f32;
                for e in &_v {
                    sum += with_bits(*e, 1);
                }
                res.push(with_bits(sum, 1));
            }
            None => break
        }
    }
    res
}

#[cfg(not(test))]
fn main() {
    let v = [10000.0f32, 3.14159, 2.71828];
    let sums = all_sums(&v);
    let res = kahan_sum(&v).unwrap();
    let max = find_max(&sums[..]).unwrap();
    println!("max: {} res: {}", max, res);
}

#[test]
fn test_kahansum() {
    let v = [10000.0f32, 3.14159, 2.71828];
    let sums = all_sums(&v);
    let res = kahan_sum(&v).unwrap();
    let max = find_max(&sums[..]).unwrap();
    assert!(max < res);
}

#[test]
fn test_withbits() {
    let v = 3.123345f32;
    let res = with_bits(v, 3);
    assert!(res == 3.123f32);
}
