use crate::{Branches, Token, CLOSE_CHAR, ESCAPE, OPEN_CHAR, SEPARATOR};

pub fn tokenize(string: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = string.chars();
    let mut payload = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            OPEN_CHAR | SEPARATOR | CLOSE_CHAR => {
                if payload.len() > 0 {
                    tokens.push(Token::Payload(payload));
                    payload = String::new();
                }

                if ch == CLOSE_CHAR {
                    let pos = tokens.iter().rposition(|token| *token == Token::Open);

                    if let Some(pos) = pos {
                        let branches: Branches = {
                            let mut to_branches = tokens.split_off(pos);
                            to_branches.remove(0);
                            to_branches
                        }
                        .into();
                        tokens.push(branches.into());
                    } else {
                        tokens.push(ch.into());
                    }
                } else {
                    tokens.push(ch.into());
                }
            }
            ESCAPE => {
                payload.push(ch);

                if let Some(next_char) = chars.next() {
                    payload.push(next_char);
                }
            }
            _ => payload.push(ch),
        }
    }

    let payload = payload.trim_end();

    if payload.len() > 0 {
        tokens.push(Token::Payload(payload.into()));
    }

    tokens
}

pub fn expand(tokens: Vec<Token>) -> Vec<String> {
    let mut output = vec![String::new()];

    for token in tokens {
        let mut aux: Vec<String> = Vec::new();
        let strings: Vec<String> = token.into();

        for root in &output {
            for string in &strings {
                aux.push(format!("{}{}", root, string));
            }
        }

        output = aux;
    }

    output
}
