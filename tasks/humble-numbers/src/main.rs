use itertools::Itertools;

struct HumbleNumberSeq {
    seq: Vec<u64>,
    current_h: u64,
    index: usize,
    next2: u64,
    next3: u64,
    next5: u64,
    next7: u64,
    last2: usize,
    last3: usize,
    last5: usize,
    last7: usize,
}

impl HumbleNumberSeq {
    fn new(max_number_of_numbers: usize) -> HumbleNumberSeq {
        HumbleNumberSeq {
            seq: vec![0; max_number_of_numbers],
            current_h: 1,
            index: 0,
            next2: 2,
            next3: 3,
            next5: 5,
            next7: 7,
            last2: 0,
            last3: 0,
            last5: 0,
            last7: 0,
        }
    }
}

impl Iterator for HumbleNumberSeq {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // Direct generation, with u64 it's good up to ~19 digits

        // if reached the max # of numbers, return None
        if self.index >= self.seq.len() {
            return None;
        }

        // this wil be returned
        self.seq[self.index] = self.current_h;
        self.index += 1;

        if self.current_h == self.next2 {
            self.last2 += 1;
            self.next2 = self.seq[self.last2] * 2;
        }

        if self.current_h == self.next3 {
            self.last3 += 1;
            self.next3 = self.seq[self.last3] * 3;
        }

        if self.current_h == self.next5 {
            self.last5 += 1;
            self.next5 = self.seq[self.last5] * 5;
        }

        if self.current_h == self.next7 {
            self.last7 += 1;
            self.next7 = self.seq[self.last7] * 7;
        }

        // store next number
        self.current_h = *vec![self.next2, self.next3, self.next5, self.next7]
            .iter()
            .min()
            .unwrap();

        // return latest number
        Some(self.seq[self.index - 1])
    }
}

fn main() {
    let seq_iter = HumbleNumberSeq::new(50);
    println!("First 50 Humble number");
    for (i, x) in seq_iter.enumerate() {
        if i % 10 == 9 {
            println!("{:>3}", x);
        } else {
            print!("{:>3}, ", x);
        }
    }

    let seq_iter = HumbleNumberSeq::new(85000);
    println!("\nOf the first 85.000 humble numbers");
    for (key, group) in &seq_iter.into_iter().group_by(|e| (*e).to_string().len()) {
        if key < 20 {
            println!("{:>5} have  {:>3} digits", group.count(), key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humble() {
        let seq_iter = HumbleNumberSeq::new(50);
        let result = seq_iter.skip(10).take(10).collect::<Vec<_>>();
        assert_eq!(result, vec![12, 14, 15, 16, 18, 20, 21, 24, 25, 27])
    }
}
