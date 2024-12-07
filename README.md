# Parsey

`parsey` is a lightweight, `no_std` framework for creating custom parsers and abstract syntax trees (ASTs).

It provides two key traits: [`Parser`] and [`Ast`], which together form the foundation
for building parsers and representing the structure of parsed data.

## Key Features

- **Generic Parsing Framework:** Abstracts the process of parsing tokens into structured data.
- **Customizable AST Nodes:** Easily define nodes of your AST by implementing the [`Ast`] trait.
- **Integration with `no_std`:** Ideal for embedded or constrained environments.

## Getting Started

### Step 1: Implement the `Parser` Trait

Define a struct that will serve as your parser. This struct must implement the [`Parser`] trait,
which processes tokens and produces an AST.

```rust
use parsey::{Ast, Parser, PeekableParser};

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
    pub fn new(mut tokens: Vec<MyToken>) -> Self {
        tokens.reverse();
        Self { tokens }
    }
}

impl Parser<MyToken, MyError> for MyParser {
    type Root = Root;

    fn expect(
        peekable_parser: &mut PeekableParser<Self, MyToken, MyError>,
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
```

### Step 2: Define the AST Nodes

Create the structure for your AST by implementing the [`Ast`] trait for each node.
The root node must match the type defined in `Parser::Root`.

```rust
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
    fn parse<P>(parser: &mut PeekableParser<P, MyToken, MyError>) -> Result<Self, MyError>
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
    fn parse<P>(parser: &mut PeekableParser<P, MyToken, MyError>) -> Result<Self, MyError>
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
```

### Step 3: Parse Tokens

Use your parser to parse a sequence of tokens into an AST.

```rust
fn main() {
    use MyToken::{One, Zero};
    use TwoBit::{OneOne, OneZero, ZeroOne, ZeroZero};

    let tokens = vec![Zero, Zero, Zero, One, One, Zero, One, One];
    let parser = MyParser::new(tokens);
    let ast = parser.parse().unwrap();
    assert_eq!(ast, Root(vec![ZeroZero, ZeroOne, OneZero, OneOne]));
}
```

[`Parser`]: https://docs.rs/parsey/latest/parsey/trait.Parser.html
[`Ast`]: https://docs.rs/parsey/latest/parsey/trait.Ast.html
