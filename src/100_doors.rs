// Implements http://rosettacode.org/wiki/100_doors
#[cfg(not(test))]
fn main() {
    let drs = doors();
    for (i, x) in drs.iter().enumerate() {
        println!("Door {} is {}", i+1, x);
    }
}

fn doors() -> Vec<&'static str> {
   let mut ret = Vec :: with_capacity(100);
   for i in std::iter::range_inclusive(1,100) {
        let x = (i as f64).powf(0.5);
        let state = if x == x.round() {"open"} else {"closed"};
        ret.push(state);
    }
    ret
}

#[test]
fn solution() {
    let drs = doors();

    // test that the doors with index corresponding to
    // a perfect square are now open
    for i in std::iter::range_inclusive(1u,10u) {
        assert!(*drs.get(i*i - 1)=="open");
    }
}