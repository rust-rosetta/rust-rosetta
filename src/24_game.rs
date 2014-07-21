// Implements http://rosettacode.org/wiki/24_game
// with a recursive descent parser for a simple calculator (+ - * /)
// using the shunting yard algorithm as explained on
// http://www.engr.mun.ca/~theo/Misc/exp_parsing.htm
// It follows operator precedence (i.e. 2 + 3 * 3 = 11),
// understands negation (-5 + 6 = 1), ignores whitespace
// and allows the use of parentheses

#![feature(macro_rules)]
use std::char;

#[cfg(not(test))]
use std::rand;

#[cfg(not(test))]
fn main() {
    let mut rng=rand::task_rng();
    let mut input = std::io::stdin();

    loop {
        let mut sample = rand::sample(&mut rng, range(1u, 10), 4);

        println!("make 24 by combining the following 4 numbers with + - * / or (q)uit");
        println!("{}", sample);

        let line = input.read_line().unwrap();
        match line.as_slice().trim() {
            "q" => break,
            input if check_values(sample.as_mut_slice(), input) => {
                    let mut p = Parser :: new(input);
                    match p.parse() {
                        Ok(i) if i==24f32 => println!("you made it!"),
                        Ok(i) => println!("you entered {}, try again!", i),
                        Err(s)  => println!("{}",s)
                    };
                }
            _ => println!("unrecognized input, try again")
        }
    }
}

// Returns true if the entered expression uses the values contained in sample
fn check_values(sample:&mut [uint], input:&str) -> bool {
    let lex=Lexer::new(input);

    let mut numbers_used : Vec<uint> = lex.filter_map(|a| match a {
                                                        Int(i)  => Some(i),
                                                        _       => None
                                                    }).collect();
    numbers_used.sort();
    sample.sort();
    numbers_used.as_slice() == sample
}

// the tokens that our parser is going to recognize
#[deriving(PartialEq,Eq,Show)]
enum Token {LParen, RParen, Plus, Minus, Slash, Star, Int(uint)}

impl Token {
   // are tokens associated to a binary operation?
   fn is_binary(&self) -> bool {
        match self {
            &Plus | &Minus | &Slash | &Star => true,
            _ => false
        }
    }
}

#[inline]
// map a character to its corresponding token
fn single_char_to_token(ch: char) -> Option<Token> {
    match ch {
            '(' => Some(LParen),
            ')' => Some(RParen),
            '+' => Some(Plus),
            '-' => Some(Minus),
            '/' => Some(Slash),
            '*' => Some(Star),
            _    => None
    }
}

// Lexer reads an expression like (a + b) / c * d
// as an iterator on the tokens that compose it
// Int(a), LParen, Plus, Int(b), RParen...
struct Lexer<'a> {
    input_str:  &'a str,
    offset:     uint
}

impl <'a> Lexer<'a> {
    fn new(input_str: &str) -> Lexer {
        Lexer { input_str: input_str, offset: 0u }
    }

    fn expect(&mut self, expected:&[Token]) -> Result<Token, String> {
        let n = self.offset;
        match self.next() {
            Some(a) if expected.contains(&a)  => Ok(a),
            other  => Err(format!("Parsing error: {} was unexpected at offset {}", other, n))
        }
    }
}

impl <'a> Iterator<Token> for Lexer<'a> {
    fn next(&mut self) -> Option<Token> {
        // slice the original string starting from the current offset
        let remaining_input=self.input_str.slice_from(self.offset);

        // keep track of the offset while advancing chars, with enumerate()
        let ch_iter=remaining_input.chars().enumerate();

        // advance to the next non-whitespace character
        let mut trimmed=ch_iter.skip_while(|&(_, ch)| ch.is_whitespace());

        let (tok, cur_offset) = match trimmed.next() {
            // found digit, check if there are others
            // and transform them to a uint
            Some((o, d)) if d.is_digit() => {
                let (mut val, mut offset)=(char::to_digit(d, 10).unwrap(), o);
                for (idx, ch) in trimmed {
                    if ch.is_digit() {val=val*10 + char::to_digit(ch, 10).unwrap();}
                    else {
                        offset=idx;
                        break;
                    }
                }
                (Some(Int(val)), offset)
            },
            // found non-digit, try transforming it to the corresponding token
            Some((o, t)) => (single_char_to_token(t), o+1),
            _   => (None, 0u)
        };
        // update the offset for the next iteration
        self.offset += cur_offset;
        tok
    }
}

