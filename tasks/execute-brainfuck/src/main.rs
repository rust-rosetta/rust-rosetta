use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::num::Wrapping;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [path] (--debug)", args[0]);
        return;
    }

    let src: Vec<char> = {
        let mut buf = String::new();
        match File::open(&args[1]) {
            Ok(mut f) => {
                f.read_to_string(&mut buf).unwrap();
            }
            Err(e) => {
                println!("Error opening '{}': {}", args[1], e);
                return;
            }
        }

        buf.chars().collect()
    };

    // Launch options
    let debug = args.contains(&"--debug".to_owned());

    // One pass to find bracket pairs.
    let brackets: HashMap<usize, usize> = {
        let mut m = HashMap::new();
        let mut scope_stack = Vec::new();
        for (idx, ch) in src.iter().enumerate() {
            match *ch {
                '[' => {
                    scope_stack.push(idx);
                }
                ']' => {
                    m.insert(scope_stack.pop().unwrap(), idx);
                }
                _ => (),
            }
        }

        m
    };

    // Program counter
    let mut pc: usize = 0;

    // Program memory
    let mut mem: [Wrapping<u8>; 5000] = [Wrapping(0); 5000];

    // Pointer
    let mut ptr: usize = 0;

    // Bracket stack
    let mut stack: Vec<usize> = Vec::new();

    let stdin_ = stdin();
    let mut reader = stdin_.lock().bytes();
    while pc < src.len() {
        let Wrapping(val) = mem[ptr];

        if debug {
            println!("(BFDB) PC: {:04} \tPTR: {:04} \t$PTR: {:03} \tSTACK_DEPTH: {} \tSYMBOL: {}",
                     pc,
                     ptr,
                     val,
                     stack.len(),
                     src[pc]);
        }

        const ONE: Wrapping<u8> = Wrapping(1);
        match src[pc] {
            '>' => {
                ptr += 1;
            }
            '<' => {
                ptr -= 1;
            }

            '+' => {
                mem[ptr] = mem[ptr] + ONE;
            }
            '-' => {
                mem[ptr] = mem[ptr] - ONE;
            }

            '[' => {
                if val == 0 {
                    pc = brackets[&pc];
                } else {
                    stack.push(pc);
                }
            }
            ']' => {
                let matching_bracket = stack.pop().unwrap();
                if val != 0 {
                    pc = matching_bracket - 1;
                }
            }

            '.' => {
                if debug {
                    println!("(BFDB) STDOUT: '{}'", val as char);  // Intercept output
                } else {
                    print!("{}", val as char);
                }
            }
            ',' => {
                mem[ptr] = Wrapping(reader.next().unwrap().unwrap());
            }

            _ => (),
        }

        pc += 1;
    }
}
