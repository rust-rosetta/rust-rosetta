use crate::{expand, Token, CLOSE_CHAR, OPEN_CHAR};

#[derive(Clone, PartialEq, Debug)]
pub struct Branches {
    tokens: Vec<Vec<Token>>,
}

impl Branches {
    pub fn new() -> Self {
        Branches { tokens: Vec::new() }
    }

    pub fn add_branch(&mut self, branch: Vec<Token>) {
        self.tokens.push(branch);
    }
}

impl From<Vec<Token>> for Branches {
    fn from(tokens: Vec<Token>) -> Branches {
        let mut branches = Branches::new();
        let mut tail = tokens.clone();

        while let Some(pos) = tail.iter().position(|token| *token == Token::Separator) {
            let mut rest = tail.split_off(pos);

            branches.add_branch(tail);
            rest.remove(0);
            tail = rest;
        }

        branches.add_branch(tail);
        branches
    }
}

impl From<Branches> for Vec<String> {
    fn from(branches: Branches) -> Vec<String> {
        let Branches {
            tokens: token_lines,
        } = branches;
        let mut vec: Vec<String> = Vec::new();
        let braces = {
            if token_lines.len() == 1 {
                true
            } else {
                false
            }
        };

        for tokens in token_lines {
            let mut vec_string = expand(tokens);
            vec.append(&mut vec_string);
        }

        if braces {
            vec.iter()
                .map(|line| format!("{}{}{}", OPEN_CHAR, line, CLOSE_CHAR))
                .collect()
        } else {
            vec
        }
    }
}
