// Implements http://rosettacode.org/wiki/Linear_congruential_generator

trait LinearCongruentialGenerator {
    fn seed(&mut self, seed: u32);
    fn next(&mut self) -> u32;
}

struct BSDLinearCongruentialGenerator {
    state: u32
}

impl BSDLinearCongruentialGenerator {
    fn new(seed: u32) -> BSDLinearCongruentialGenerator {
        BSDLinearCongruentialGenerator { state: seed }
    }
}

impl LinearCongruentialGenerator for BSDLinearCongruentialGenerator {
    fn seed(&mut self, seed: u32) {
        self.state = seed;
    }

    fn next(&mut self) -> u32 {
        self.state = (1103515245 * self.state + 12345) % (1 << 31);
        self.state
    }
}

struct MSLinearCongruentialGenerator {
    state: u32
}

impl MSLinearCongruentialGenerator {
    fn new(seed: u32) -> MSLinearCongruentialGenerator {
        MSLinearCongruentialGenerator { state: seed }
    }
}

impl LinearCongruentialGenerator for MSLinearCongruentialGenerator {
    fn seed(&mut self, seed: u32) {
        self.state = seed;
    }

    fn next(&mut self) -> u32 {
        self.state = (214013 * self.state + 2531011) % (1 << 31);
        self.state >> 16
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
    use super::{LinearCongruentialGenerator, BSDLinearCongruentialGenerator, MSLinearCongruentialGenerator};

    #[test]
    fn bsd() {
        let values = [12345u32, 1406932606, 654583775, 1449466924, 229283573, 1109335178, 1051550459, 1293799192, 794471793, 551188310];
        let mut lcg = BSDLinearCongruentialGenerator::new(0);
        for val in values.iter() {
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
