/// state for producing generalized Fibonacci sequences
struct GenFibonacci {
    /// current values being summed
    buf: Vec<u64>,

    /// current sum
    sum: u64,

    /// index of smallest element
    idx: usize,
}

/// note: iterator starts with values *after* the buffer contents
impl Iterator for GenFibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let result = self.sum;                      // capture current sum
        self.sum += result - self.buf[self.idx];    // add new elt, subtract old
        self.buf[self.idx] = result;                // write new elt to buffer
        self.idx = (self.idx + 1) % self.buf.len(); // advance index
        Some(result)                                // return result
    }
}

/// prints the starting buf and len number of additional elements
fn print(buf: Vec<u64>, len: usize) {
    let mut sum = 0;
    for &elt in &buf {
        sum += elt;
        print!("\t{}", elt);
    }
    let iter = GenFibonacci {
        buf: buf,
        sum: sum,
        idx: 0,
    };
    for x in iter.take(len) {
        print!("\t{}", x);
    }
}

#[cfg(test)]
mod tests {
    use super::GenFibonacci;

    /// test equivalence between tgt and sequence generated from buf
    fn test(mut buf: Vec<u64>, tgt: Vec<u64>) {
        let mut sum = 0;
        for elt in &buf {
            sum += *elt;
        }
        let mut iter = GenFibonacci {
            buf: buf.clone(),
            sum: sum,
            idx: 0,
        };
        while buf.len() < tgt.len() {
            buf.push(iter.next().unwrap());
        }

        assert_eq!(buf, tgt);
    }

    #[test]
    fn test_fib2() {
        test(vec![1, 1], vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
    #[test]
    fn test_fib3() {
        test(vec![1, 1, 2], vec![1, 1, 2, 4, 7, 13, 24, 44, 81, 149]);
    }
    #[test]
    fn test_fib4() {
        test(vec![1, 1, 2, 4], vec![1, 1, 2, 4, 8, 15, 29, 56, 108, 208]);
    }
    #[test]
    fn test_lucas() {
        test(vec![2, 1], vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76]);
    }
}


// main() should print:
// Fib2:	1	1	2	3	5	8	13	21	34	55
// Fib3:	1	1	2	4	7	13	24	44	81	149
// Fib4:	1	1	2	4	8	15	29	56	108	208
// Lucas:	2	1	3	4	7	11	18	29	47	76
fn main() {
    print!("Fib2:");
    print(vec![1, 1], 10 - 2);

    print!("\nFib3:");
    print(vec![1, 1, 2], 10 - 3);

    print!("\nFib4:");
    print(vec![1, 1, 2, 4], 10 - 4);

    print!("\nLucas:");
    print(vec![2, 1], 10 - 2);
    println!("");
}
