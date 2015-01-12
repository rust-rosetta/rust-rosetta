// http://rosettacode.org/wiki/Fibonacci_n-step_number_sequences

// stores current values, current sum, and position of least value (to be overwritten next)
struct GenFibonacci {
    buf:    Vec<u64>,
    sum:    u64,
    idx:    usize,
}

// iterator starts *after* the buffer contents
impl Iterator for GenFibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let result = Some(self.sum);
        self.sum -= self.buf[self.idx];
        self.buf[self.idx] += self.sum;
        self.sum += self.buf[self.idx];
        self.idx = (self.idx + 1) % self.buf.len();
        result
    }
}

// prints the starting buf and len number of additional elements
fn print(buf: Vec<u64>, len: usize) {
    let mut sum = 0;
    for &elt in buf.iter() { sum += elt; print!("\t{}", elt); }
    let iter = GenFibonacci { buf: buf, sum: sum, idx: 0 };
    for x in iter.take(len) {
        print!("\t{}", x);
    }
}

fn test(mut buf: Vec<u64>, tgt: Vec<u64>) {
    let mut sum = 0;
    for &elt in buf.iter() { sum += elt; }
    let mut iter = GenFibonacci { buf: buf.clone(), sum: sum, idx: 0 };
    while (buf.len() < tgt.len()) {
        buf.push(iter.next().unwrap());
    }

    assert_eq!(buf, tgt);
}

#[test] fn test_fib2() { test(vec![1,1], vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]); }
#[test] fn test_fib3() { test(vec![1,1,2], vec![1, 1, 2, 4, 7, 13, 24, 44, 81, 149]); }
#[test] fn test_fib4() { test(vec![1,1,2,4], vec![1, 1, 2, 4, 8, 15, 29, 56, 108, 208]); }
#[test] fn test_lucas() { test(vec![2,1], vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76]); }


// should print:
// Fib2:	1	1	2	3	5	8	13	21	34	55
// Fib3:	1	1	2	4	7	13	24	44	81	149
// Fib4:	1	1	2	4	8	15	29	56	108	208
// Lucas:	2	1	3	4	7	11	18	29	47	76

fn print_output() {
    print!("Fib2:");
    print(vec![1,1], 10 - 2);

    print!("\nFib3:");
    print(vec![1,1,2], 10 - 3);

    print!("\nFib4:");
    print(vec![1,1,2,4], 10 - 4);

    print!("\nLucas:");
    print(vec![2,1], 10 - 2);
}
