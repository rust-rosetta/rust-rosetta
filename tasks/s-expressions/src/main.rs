//! This implementation isn't based on anything in particular, although it's probably informed by a
//! lot of Rust's JSON encoding code.  It should be very fast (both encoding and decoding the toy
//! example here takes under a microsecond on my machine) and tries to avoid unnecessary allocation.
//!
//! In a real implementation, most of this would be private, with only a few visible functions, and
//! there would be somewhat nicer signatures (in particular, the fact that `ParseContext` has to be
//! mutable would get annoying in real code pretty quickly, so it would probably be split out).
//!
//! It supports the ability to read individual atoms, not just lists, although whether this is
//! useful is questionable.
//!
//! Caveats: Does not support symbols vs. non-symbols (it wouldn't be hard, but it would greatly
//! complicate setting up our test structure since we'd have to force it to go through functions
//! that checked to make sure `Symbol`s couldn't have spaces, or slow down our parser by checking
//! for this information each time, which is obnoxious).  Does not support string escaping, because
//! the decoding technique doesn't allocate extra space for strings.  Does support numbers, but
//! only float types (supporting more types is possible but would complicate the code
//! significantly).
#![feature(rustc_private)]
#![feature(test)]

extern crate arena;
extern crate test;

use arena::TypedArena;

use std::io;
use std::num::FpCategory;
use std::str::FromStr;
use self::SExp::*;
use self::Error::*;
use self::Token::*;

/// The actual `SExp` structure.  Supports `f64`s, lists, and string literals.  Note that it takes
/// everything by reference, rather than owning it--this is mostly done just so we can allocate
/// `SExp`s statically (since we don't have to call `Vec`).  It does complicate the code a bit,
/// requiring us to have a `ParseContext` that holds an arena where lists are actually allocated.
#[derive(PartialEq,Debug)]
enum SExp<'a> {
    /// Float literal: 0.5
    F64(f64),

    /// List of SExps: ( a b c)
    List(&'a [SExp<'a>]),

    /// Plain old string literal: "abc"
    Str(&'a str),
}

/// Errors that can be thrown by the parser.
#[derive(PartialEq, Debug)]
enum Error {
    /// If the float is `NaN`, `Infinity`, etc.
    NoReprForFloat,

    /// Missing an end double quote during string parsing
    UnterminatedStringLiteral,

    /// Some other kind of I/O error
    IoError,

    /// ) appeared where it shouldn't (usually as the first token)
    IncorrectCloseDelimiter,

    /// Usually means a missing ), but could also mean there were no tokens at all.
    UnexpectedEOF,

    /// More tokens after the list is finished, or after a literal if there is no list.
    ExpectedEOF,
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Error {
        Error::IoError
    }
}

/// Tokens returned from the token stream.
#[derive(PartialEq, Debug)]
enum Token<'a> {
    /// Left parenthesis
    ListStart,

    /// Right parenthesis
    ListEnd,

    /// String or float literal, quotes removed.
    Literal(SExp<'a>),

    /// Stream is out of tokens.
    EOF,
}

/// An iterator over a string that yields a stream of Tokens.
///
/// Implementation note: it probably seems weird to store first, rest, AND string, since they should
/// all be derivable from string.  But see below.
#[derive(Copy, Clone, Debug)]
struct Tokens<'a> {
    /// The part of the string that still needs to be parsed
    string: &'a str,

    /// The first character to parse
    first: Option<char>,

    /// The rest of the string after the first character
    rest: &'a str,
}

impl<'a> Tokens<'a> {
    /// Initialize a token stream for a given string.
    fn new(string: &str) -> Tokens {
        let mut chars = string.chars();

        match chars.next() {
            Some(ch) => {
                Tokens {
                    string: string,
                    first: Some(ch),
                    rest: chars.as_str(),
                }
            }
            None => {
                Tokens {
                    string: string,
                    first: None,
                    rest: string,
                }
            }
        }
    }

    /// Utility function to update information in the iterator.  It might not be performant to keep
    /// rest cached, but there are times where we don't know exactly what string is (at least, not
    /// in a way that we can *safely* reconstruct it without allocating), so we keep both here.
    /// With some unsafe code we could probably get rid of one of them (and maybe first, too).
    fn update(&mut self, string: &'a str) {
        self.string = string;
        let mut chars = self.string.chars();

        if let Some(ch) = chars.next() {
            self.first = Some(ch);
            self.rest = chars.as_str();
        } else {
            self.first = None;
        };
    }

