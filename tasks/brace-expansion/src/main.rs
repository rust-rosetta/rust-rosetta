const OPEN_CHAR: char = '{';
const CLOSE_CHAR: char = '}';
const SEPARATOR: char = ',';
const ESCAPE: char = '\\';

#[derive(Debug, PartialEq, Clone)]
enum Token {
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

#[derive(Debug, PartialEq, Clone)]
struct Branches {
    tokens: Vec<Vec<Token>>,
}

impl Branches {
    fn new() -> Branches {
        Branches { tokens: Vec::new() }
    }

    fn add_branch(&mut self, branch: Vec<Token>) {
        self.tokens.push(branch);
    }

    fn from(tokens: &Vec<Token>) -> Branches {
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

impl From<Branches> for Token {
    fn from(branches: Branches) -> Token {
        Token::Branches(branches)
    }
}

impl From<Vec<Token>> for Branches {
    fn from(tokens: Vec<Token>) -> Branches {
        Branches::from(&tokens)
    }
}

impl From<Token> for String {
    fn from(token: Token) -> String {
        match token {
            Token::Branches(_) => panic!("Cannot convert to String!"),
            Token::Payload(text) => text,
            Token::Open => OPEN_CHAR.to_string(),
            Token::Close => CLOSE_CHAR.to_string(),
            Token::Separator => SEPARATOR.to_string(),
        }
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
            let mut vec_string = output(tokens);
            vec.append(&mut vec_string);
        }
        if braces {
            vec.iter()
                .map(|line| format!("{}{}{}", OPEN_CHAR, line, CLOSE_CHAR))
                .collect::<Vec<String>>()
        } else {
            vec
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

fn tokenize(string: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = string.chars();
    let mut payload = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            OPEN_CHAR | SEPARATOR | CLOSE_CHAR => {
                if payload.len() > 0 {
                    tokens.push(Token::Payload(payload));
                }
                payload = String::new();
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

fn output(tokens: Vec<Token>) -> Vec<String> {
    let mut output: Vec<String> = vec![String::new()];
    for token in tokens {
        let mut aux: Vec<String> = Vec::new();
        let strings: Vec<String> = token.into();
        for root in &output {
            for string in &strings {
                aux.push({ format!("{}{}", root, string) });
            }
        }
        output = aux;
    }
    output
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let tokens: Vec<Token> = tokenize(&input);
    // println!("Tokens:\n{:#?}", tokens);

    let output = output(tokens);
    for line in &output {
        println!("{}", line);
    }
}
