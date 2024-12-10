//! # Parsey
//!
//! `parsey` is a lightweight framework for creating custom parsers and abstract syntax trees (ASTs).
//! It provides two key traits: [`Parser`] and [`Ast`], which together form the foundation
//! for building parsers and representing the structure of parsed data.
//!
//! ## Key Features
//! - **Generic Parsing Framework:** Abstracts the process of parsing tokens into structured data.
//! - **Customizable AST Nodes:** Easily define nodes of your AST by implementing the [`Ast`] trait.
//!
//! ## Getting Started
//!
//! Let's implement a simple parser that parses a stream of zero and one tokens into groups of two
//! bits!
//!
//! ### Step 1: Implement the `Parser` Trait
//! Define a struct that will serve as your parser. This struct must implement the [`Parser`] trait,
//! which iterates over tokens and produces an AST.
//!
//! ```rust,ignore
//! use parsey::{parse, require_next_n, Ast, Parser, TokenStream};
//!
//! #[derive(Debug, PartialEq)]
//! pub enum MyToken {
//!     Zero,
//!     One,
//! }
//!
//! #[derive(Debug, PartialEq)]
//! pub struct MyError;
//!
//! pub struct MyParser {
//!     tokens: Vec<MyToken>,
//! }
//!
//! impl Parser<MyToken, MyError> for MyParser {
//!     type Root = Root;
//!
//!     fn expect(
//!         token_stream: &mut TokenStream<Self, MyToken, MyError>,
//!         token: MyToken,
//!     ) -> Result<(), MyError> {
//!         if token_stream.peek() == Some(&token) {
//!             token_stream.next();
//!             Ok(())
//!         } else {
//!             Err(MyError)
//!         }
//!     }
//! }
//!
//! impl Iterator for MyParser {
//!     type Item = MyToken;
//!
//!     fn next(&mut self) -> Option<Self::Item> {
//!         self.tokens.pop()
//!     }
//! }
//!
//! impl From<Vec<MyToken>> for MyParser {
//!     fn from(mut value: Vec<MyToken>) -> Self {
//!         value.reverse();
//!         Self { tokens: value }
//!     }
//! }
//! ```
//!
//! ### Step 2: Define the AST Nodes
//! Create the structure for your AST by implementing the [`Ast`] trait for each node.
//! The root node must match the type defined in `Parser::Root`.
//!
//! ```rust,ignore
//! #[derive(Debug, PartialEq)]
//! pub struct Root(Vec<TwoBit>);
//!
//! #[derive(Debug, PartialEq)]
//! pub enum TwoBit {
//!     ZeroZero,
//!     ZeroOne,
//!     OneZero,
//!     OneOne,
//! }
//!
//! impl Ast<MyToken, MyError> for Root {
//!     fn parse<P>(token_stream: &mut TokenStream<P, MyToken, MyError>) -> Result<Self, MyError>
//!     where
//!         P: Parser<MyToken, MyError>,
//!     {
//!         let mut two_bits = vec![];
//!         while !token_stream.is_empty() {
//!             two_bits.push(TwoBit::parse(token_stream)?);
//!         }
//!         Ok(Self(two_bits))
//!     }
//! }
//!
//! impl Ast<MyToken, MyError> for TwoBit {
//!     fn parse<P>(token_stream: &mut TokenStream<P, MyToken, MyError>) -> Result<Self, MyError>
//!     where
//!         P: parsey::Parser<MyToken, MyError>,
//!     {
//!         use MyToken::*;
//!         use TwoBit::*;
//!
//!         match require_next_n!(token_stream, 2, MyError) {
//!             [Zero, Zero] => Ok(ZeroZero),
//!             [Zero, One] => Ok(ZeroOne),
//!             [One, Zero] => Ok(OneZero),
//!             [One, One] => Ok(OneOne),
//!         }
//!     }
//! }
//! ```
//!
//! ### Step 3: Parse Tokens
//!
//! Use your parser to parse a sequence of tokens into an AST.
//!
//! ```rust,ignore
//! use MyToken::{One, Zero};
//! use TwoBit::{OneOne, OneZero, ZeroOne, ZeroZero};
//!
//! let tokens = vec![Zero, Zero, Zero, One, One, Zero, One, One];
//! let ast = parse::<MyParser, MyToken, MyError>(tokens);
//! assert_eq!(ast, Ok(Root(vec![ZeroZero, ZeroOne, OneZero, OneOne])));
//! ```

pub use ast::Ast;
pub use parser::Parser;
pub use token_stream::TokenStream;

mod ast;
mod parser;
mod token_stream;

/// Parse a vec of tokens into the provided root AST node.
///
/// # Type Parameters
/// - `P`: The parser used to parse the tokens.
/// - `Token`: The type of token being parsed.
/// - `Error`: The error type that can be returned from parsing.
pub fn parse<P, Token, Error>(tokens: Vec<Token>) -> Result<P::Root, Error>
where
    P: Parser<Token, Error>,
{
    P::from(tokens).parse()
}

/// Get the next `n` tokens from `token_stream`.
#[macro_export]
macro_rules! next_n {
    ($token_stream:expr, $n:expr) => {{
        let tokens: [Option<_>; $n] = $token_stream.next_n($n).try_into().unwrap();
        tokens
    }};
}

/// Peek at the next `n` tokens from `token_stream` without consuming them.
#[macro_export]
macro_rules! peek_n {
    ($token_stream:expr, $n:expr) => {{
        let tokens: [Option<&_>; $n] = $token_stream.peek_n($n).try_into().unwrap();
        tokens
    }};
}

/// Get the next `n` tokens from `token_stream` or return the provided
/// error if the token stream ends before the required amount of tokens
/// are consumed.
#[macro_export]
macro_rules! require_next_n {
    ($token_stream:expr, $n:expr, $error:expr) => {
        match $token_stream.require_next_n($n) {
            Some(tokens) => {
                // Unwrapping here is safe
                let tokens: [_; $n] = tokens.try_into().unwrap();
                tokens
            }
            None => return Err($error),
        };
    };
}

/// Peek at the next `n` tokens from `token_stream` or return the provided
/// error if the token stream ends before the required amount of tokens
/// are peeked.
#[macro_export]
macro_rules! require_peek_n {
    ($token_stream:expr, $n:expr, $error:expr) => {
        match $token_stream.require_peek_n($n) {
            Some(tokens) => {
                // Unwrapping here is safe
                let tokens: [&_; $n] = tokens.try_into().unwrap();
                tokens
            }
            None => return Err($error),
        };
    };
}
