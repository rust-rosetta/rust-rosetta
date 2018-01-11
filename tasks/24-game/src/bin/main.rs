//! Implements with a recursive descent parser for a simple calculator (+ - * /) using the
//! [shunting yard algorithm]. It follows operator precedence (i.e. 2 + 3 * 3 = 11), understands
//! negation (-5 + 6 = 1), ignores whitespace and allows the use of parentheses.
//!
//! [shunting yard algorithm]: http://www.engr.mun.ca/~theo/Misc/exp_parsing.htm.

extern crate rand;

use std::f32;
use std::cmp::Ordering::{self, Greater};
use std::iter::Peekable;
use std::str::CharIndices;

use rand::seq;

fn main() {
    use std::io;

    let mut rng = rand::thread_rng();
    let input = io::stdin();

    loop {
        let mut sample = seq::sample_iter(&mut rng, (1u32..10), 4).unwrap();

        println!("make 24 by combining the following 4 numbers with + - * / or (q)uit");
        println!("{:?}", sample);
        let mut line = String::new();
        let _ = input.read_line(&mut line).unwrap();
        match line.trim() {
            "q" => break,
            input => {
                if check_values(&mut sample[..], input) {
                    match Parser::new(input).parse() {
                        Ok(i) => {
                            if (i - 24.).abs() < f32::EPSILON {
                                println!("you made it!");
                            } else {
                                println!("you entered {}, try again!", i);
                            }
                        }
                        Err(s) => println!("{}", s),
                    };
                } else {
                    println!("unrecognized input, try again")
                }
            }
        }
    }
}

/// Returns true if the entered expression uses the values contained in sample
pub fn check_values(sample: &mut [u32], input: &str) -> bool {
    let lex = Lexer::new(input);

    let mut numbers_used = lex.filter_map(|(_, a)| {
            match a {
                Token::Int(i) => Some(i),
                _ => None,
            }
        })
        .collect::<Vec<u32>>();

    numbers_used.sort();
    sample.sort();
    numbers_used == sample
}

/// the tokens that our parser is going to recognize
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Slash,
    Star,
    Unknown,
    Int(u32),
}

impl Token {
    /// are tokens associated to a binary operation?
    fn is_binary(&self) -> bool {
        match *self {
            Token::Plus | Token::Minus | Token::Slash | Token::Star => true,
            _ => false,
        }
    }
}

pub struct Lexer<'a> {
    input: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.char_indices().peekable() }
    }

    fn expect<I>(iter: &mut I, expected: &[Token]) -> Result<Token, String>
        where I: Iterator<Item = (usize, Token)>
    {
        match iter.next() {
            Some((_, a)) if expected.contains(&a) => Ok(a),
            Some((n, other)) => {
                Err(format!("Parsing error: {:?} was unexpected at offset {}", other, n))
            }
            None => Err("unexpected end of token list".into()),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (usize, Token);

    fn next(&mut self) -> Option<(usize, Token)> {
        if let Some((idx, c)) = self.input
            .by_ref()
            .skip_while(|&(_, c)| c.is_whitespace())
            .next() {
            let ret = match c {
                '(' => Token::LParen,
                ')' => Token::RParen,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '/' => Token::Slash,
                '*' => Token::Star,
                d @ '0'...'9' => {
                    let mut val = d.to_digit(10).unwrap();
                    while let Some(dg) = self.input
                        .by_ref()
                        .peek()
                        .and_then(|&(_, di)| di.to_digit(10)) {
                        val = val * 10 + dg;
                        self.input.by_ref().next();
                    }
                    Token::Int(val)
                }
                _ => Token::Unknown,
            };
            Some((idx, ret))
        } else {
            None
        }
    }
}

/// Operators are a "higher level" concept than tokens as they define the semantics of the
/// expression language e.g. token "Minus" can correspond to the unary Neg Operator (-a) or to the
/// binary Sub operator (a - b)
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Operator {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Sentinel,
}

impl Operator {
    fn precedence(&self) -> usize {
        match *self {
            Operator::Sentinel => 0,
            Operator::Add | Operator::Sub => 1,
            Operator::Neg => 2,
            Operator::Mul | Operator::Div => 3,
        }
    }
}

/// Operator precedence for binary operators:
/// * if x has higher precedence than y precedence for x > precedence for y
/// * if x and y have equal precedence the first one has precedence
/// e.g. in expression (4 / 2 * 2) operators * and / have the same precedence, but the operations
/// must be performed in the order they appear (division first, multiplication second) otherwise
/// results are different
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Operator) -> Option<Ordering> {
        match (self.precedence(), other.precedence()) {
            (a, b) if a == b => Some(Greater),
            (a, b) => a.partial_cmp(&b),
        }
    }
}