    /// This is where the lexing happens.  Note that it does not handle string escaping.
    fn next_token(&mut self) -> Result<Token<'a>, Error> {
        loop {
            match self.first {
                // List start
                Some('(') => {
                    self.update(self.rest);
                    return Ok(ListStart);
                }
                // List end
                Some(')') => {
                    self.update(self.rest);
                    return Ok(ListEnd);
                }
                // Quoted literal start
                Some('"') => {
                    // Split the string at most once.  This lets us get a
                    // reference to the next piece of the string without having
                    // to loop through the string again.
                    let mut iter = self.rest.splitn(2, '"');
                    // The first time splitn is run it will never return None, so this is safe.
                    let str = iter.next().unwrap();
                    match iter.next() {
                        // Extract the interior of the string without allocating.  If we want to
                        // handle string escaping, we would have to allocate at some point though.
                        Some(s) => {
                            self.update(s);
                            return Ok(Literal(Str(str)));
                        }
                        None => return Err(UnterminatedStringLiteral),
                    }
                }
                // Plain old literal start
                Some(c) => {
                    // Skip whitespace.  This could probably be made more efficient.
                    if c.is_whitespace() {
                        self.update(self.rest);
                        continue;
                    }
                    // Since we've exhausted all other possibilities, this must be a real literal.
                    // Unlike the quoted case, it's not an error to encounter EOF before whitespace.
                    let mut end_ch = None;
                    let str = {
                        let mut iter = self.string.splitn(2, |ch: char| {
                            let term = ch == ')' || ch == '(';
                            if term {
                                end_ch = Some(ch)
                            }
                            term || ch.is_whitespace()
                        });
                        // The first time splitn is run it will never return None, so this is safe.
                        let str = iter.next().unwrap();
                        self.rest = iter.next().unwrap_or("");
                        str
                    };
                    match end_ch {
                        // self.string will be incorrect in the Some(_) case.  The only reason it's
                        // okay is because the next time next() is called in this case, we know it
                        // will be '(' or ')', so it will never reach any code that actually looks
                        // at self.string.  In a real implementation this would be enforced by
                        // visibility rules.
                        Some(_) => self.first = end_ch,
                        None => self.update(self.rest),
                    }
                    return Ok(Literal(parse_literal(str)));
                }
                None => return Ok(EOF),
            }
        }
    }
}

/// This is not the most efficient way to do this, because we end up going over numeric literals
/// twice, but it avoids having to write our own number parsing logic.
fn parse_literal(literal: &str) -> SExp {
    match literal.bytes().next() {
        Some(b'0'...b'9') |
        Some(b'-') => {
            match f64::from_str(literal) {
                Ok(f) => F64(f),
                Err(_) => Str(literal),
            }
        }
        _ => Str(literal),
    }
}

/// Parse context, holds information required by the parser (and owns any allocations it makes)
struct ParseContext<'a> {
    /// The string being parsed.  Not required, but convenient.
    string: &'a str,

    /// Arena holding any allocations made by the parser.
    arena: Option<TypedArena<Vec<SExp<'a>>>>,

    /// Stored in the parse context so it can be reused once allocated.
    stack: Vec<Vec<SExp<'a>>>,
}

impl<'a> ParseContext<'a> {
    /// Create a new parse context from a given string
    fn new(string: &'a str) -> ParseContext<'a> {
        ParseContext {
            string: string,
            arena: None,
            stack: Vec::new(),
        }
    }
}

impl<'a> SExp<'a> {
    /// Serialize a SExp.
    fn encode<T: io::Write>(&self, writer: &mut T) -> Result<(), Error> {
        match *self {
            F64(f) => {
                match f.classify() {
                    // We don't want to identify NaN, Infinity, etc. as floats.
                    FpCategory::Normal | FpCategory::Zero => {
                        try!(write!(writer, "{}", f));
                        Ok(())
                    }
                    _ => Err(Error::NoReprForFloat),
                }
            }
            List(ref l) => {
                // Writing a list is very straightforward--write a left parenthesis, then
                // recursively call encode on each member, and then write a right parenthesis.  The
                // only reason the logic is as long as it is is to make sure we don't write
                // unnecessary spaces between parentheses in the zero or one element cases.
                try!(write!(writer, "{}", '('));
                let mut iter = l.iter();
                if let Some(sexp) = iter.next() {
                    try!(sexp.encode(writer));
                    for sexp in iter {
                        try!(write!(writer, "{}", ' '));
                        try!(sexp.encode(writer));
                    }
                }
                try!(write!(writer, "{}", ')'));
                Ok(())
            }
            Str(s) => {
                try!(write!(writer, "\"{}\"", s));
                Ok(())
            }
        }
    }

