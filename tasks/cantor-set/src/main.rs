use convert_base::Convert;
use std::fmt;

struct CantorSet {
    cells: Vec<Vec<bool>>,
}
fn number_to_vec(n: usize) -> Vec<u32> {
    // for the conversion we need the digits in reverse order
    // i.e the least significant digit in the first element of the vector
    n.to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

impl CantorSet {
    fn new(lines: usize) -> CantorSet {
        // Convert from base 10- to base 3
        let mut base = Convert::new(10, 3);
        let mut cells: Vec<Vec<bool>> = vec![];

        for line in 0..lines {
            // calculate how many repeating sequence will be in the given line
            let segment_size = 3_usize.pow((lines - line - 1) as u32);
            let segment: Vec<bool> = (0..3_usize.pow(line as u32))
                .map(|n| {
                    let output = base.convert::<u32, u32>(&number_to_vec(n));
                    // return false in case the base 3 number contains at least one "1"
                    // otherwise return true
                    !output.contains(&1)
                })
                .collect();

            // copy the segment "segment_size" time
            let mut accum: Vec<bool> = Vec::with_capacity(segment.len() * segment_size);
            for c in segment.iter() {
                accum.extend(std::iter::repeat(*c).take(segment_size))
            }

            cells.push(accum);
        }

        CantorSet { cells }
    }
}

impl fmt::Display for CantorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.iter() {
            for c in line {
                write!(f, "{}", if *c { "█" } else { " " })?
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
fn main() {
    let cs = CantorSet::new(5);
    println!("Cantor set:");
    println!("{}", cs);
}