/// Recursive descent parser with the shunting yard algorithm as explained in the crate
/// documentation. I followed the names of the methods as closely as possible vs. the pseudo-code
/// that illustrates the algorithm.
pub struct Parser<'a> {
    operators: Vec<Operator>,
    operands: Vec<f32>,
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser {
            operators: vec![],
            operands: vec![],
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<f32, String> {
        self.operators.push(Operator::Sentinel);
        try!(self.e());
        self.operands
            .last()
            .cloned()
            .ok_or_else(|| String::from("something went wrong, got no result"))
    }

    fn e(&mut self) -> Result<(), String> {
        try!(self.p());
        while let Some(&(_, x)) = self.lexer.by_ref().peek() {
            if !x.is_binary() {
                break;
            }

            let op = match x {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Sub,
                Token::Star => Operator::Mul,
                Token::Slash => Operator::Div,
                // there are no other binary operators
                _ => unreachable!(),
            };
            self.push_operator(op);

            // Consume the peeked value
            self.lexer.by_ref().next();
            try!(self.p());
        }

        while let Some(&op) = self.operators.last() {
            if op == Operator::Sentinel {
                return Ok(());
            }
            self.pop_operator();
        }
        unreachable!() // algorithm fail: reached the end without finding
                       // the sentinel
    }

    fn p(&mut self) -> Result<(), String> {
        match self.lexer.by_ref().next() {
            Some((_, Token::Int(n))) => self.operands.push(n as f32),
            Some((_, Token::LParen)) => {
                self.operators.push(Operator::Sentinel);
                try!(self.e());
                try!(Lexer::expect(&mut self.lexer, &[Token::RParen]));
                self.operators.pop();
            }
            Some((_, Token::Minus)) => {
                self.push_operator(Operator::Neg);
                try!(self.p());
            }
            Some((p, e)) => return Err(format!("unexpected token {:?} at pos {}", e, p)),
            _ => return Err("unexpected end of command".to_string()),
        }
        Ok(())
    }

    fn pop_operator(&mut self) {
        match self.operators.pop() {
            Some(Operator::Add) => self.binary_op(|t1, t2| t1 + t2),
            Some(Operator::Sub) => self.binary_op(|t1, t2| t1 - t2),
            Some(Operator::Mul) => self.binary_op(|t1, t2| t1 * t2),
            Some(Operator::Div) => self.binary_op(|t1, t2| t1 / t2),
            Some(Operator::Neg) => self.unary_op(|t1| -t1),
            _ => unreachable!(),
        }
    }

    fn push_operator(&mut self, op: Operator) {
        match self.operators.last() {
            Some(&last_op) if last_op > op => self.pop_operator(),
            _ => {}
        }
        self.operators.push(op);
    }

    #[inline]
    fn binary_op<F>(&mut self, op: F)
        where F: Fn(f32, f32) -> f32
    {
        match (self.operands.pop(), self.operands.pop()) {
            (Some(t1), Some(t2)) => self.operands.push(op(t2, t1)),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn unary_op<F>(&mut self, op: F)
        where F: Fn(f32) -> f32
    {
        match self.operands.pop() {
            Some(t1) => self.operands.push(op(t1)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, Lexer, Parser};
    use super::Operator::{Add, Sub, Mul, Div};
    use super::check_values;
    use super::Token::{LParen, RParen, Plus, Slash, Star, Int};

    #[test]
    fn test_precedence() {
        assert!(Mul > Add);
        // Ordering matters for these pairs
        assert!(Mul > Div);
        assert!(Div > Mul);

        assert!(Add > Sub);
        assert!(Sub > Add);

        assert!(!(Mul < Div));
    }

    #[test]
    fn lexer_iter() {
        // test read token and character's offset in the iterator
        let t = |lex: &mut Lexer, exp_tok: Token, exp_pos: usize| {
            assert_eq!(lex.next(), Some((exp_pos, exp_tok)));
        };

        let tok = &mut Lexer::new("  15 + 4");
        t(tok, Int(15), 2);
        t(tok, Plus, 5);
        let read = Lexer::expect(tok, &[LParen, Int(4), RParen]);
        assert_eq!(read, Ok(Int(4)));

        let mut tok = Lexer::new("");
        assert_eq!(tok.next(), None);

        let tok = &mut Lexer::new("     ");
        assert_eq!(tok.next(), None);

        let tok = &mut Lexer::new("2 * (3+4/2)");
        t(tok, Int(2), 0);
        t(tok, Star, 2);
        t(tok, LParen, 4);
        t(tok, Int(3), 5);
        t(tok, Plus, 6);
        t(tok, Int(4), 7);
        t(tok, Slash, 8);
        t(tok, Int(2), 9);
        t(tok, RParen, 10);
    }

    #[test]
    fn parse() {
        fn t(input: &str, expected: Result<f32, String>) {
            assert_eq!(Parser::new(input).parse(), expected)
        }

        t("2+2", Ok(4.));
        t("2+3*4", Ok(14.));
        t("4*(3+2)", Ok(20.));
        t("5/(3+2)*3", Ok(3.));
        t("2++12", Err("unexpected token Plus at pos 2".to_string()));
        t("-2+12", Ok(10.));
        t("-2*(2+3)", Ok(-10.));

        // Testing precedence
        t("4 / 2 * 2", Ok(4.));
        t("2 * 2 / 4", Ok(1.));
    }

    #[test]
    fn try_check_values() {
        let m = &mut [1, 2, 3, 4];
        assert!(check_values(m, "1+3 -(4/2)"));
        // new testcase for #314
        assert!(check_values(m, "1+2+3+4"));
    }
}
