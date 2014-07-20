//Implements http://rosettacode.org/wiki/9_billion_names_of_God_the_integer

extern crate num;

use num::bigint::BigUint;
use std::string::String;
use std::cmp::min;

fn cumu(num: uint, cache: &mut Vec<Vec<BigUint>>) -> &Vec<BigUint> {
    let len = cache.len();
    for l in range(len, num+1) {
        let initial_value:BigUint = from_str("0").unwrap();
        let mut r: Vec<BigUint> = vec!(initial_value);
        for x in range(1, l+1) {
            let y = r[x -1].clone();
            let z = (*cache)[l-x][min(x, l-x)].clone();
            let w = y+z;
            r.push(w)
        }
        cache.push(r);
    }
    &(*cache)[num]
}

// Returns a line
fn row(num: uint,  cache: &mut Vec<Vec<BigUint>>) -> String {
    let r = cumu(num,cache);
    let mut returned_string = String::new();
    for i in range(0,num) {
        let i = r[i+1] - r[i];
        returned_string.push_str(i.to_string().as_slice());
        returned_string.push_str(", ");
    }
    returned_string
}

#[cfg(not(test))]
fn main() {
    let mut cache: Vec<Vec<BigUint>> = Vec::new();
    let initial_value:BigUint = from_str("1").unwrap();
    let initial_vector : Vec<BigUint> = vec!(initial_value);
    cache.push(initial_vector);

    println!("rows");
    for n in range(1u, 11) {
        let x = n;
        println!("{}: {}", n, row(x,&mut cache));
    }

    println!("sums");

    let x: Vec<uint> = vec!(23, 123, 1234, 12345);
    for y in x.iter() {
  let z = cumu(*y,&mut cache);
  let w = z.last();
  println!("{}: {}", y, w.unwrap());
    }
}

#[test]
fn test_cumu() {
    let mut cache: Vec<Vec<BigUint>> = Vec::new();

    let initial_value:BigUint = from_str("1").unwrap();

    let initial_vector : Vec<BigUint> = vec!(initial_value);
    cache.push(initial_vector);


    let a: Vec<uint> = vec!(23, 123, 1234);
    let b: Vec<BigUint> = vec!(
        from_str("1255").unwrap(),
        from_str("2552338241").unwrap(),
        from_str("156978797223733228787865722354959930").unwrap());

    let mut n=0;
    for y in a.iter() {
        let z = cumu(*y,&mut cache);
        let w = z.last().unwrap();
        assert!(w == &b[n]);
        n= n+1;
    }
}


#[test]
fn test_row() {

    let mut cache: Vec<Vec<BigUint>> = Vec::new();

    let initial_value:BigUint = from_str("1").unwrap();

    let initial_vector : Vec<BigUint> = vec!(initial_value);
    cache.push(initial_vector);

    let a: String = from_str("1, 2, 1, 1, ").unwrap();

    let x = 4;
    assert!(a == row(x,&mut cache));


}
