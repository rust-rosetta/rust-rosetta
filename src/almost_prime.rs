// Implements http://rosettacode.org/wiki/Almost_prime

fn kprime(n: int, k: int) -> bool {
    let mut _n = n;
    let mut p = 2;
    let mut f = 0;

    while f < k && p * p <= _n {
        while 0 == _n % p {
            _n /= p;
            f += 1;
        }
        p += 1;
    }

    f + (_n > 1) as int == k
}

fn main() {
    for k in range(1, 6) {
        let mut i = 2;
        let mut v: ~[int] = ~[];

        while v.len() < 10 {
            if kprime(i, k) {
                v.push(i);
            }
            i += 1;
        }
        println!("k = {}: {:?}", k, v);
    }
}
