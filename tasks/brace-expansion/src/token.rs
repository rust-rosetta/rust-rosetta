use std::char;

use crate::{Branches, CLOSE_CHAR, OPEN_CHAR, SEPARATOR};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Open,
    Close,
    Separator,
    Payload(String),
    Branches(Branches),
}

impl From<char> for Token {
    fn from(ch: char) -> Token {
        match ch {
            OPEN_CHAR => Token::Open,
            CLOSE_CHAR => Token::Close,
            SEPARATOR => Token::Separator,
            _ => panic!("Non tokenizable char!"),
        }
    }
}

impl From<Branches> for Token {
    fn from(branches: Branches) -> Token {
        Token::Branches(branches)
    }
}

impl From<Token> for String {
    fn from(token: Token) -> String {
        match token {
            Token::Open => OPEN_CHAR.to_string(),
            Token::Close => CLOSE_CHAR.to_string(),
            Token::Separator => SEPARATOR.to_string(),
            Token::Payload(text) => text,
            Token::Branches(_) => panic!("Cannot convert to String!"),
        }
    }
}

impl From<Token> for Vec<String> {
    fn from(token: Token) -> Vec<String> {
        match token {
            Token::Branches(branches) => branches.into(),
            _ => {
                let frag: String = token.into();
                vec![frag]
            }
        }
    }
}
