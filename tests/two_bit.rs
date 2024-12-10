use parsey::{parse, require_next_n, Ast, Parser, TokenStream};

#[test]
pub fn two_bit() {
    use MyToken::{One, Zero};
    use TwoBit::{OneOne, OneZero, ZeroOne, ZeroZero};

    let tokens = vec![Zero, Zero, Zero, One, One, Zero, One, One];
    let ast = parse::<MyParser, MyToken, MyError>(tokens);
    assert_eq!(ast, Ok(Root(vec![ZeroZero, ZeroOne, OneZero, OneOne])));
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

impl Parser<MyToken, MyError> for MyParser {
    type Root = Root;

    fn expect(
        token_stream: &mut TokenStream<Self, MyToken, MyError>,
        token: MyToken,
    ) -> Result<(), MyError> {
        if token_stream.peek() == Some(&token) {
            token_stream.next();
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
    fn parse<P>(token_stream: &mut TokenStream<P, MyToken, MyError>) -> Result<Self, MyError>
    where
        P: Parser<MyToken, MyError>,
    {
        let mut two_bits = vec![];
        while !token_stream.is_empty() {
            two_bits.push(TwoBit::parse(token_stream)?);
        }
        Ok(Self(two_bits))
    }
}

impl Ast<MyToken, MyError> for TwoBit {
    fn parse<P>(token_stream: &mut TokenStream<P, MyToken, MyError>) -> Result<Self, MyError>
    where
        P: parsey::Parser<MyToken, MyError>,
    {
        use MyToken::*;
        use TwoBit::*;

        match require_next_n!(token_stream, 2, MyError) {
            [Zero, Zero] => Ok(ZeroZero),
            [Zero, One] => Ok(ZeroOne),
            [One, Zero] => Ok(OneZero),
            [One, One] => Ok(OneOne),
        }
    }
}
