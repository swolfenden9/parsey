use parsey::{Ast, Parser, TokenStream};

#[test]
fn two_bit() {
    use MyToken::{One, Zero};
    use TwoBit::{OneOne, OneZero, ZeroOne, ZeroZero};

    let tokens = vec![Zero, Zero, Zero, One, One, Zero, One, One];
    let parser = MyParser::new(tokens);
    let ast = parser.parse().unwrap();
    assert_eq!(ast, Root(vec![ZeroZero, ZeroOne, OneZero, OneOne]));
}

#[derive(Debug, PartialEq)]
pub enum MyToken {
    Zero,
    One,
}

#[derive(Debug, PartialEq)]
pub struct MyError;

pub struct MyParser {
    tokens: Vec<MyToken>,
}

impl MyParser {
    pub fn new(tokens: Vec<MyToken>) -> Self {
        tokens.into()
    }
}

impl Parser<MyToken, MyError> for MyParser {
    type Root = Root;

    fn expect(
        peekable_parser: &mut TokenStream<Self, MyToken, MyError>,
        token: MyToken,
    ) -> Result<(), MyError> {
        if peekable_parser.peek() == Some(&token) {
            peekable_parser.next();
            Ok(())
        } else {
            Err(MyError)
        }
    }
}

impl Iterator for MyParser {
    type Item = MyToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
    }
}

impl From<Vec<MyToken>> for MyParser {
    fn from(mut value: Vec<MyToken>) -> Self {
        value.reverse();
        Self { tokens: value }
    }
}

#[derive(Debug, PartialEq)]
pub struct Root(Vec<TwoBit>);

#[derive(Debug, PartialEq)]
pub enum TwoBit {
    ZeroZero,
    ZeroOne,
    OneZero,
    OneOne,
}

impl Ast<MyToken, MyError> for Root {
    fn parse<P>(parser: &mut TokenStream<P, MyToken, MyError>) -> Result<Self, MyError>
    where
        P: Parser<MyToken, MyError>,
    {
        let mut two_bits = vec![];
        while parser.peek().is_some() {
            two_bits.push(TwoBit::parse(parser)?);
        }
        Ok(Self(two_bits))
    }
}

impl parsey::Ast<MyToken, MyError> for TwoBit {
    fn parse<P>(parser: &mut TokenStream<P, MyToken, MyError>) -> Result<Self, MyError>
    where
        P: parsey::Parser<MyToken, MyError>,
    {
        match parser.next() {
            Some(MyToken::Zero) => match parser.next() {
                Some(MyToken::Zero) => Ok(TwoBit::ZeroZero),
                Some(MyToken::One) => Ok(TwoBit::ZeroOne),
                _ => Err(MyError),
            },
            Some(MyToken::One) => match parser.next() {
                Some(MyToken::Zero) => Ok(TwoBit::OneZero),
                Some(MyToken::One) => Ok(TwoBit::OneOne),
                _ => Err(MyError),
            },
            _ => Err(MyError),
        }
    }
}