// operators are a "higher level" concept than tokens
// as they define the semantics of the expression language
// e.g. token "Minus" can correspond to the unary Neg Operator (-a)
// or to the binary Sub operator (a - b)
#[deriving(PartialEq, Eq)]
enum Operator {Neg, Add, Sub, Mul, Div, Sentinel}

#[deriving(PartialEq, Eq)]
struct OperatorPrecedence(Operator);

impl Operator {
     fn precedence(self) -> OperatorPrecedence  {
        OperatorPrecedence(self)
    }
}

impl OperatorPrecedence {
    fn prec(self) -> uint {
        match self {
            OperatorPrecedence(Sentinel) => 0u,
            OperatorPrecedence(Add) | OperatorPrecedence(Sub) => 1u,
            OperatorPrecedence(Neg)        => 2u,
            OperatorPrecedence(Mul) | OperatorPrecedence(Div) => 3u
        }
    }
}

/*
    Operator precedence for binary operators:
    * if x has higher precedence than y precedence for x > precedence for y
    * if x and y have equal precedence the first one has precedence
    e.g. in expression (4 / 2 * 2) operators * and / have the same precedence,
    but the operations must be performed in the order they appear
    (division first, multiplication second) otherwise results are different
*/
impl PartialOrd for OperatorPrecedence {
    fn partial_cmp(&self, other: &OperatorPrecedence) -> Option<Ordering> {
        let lower = match (self, other) {
            (&OperatorPrecedence(Mul), &OperatorPrecedence(Div)) => false,
            (&OperatorPrecedence(Div), &OperatorPrecedence(Mul)) => false,
            (&OperatorPrecedence(Add), &OperatorPrecedence(Sub)) => false,
            (&OperatorPrecedence(Sub), &OperatorPrecedence(Add)) => false,
            _  => self.prec() < other.prec()
        };

        if lower {
            Some(Less)
        } else if self == other {
            Some(Equal)
        } else {
            Some(Greater)
        }
    }
}

// recursive descent parser
// with the shunting yard algorithm as explained on
// http://www.engr.mun.ca/~theo/Misc/exp_parsing.htm
// I followed the names of the methods as closely as possible vs the pseudo-code
// that illustrates the algorithm
struct Parser<'a> {
    operators: Vec<Operator>,
    operands:  Vec<f32>,
    lexer:     Lexer<'a>
}

impl <'a> Parser<'a> {
    fn new(input_str: &str) -> Parser {
        Parser {
            operators:  Vec::new(),
            operands:   Vec::new(),
            lexer:      Lexer::new(input_str)
        }
    }

    fn parse(&mut self) -> Result<f32, String> {
        self.operators.push(Sentinel);
        try!(self.e());
        let res=self.operands.last();
        let ret=match res {
            Some(&r) => Ok(r),
            None    => Err("something went wrong, got no result".to_string())
        };
        ret
    }

    fn e(&mut self) -> Result<(), String> {
        try!(self.p());

        let mut n=self.lexer.next();

        while n.is_some() && n.unwrap().is_binary() {
            match n {
                Some(Plus) => self.push_operator(Add),
                Some(Minus) => self.push_operator(Sub),
                Some(Star) => self.push_operator(Mul),
                Some(Slash) => self.push_operator(Div),
                _ => unreachable!() //shouldn't get there (there are no other
                                    // binary operators
            };
            try!(self.p());

            let mut n_peek = self.lexer.peekable();
            if n_peek.peek().is_none() || !n_peek.peek().unwrap().is_binary() { break; }
            n = self.lexer.next();
        }
        while self.operators.last().is_some() && self.operators.last().unwrap() != &Sentinel {
            self.pop_operator();
        }
        Ok(())
    }

