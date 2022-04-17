use std::fmt::{Debug, Formatter};
use std::vec;

#[derive(Clone, Copy)]
enum Token {
    NUM(f64),
    INT(u64),
    ADD,
    SUB,
    MUL,
    DIV,
    NUL,
}

use Token::*;

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NUM(n) => format!("NUM: {}", n),
            INT(n) => format!("INT: {}", n),
            ADD => "ADD".to_string(),
            SUB => "SUB".to_string(),
            MUL => "MUL".to_string(),
            DIV => "DIV".to_string(),
            NUL => "NUL".to_string(),
        })
    }
}

type Tokens = Vec<Token>;

fn tokenize(input: &'static str) -> Tokens {
    let mut result = vec![NUL];

    for c in input.chars() {
        let t_type = get_type(&c);
        match t_type {
            Some(INT(n)) => {
                if let INT(p) = result.last().unwrap() {
                    let new = p * 10 + n;
                    if let Some(prev) = result.last_mut() {
                        *prev = INT(new);
                    }
                } else {
                    result.push(INT(n));
                }
            },
            Some(token) => result.push(token),
            None => continue,
        }
    };

    result[1..].to_vec()
}

fn get_type(c: &char) -> Option<Token> {
    if c.is_digit(10) {
        return Some(Token::INT(c.to_digit(10).unwrap() as u64))
    };

    match c {
        '+' => Some(Token::ADD),
        '-' => Some(Token::SUB),
        '*' => Some(Token::MUL),
        '/' => Some(Token::DIV),
        _ => None
    }
}

fn main() {
    let expr = "69 + 420 - 666";
    let tokens = tokenize(expr);
    println!("{:?}", tokens);
}
