# Parsey

A simple parser-generator framework. Much work to be done!

## Examples

A simple zero-one grammar.

```rust
use parsey::Ast;
use std::{fmt, iter::Peekable};

fn main() {
    use Token::{One, Zero};
    let tokens = vec![Zero, Zero, Zero, One, One, Zero, One, One];
    let p = Parser::new(tokens);
    let ast = parsey::Parser::parse(p).unwrap();
    println!("{:?}", ast);
}

#[derive(Debug)]
pub enum Token {
    Zero,
    One,
}

/// The main parser struct that holds the tokens to be parsed.
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();
        Self { tokens }
    }
}

impl parsey::Parser<Token, Error> for Parser {
    type Root = Program;
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
    }
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

#[derive(Debug)]
pub struct Program(Vec<Stat>);

impl Ast<Token, Error> for Program {
    fn parse<P>(parser: &mut Peekable<P>) -> Result<Self, Error>
    where
        P: parsey::Parser<Token, Error>,
    {
        let mut statements = vec![];
        while let Some(_) = parser.peek() {
            let result = Stat::parse(parser)?;
            statements.push(result);
        }

        Ok(Self(statements))
    }
}

#[derive(Debug)]
pub enum Stat {
    ZeroZero,
    ZeroOne,
    OneZero,
    OneOne,
}

impl Ast<Token, Error> for Stat {
    fn parse<P>(parser: &mut Peekable<P>) -> Result<Self, Error>
    where
        P: parsey::Parser<Token, Error>,
    {
        match parser.next() {
            Some(Token::Zero) => match parser.next() {
                Some(Token::Zero) => Ok(Stat::ZeroZero),
                Some(Token::One) => Ok(Stat::ZeroOne),
                _ => Err(Error),
            },
            Some(Token::One) => match parser.next() {
                Some(Token::Zero) => Ok(Stat::OneZero),
                Some(Token::One) => Ok(Stat::OneOne),
                _ => Err(Error),
            },
            _ => Err(Error),
        }
    }
}
```