    fn p(&mut self) -> Result<(), String> {
        match self.lexer.next() {
            Some(Int(n)) => {
                self.operands.push(n as f32);
            },
            Some(LParen) => {
                self.operators.push(Sentinel);
                try!(self.e());
                try!(self.lexer.expect(&[RParen]));
                self.operators.pop();
            },
            Some(Minus) => {
                self.push_operator(Neg);
                try!(self.p());
            },
            Some(e) => return Err(format!("unexpected token {}", e)),
            _       => return Err("unexpected end of command".to_string())
            //Some(Minus) =>
        }
        Ok(())
    }

    fn pop_operator(&mut self) {
        match self.operators.pop() {
            Some(Add) => self.binary_op(|t1,t2| t1+t2),
            Some(Sub) => self.binary_op(|t1,t2| t1-t2),
            Some(Mul) => self.binary_op(|t1,t2| t1*t2),
            Some(Div) => self.binary_op(|t1,t2| t1/t2),
            Some(Neg) => self.unary_op(|t1| -t1),
            _         => unreachable!()
            }
    }

    fn push_operator(&mut self, op:Operator) {
        match self.operators.last() {
            Some(&a) if a.precedence() > op.precedence() => self.pop_operator(),
            _ => ()
        }
        self.operators.push(op);
    }

    #[inline]
    fn binary_op(&mut self, op:|f32,f32| -> f32) {
        let t1=self.operands.pop().unwrap();
        let t2=self.operands.pop().unwrap();
        self.operands.push(op(t2,t1));
    }

    #[inline]
    fn unary_op(&mut self, op:|f32| -> f32) {
        let t1=self.operands.pop().unwrap();
        self.operands.push(op(t1));
    }
}


#[test]
fn test_precedence() {
    assert!(Mul.precedence() > Add.precedence());
    assert!(Mul.precedence() > Div.precedence());
    assert!((Mul.precedence() < Div.precedence())==false);
}

#[test]
fn lexer_iter() {
    // test read token and character's offset in the iterator
    macro_rules! test_tok( ($tok:ident, $exp_tok:expr, $exp_pos:expr) =>
        ( assert_eq!(($tok.next(), $tok.offset), (Some($exp_tok), $exp_pos));))

    let mut tok=Lexer::new("  15 + 4");
    test_tok!(tok, Int(15), 4);
    test_tok!(tok, Plus, 6);
    let read=tok.expect(&[LParen,Int(4),RParen]);
    assert_eq!(read, Ok(Int(4)));

    let mut tok=Lexer::new("");
    assert_eq!(tok.next(), None);

    let mut tok=Lexer::new("     ");
    assert_eq!(tok.next(), None);

    let mut tok=Lexer::new("2 * (3+4/2)");
    test_tok!(tok, Int(2), 1);
    test_tok!(tok, Star, 3);
    test_tok!(tok, LParen, 5);
    test_tok!(tok, Int(3), 6);
    test_tok!(tok, Plus, 7);
    test_tok!(tok, Int(4), 8);
    test_tok!(tok, Slash, 9);
    test_tok!(tok, Int(2), 10);
    test_tok!(tok, RParen, 11);
}

#[test]
fn parse()
{
    assert_eq!(Parser::new("2+2").parse(), Ok(4.));
    assert_eq!(Parser::new("2+3*4").parse(), Ok(14.));
    assert_eq!(Parser::new("4*(3+2)").parse(), Ok(20.));
    assert_eq!(Parser::new("5/(3+2)*3").parse(), Ok(3.));
    assert_eq!(Parser::new("2++12").parse(), Err("unexpected token Plus".to_string()));
    assert_eq!(Parser::new("-2+12").parse(), Ok(10.));
    assert_eq!(Parser::new("-2*(2+3)").parse(), Ok(-10.));
}

#[test]
fn try_check_values() {
    let m = &mut [1,2,3,4];
    assert!(check_values(m, "1+3 -(4/2)"));
}
