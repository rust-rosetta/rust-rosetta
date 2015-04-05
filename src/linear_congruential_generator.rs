// Implements http://rosettacode.org/wiki/Linear_congruential_generator
use std::num::Wrapping as w;

trait LinearCongruentialGenerator {
    fn seed(&mut self, seed: u32);
    fn next(&mut self) -> u32;
}

struct BSDLinearCongruentialGenerator {
    state: w<u32>
}

impl BSDLinearCongruentialGenerator {
    fn new(seed: u32) -> BSDLinearCongruentialGenerator {
        BSDLinearCongruentialGenerator { state: w(seed) }
    }
}

impl LinearCongruentialGenerator for BSDLinearCongruentialGenerator {
    fn seed(&mut self, seed: u32) {
        self.state = w(seed);
    }

    fn next(&mut self) -> u32 {
        self.state = w((w(1103515245) * self.state + w(12345)).0 % (1 << 31));
        self.state.0
    }
}

struct MSLinearCongruentialGenerator {
    state: w<u32>
}

impl MSLinearCongruentialGenerator {
    fn new(seed: u32) -> MSLinearCongruentialGenerator {
        MSLinearCongruentialGenerator { state: w(seed) }
    }
}

impl LinearCongruentialGenerator for MSLinearCongruentialGenerator {
    fn seed(&mut self, seed: u32) {
        self.state = w(seed);
    }

    fn next(&mut self) -> u32 {
        self.state = w((w(214013) * self.state + w(2531011)).0 % (1 << 31));
        let w(r) = self.state >> 16;
        r
    }
}

#[cfg(not(test))]
fn main() {
    let mut bsd = BSDLinearCongruentialGenerator::new(0);
    let mut ms = MSLinearCongruentialGenerator::new(0);
    let names = ["BSD", "Microsoft"];
    let mut lcgs: [&mut LinearCongruentialGenerator; 2] = [&mut bsd, &mut ms];
    for (name, lcg) in names.iter().zip(lcgs.iter_mut()) {
        println!("{}", name);
        for _ in 0..10 {
            let next: u32 = lcg.next();
            println!("{}", next);
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::{LinearCongruentialGenerator, BSDLinearCongruentialGenerator,
                MSLinearCongruentialGenerator};

    #[test]
    fn bsd() {
        let values = [12345u32, 1406932606, 654583775, 1449466924, 229283573,
                      1109335178, 1051550459, 1293799192, 794471793, 551188310];
        let mut lcg = BSDLinearCongruentialGenerator::new(0);
        for val in &values {
            assert_eq!(lcg.next(), *val);
        }
    }

    #[test]
    fn ms() {
        let values = [38u32, 7719, 21238, 2437, 8855, 11797, 8365, 32285, 10450, 30612];
        let mut lcg = MSLinearCongruentialGenerator::new(0);
        for val in values.iter() {
            assert_eq!(lcg.next(), *val);
        }

    }
}
