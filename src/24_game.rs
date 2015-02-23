// Implements http://rosettacode.org/wiki/24_game
// with a recursive descent parser for a simple calculator (+ - * /)
// using the shunting yard algorithm as explained on
// http://www.engr.mun.ca/~theo/Misc/exp_parsing.htm
// It follows operator precedence (i.e. 2 + 3 * 3 = 11),
// understands negation (-5 + 6 = 1), ignores whitespace
// and allows the use of parentheses

// We use a glob import in our test module. Seperating tests into a seperate
// module enforces visibility restrictions so the test module can only access
// publically exported code, the same as any user of the code.

#![allow(unused_features)] // feature(rand) is used only in main


#![feature(unicode)]
#![feature(collections)]
#![feature(old_io)]

extern crate rand;

use std::cmp::Ordering::{self, Greater};
use std::char::CharExt;

#[cfg(not(test))]
fn main() {
    use std::old_io;

    let mut rng = rand::thread_rng();
    let mut input = old_io::stdin();

    loop {
        let mut sample = rand::sample(&mut rng, (1u32..10), 4);

        println!("make 24 by combining the following 4 numbers with + - * / or (q)uit");
        println!("{:?}", sample);

        let line = input.read_line().unwrap();
        match line.trim() {
            "q" => break,
            input => {
                if check_values(&mut sample[..], input) {
                    match Parser::new(input).parse() {
                        Ok(i) if i == 24. => println!("you made it!"),
                        Ok(i) => println!("you entered {}, try again!", i),
                        Err(s)  => println!("{}", s)
                    };
                } else {
                    println!("unrecognized input, try again")
                }
            }
        }
    }
}

// Returns true if the entered expression uses the values contained in sample
pub fn check_values(sample:&mut [u32], input:&str) -> bool {
    let lex = Lexer::new(input);

    let mut numbers_used = lex.filter_map(|a| {
        match a {
            Token::Int(i) => Some(i),
            _ => None
        }
    }).collect::<Vec<u32>>();

    numbers_used.sort();
    sample.sort();
    numbers_used == sample
}

// the tokens that our parser is going to recognize
#[derive(PartialEq,Eq,Debug, Copy)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Slash,
    Star,
    Int(u32)
}

impl Token {
   // are tokens associated to a binary operation?
   fn is_binary(&self) -> bool {
        match *self {
            Token::Plus | Token::Minus | Token::Slash | Token::Star => true,
            _ => false
        }
    }
}

trait Tokenable { fn as_token(&self) -> Option<Token>; }

// map a character to its corresponding token
impl Tokenable for char {
    #[inline]
    fn as_token(&self) -> Option<Token> {
        let tok = match *self {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Star,
            _ => return None
        };

        Some(tok)
    }
}

// Lexer reads an expression like (a + b) / c * d
// as an iterator on the tokens that compose it
// Int(a), LParen, Plus, Int(b), RParen...
#[derive(Copy)]
pub struct Lexer<'a> {
    input: &'a str,
    offset: usize
}

impl <'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input, offset: 0 }
    }

    fn expect(&mut self, expected:&[Token]) -> Result<Token, String> {
        let n = self.offset;
        match self.next() {
            Some(a) if expected.contains(&a)  => Ok(a),
            other  => Err(format!("Parsing error: {:?} was unexpected at offset {}",
                                  other,
                                  n))
        }
    }
}

impl <'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // slice the original string starting from the current offset
        let mut remaining = self.input[self.offset..]
                                      // keep track of the original indice
                                      .char_indices()
                                      // advance to the next non-whitespace char
                                      .skip_while(|&(_, ch)| ch.is_whitespace());

        let (tok, cur_offset) = match remaining.next() {
            // Found a digit. if there are others, transform them to `u32`
            Some((mut offset, ch)) if ch.is_numeric() => {
                let mut val = ch.to_digit(10).unwrap();
                let mut more = false;

                for (idx, ch) in remaining {
                    more = true;
                    if CharExt::is_numeric(ch) {
                        let digit = CharExt::to_digit(ch, 10).unwrap();
                        val = val * 10 + digit;
                    } else {
                        offset = idx;
                        break;
                    }
                }
                if !more {
                    offset = 1;
                }
                (Some(Token::Int(val)), offset)
            },
            // found non-digit, try transforming it to the corresponding token
            Some((o, ch)) => (ch.as_token(), o + 1),
            _   => (None, 0)
        };

        // update the offset for the next iteration
        self.offset += cur_offset;
        tok
    }
}

// Operators are a "higher level" concept than tokens as they define the
// semantics of the expression language e.g. token "Minus" can correspond to
// the unary Neg Operator (-a) or to the binary Sub operator (a - b)
#[derive(PartialEq, Eq, Copy)]
pub enum Operator {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Sentinel
}

