extern crate rand;

pub use rand::{Rng, SeedableRng};

pub struct BsdLcg {
    state: u32,
}

impl Rng for BsdLcg {
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.state %= 1 << 31;
        self.state
    }
}

impl SeedableRng<u32> for BsdLcg {
    fn from_seed(seed: u32) -> Self {
        Self { state: seed }
    }
    fn reseed(&mut self, seed: u32) {
        self.state = seed;
    }
}

pub struct MsLcg {
    state: u32,
}

impl Rng for MsLcg {
    // Because this only uses the high 16 bits of the state (`>> 16`), this should technically use
    // `next_u16`, but the `rand` crate does not provide it.  If serious usage is required,
    // implementing this function as a concatenation of two `next_u16`s (elsewhere defined) should
    // work.
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(214_013).wrapping_add(2_531_011);
        self.state %= 1 << 31;
        self.state >> 16 //  rand_n = state_n / 2^16
    }
}

impl SeedableRng<u32> for MsLcg {
    fn from_seed(seed: u32) -> Self {
        Self { state: seed }
    }
    fn reseed(&mut self, seed: u32) {
        self.state = seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ms() {
        let mut rng = MsLcg::from_seed(1);

        for &num in &[41, 18467, 6334, 26500, 19169] {
            assert_eq!(rng.next_u32(), num);
        }
    }

    #[test]
    fn test_bsd() {
        let mut rng = BsdLcg::from_seed(1);

        for &num in &[1103527590, 377401575, 662824084, 1147902781, 2035015474] {
            assert_eq!(rng.next_u32(), num);
        }
    }
}
