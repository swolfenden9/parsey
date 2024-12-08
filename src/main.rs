use parsey::{Ast, Parser, PeekableParser};

pub fn main() {
    let tokens = vec![Token::One, Token::Zero, Token::One, Token::Zero];
    let parser = MyParser::new(tokens);
    match parser.parse() {
        Ok(ast) => println!("Ast: {:?}", ast),
        Err(e) => eprintln!("Parsing error: {:?}", e),
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Zero,
    One,
}

#[derive(Debug)]
pub struct Error;

pub struct MyParser {
    tokens: Vec<Token>,
}

impl MyParser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();
        Self { tokens }
    }
}

impl Parser<Token, Error> for MyParser {
    type Root = Root;

    fn expect(
        peekable_parser: &mut PeekableParser<Self, Token, Error>,
        token: Token,
    ) -> Result<(), Error> {
        if peekable_parser.peek() == Some(&token) {
            peekable_parser.next();
            Ok(())
        } else {
            Err(Error)
        }
    }
}

impl Iterator for MyParser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
    }
}

#[derive(Debug)]
pub struct Root(Vec<Statement>);

impl Ast<Token, Error> for Root {
    fn parse<P>(parser: &mut PeekableParser<P, Token, Error>) -> Result<Self, Error>
    where
        P: Parser<Token, Error>,
    {
        let mut statements = vec![];
        while parser.peek().is_some() {
            statements.push(Statement::parse(parser)?);
        }
        Ok(Self(statements))
    }
}

#[derive(Debug)]
pub enum Statement {
    ZeroZero,
    ZeroOne,
    OneZero,
    OneOne,
}

impl Ast<Token, Error> for Statement {
    fn parse<P>(_parser: &mut PeekableParser<P, Token, Error>) -> Result<Self, Error>
    where
        P: Parser<Token, Error>,
    {
        todo!()
    }
}