impl Operator {
     fn precedence(&self) -> usize  {
        match *self {
            Operator::Sentinel => 0,
            Operator::Add | Operator::Sub => 1,
            Operator::Neg => 2,
            Operator::Mul | Operator::Div => 3
        }
    }
}

// Operator precedence for binary operators:
// * if x has higher precedence than y precedence for x > precedence for y
// * if x and y have equal precedence the first one has precedence
// e.g. in expression (4 / 2 * 2) operators * and / have the same precedence,
// but the operations must be performed in the order they appear
// (division first, multiplication second) otherwise results are different
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Operator) -> Option<Ordering> {
        match (self.precedence(), other.precedence()) {
            (a, b) if a == b => Some(Greater),
            (a, b) => a.partial_cmp(&b)
        }
    }
}

// recursive descent parser
// with the shunting yard algorithm as explained on
// http://www.engr.mun.ca/~theo/Misc/exp_parsing.htm
// I followed the names of the methods as closely as possible vs the pseudo-code
// that illustrates the algorithm
pub struct Parser<'a> {
    operators: Vec<Operator>,
    operands: Vec<f32>,
    lexer: Lexer<'a>
}

impl <'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser {
            operators: vec![],
            operands: vec![],
            lexer: Lexer::new(input)
        }
    }

    pub fn parse(&mut self) -> Result<f32, String> {
        self.operators.push(Operator::Sentinel);
        try!(self.e());
        return match self.operands.last() {
            Some(r) => Ok(*r),
            None => Err("something went wrong, got no result".to_string())
        }
    }

    fn e(&mut self) -> Result<(), String> {
        try!(self.p());

        loop {
            match self.lexer.peekable().peek() {
                Some(&x) if x.is_binary() => {
                    let op = match x {
                        Token::Plus => Operator::Add,
                        Token::Minus => Operator::Sub,
                        Token::Star => Operator::Mul,
                        Token::Slash => Operator::Div,
                        // there are no other binary operators
                        _ => unreachable!()
                    };

                    self.push_operator(op);

                    // Consume the peeked value
                    self.lexer.next();
                    try!(self.p());
                }
                _ => break
            }
        }

        loop {
            match self.operators.last() {
                Some(&op) if op != Operator::Sentinel => self.pop_operator(),
                _ => return Ok(())
            }
        }
    }

    fn p(&mut self) -> Result<(), String> {
        match self.lexer.next() {
            Some(Token::Int(n)) => self.operands.push(n as f32),
            Some(Token::LParen) => {
                self.operators.push(Operator::Sentinel);
                try!(self.e());
                try!(self.lexer.expect(&[Token::RParen]));
                self.operators.pop();
            },
            Some(Token::Minus) => {
                self.push_operator(Operator::Neg);
                try!(self.p());
            },
            Some(e) => return Err(format!("unexpected token {:?}", e)),
            _ => return Err("unexpected end of command".to_string())
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
            _ => unreachable!()
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
    fn binary_op<F>(&mut self, op: F) where F: Fn(f32, f32) -> f32 {
        match (self.operands.pop(), self.operands.pop()) {
            (Some(t1), Some(t2)) => self.operands.push(op(t2, t1)),
            _ => unreachable!()
        }
    }

    #[inline]
    fn unary_op<F>(&mut self, op: F) where F: Fn(f32) -> f32 {
        match self.operands.pop() {
            Some(t1) => self.operands.push(op(t1)),
            _ => unreachable!()
        }
    }
}


#[cfg(test)]
mod test {
    use super::{Token, Lexer, Parser};
    use super::Operator::{Add, Sub, Mul, Div};
    use super::{check_values};
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
            assert_eq!(lex.next(), Some(exp_tok));
            assert_eq!(lex.offset, exp_pos);
        };

        let tok = &mut Lexer::new("  15 + 4");
        t(tok, Int(15), 4);
        t(tok, Plus, 6);
        let read = tok.expect(&[LParen,Int(4),RParen]);
        assert_eq!(read, Ok(Int(4)));

        let mut tok = Lexer::new("");
        assert_eq!(tok.next(), None);

        let tok = &mut Lexer::new("     ");
        assert_eq!(tok.next(), None);

        let tok = &mut Lexer::new("2 * (3+4/2)");
        t(tok, Int(2), 1);
        t(tok, Star, 3);
        t(tok, LParen, 5);
        t(tok, Int(3), 6);
        t(tok, Plus, 7);
        t(tok, Int(4), 8);
        t(tok, Slash, 9);
        t(tok, Int(2), 10);
        t(tok, RParen, 11);
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
        t("2++12", Err("unexpected token Plus".to_string()));
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
