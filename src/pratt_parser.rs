use std::fmt;

#[derive(Debug)]
pub enum Token {
    Atom(char),
    Op(char),
    Eof,
}

pub enum S {
    Atom(char),
    Cons(char, Vec<S>)
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f,"({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f,")")
            }
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c{
                _ => Token::Op(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();

        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
}