    /// Deserialize a SExp.
    fn parse(ctx: &'a mut ParseContext<'a>) -> Result<SExp<'a>, Error> {
        ctx.arena = Some(TypedArena::new());
        // Hopefully this unreachable! gets optimized out, because it should literally be
        // unreachable.
        let arena = match ctx.arena {
            Some(ref mut arena) => arena,
            None => unreachable!(),
        };
        let ParseContext { string, ref mut stack, .. } = *ctx;
        // Make sure the stack is cleared--we keep it in the context to avoid unnecessary
        // reallocation between parses (if you need to remember old parse information for a new
        // list, you can pass in a new context).
        stack.clear();
        let mut tokens = Tokens::new(string);
        // First, we check the very first token to see if we're parsing a full list.  It
        // simplifies parsing a lot in the subsequent code if we can assume that.
        let next = tokens.next_token();
        let mut list = match try!(next) {
            ListStart => Vec::new(),
            Literal(s) => {
                return if try!(tokens.next_token()) == EOF {
                    Ok(s)
                } else {
                    Err(ExpectedEOF)
                }
            }
            ListEnd => return Err(IncorrectCloseDelimiter),
            EOF => return Err(UnexpectedEOF),
        };

        // We know we're in a list if we got this far.
        loop {
            let tok = tokens.next_token();
            match try!(tok) {
                ListStart => {
                    // We push the previous context onto our stack when we start reading a new list.
                    stack.push(list);
                    list = Vec::new()
                }
                Literal(s) => list.push(s), // Plain old literal, push it onto the current list
                ListEnd => {
                    match stack.pop() { // Pop the old context off the stack on list end.
                        Some(mut l) => {
                            // We allocate a slot for the current list in our parse context (needed
                            // for safety) before pushing it onto its parent list.
                            l.push(List(&*arena.alloc(list)));
                            // Now reset the current list to the parent list
                            list = l;
                        }
                        // There was nothing on the stack, so we're at the end of the topmost list.
                        // The check to make sure there are no more tokens is required for
                        // correctness.
                        None => {
                            return match try!(tokens.next_token()) {
                                EOF => Ok(List(&*arena.alloc(list))),
                                _ => Err(ExpectedEOF),
                            }
                        }
                    }
                }
                // We encountered an EOF before the list ended--that's an error.
                EOF => return Err(UnexpectedEOF),
            }
        }
    }

    /// Convenience method for the common case where you just want to encode a SExp as a String.
    fn buffer_encode(&self) -> Result<String, Error> {
        let mut m = Vec::new();
        try!(self.encode(&mut m));
        // Because encode() only ever writes valid UTF-8, we can safely skip the secondary check we
        // normally have to do when converting from Vec<u8> to String.  If we didn't know that the
        // buffer was already UTF-8, we'd want to call container_as_str() here.
        unsafe { Ok(String::from_utf8_unchecked(m)) }
    }
}

const SEXP_STRUCT: SExp<'static> =
    List(&[List(&[Str("data"), Str("quoted data"), F64(123.), F64(4.5)]),
           List(&[Str("data"),
                  List(&[Str("!@#"), List(&[F64(4.5)]), Str("(more"), Str("data)")])])]);

fn try_encode() -> Result<String, Error> {
    SEXP_STRUCT.buffer_encode()
}

const SEXP_STRING_IN: &'static str = r#"((data "quoted data" 123 4.5)
(data (!@# (4.5) "(more" "data)")))"#;

fn try_decode<'a>(ctx: &'a mut ParseContext<'a>) -> Result<SExp<'a>, Error> {
    SExp::parse(ctx)
}

fn main() {
    println!("{:?}", try_encode());
    let ctx = &mut ParseContext::new(SEXP_STRING_IN);
    println!("{:?}", try_decode(ctx));
}

#[bench]
fn bench_decode(b: &mut test::Bencher) {
    b.iter(|| {
        let ctx = &mut ParseContext::new(SEXP_STRING_IN);
        assert!(try_decode(ctx).is_ok());
    })
}

#[bench]
fn bench_encode(b: &mut test::Bencher) {
    b.iter(|| {
        assert!(try_encode().is_ok());
    })
}

#[test]
fn test_sexp_encode() {
    const SEXP_STRING: &'static str =
        r#"(("data" "quoted data" 123 4.5) ("data" ("!@#" (4.5) "(more" "data)")))"#;
    assert_eq!(Ok(SEXP_STRING.to_string()), try_encode());
}

#[test]
fn test_sexp_decode() {
    let ctx = &mut ParseContext::new(SEXP_STRING_IN);
    assert_eq!(Ok(SEXP_STRUCT), try_decode(ctx));
}
