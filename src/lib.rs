//! # Parsey
//!
//! `parsey` is a lightweight, `no_std` framework for creating custom parsers and abstract syntax trees (ASTs).
//! It provides two key traits: [`Parser`] and [`Ast`], which together form the foundation
//! for building parsers and representing the structure of parsed data.
//!
//! ## Key Features
//! - **Generic Parsing Framework:** Abstracts the process of parsing tokens into structured data.
//! - **Customizable AST Nodes:** Easily define nodes of your AST by implementing the [`Ast`] trait.
//! - **Integration with `no_std`:** Ideal for embedded or constrained environments.
//!
//! ## Getting Started
//!
//! ### Step 1: Implement the `Parser` Trait
//! Define a struct that will serve as your parser. This struct must implement the [`Parser`] trait,
//! which processes tokens and produces an AST.
//!
//! ```rust,ignore
//! use parsey::{Ast, Parser, PeekableParser};
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
//! impl MyParser {
//!     pub fn new(mut tokens: Vec<MyToken>) -> Self {
//!         tokens.reverse();
//!         Self { tokens }
//!     }
//! }
//!
//! impl Parser<MyToken, MyError> for MyParser {
//!     type Root = Root;
//!
//!     fn expect(
//!         peekable_parser: &mut PeekableParser<Self, MyToken, MyError>,
//!         token: MyToken,
//!     ) -> Result<(), MyError> {
//!         if peekable_parser.peek() == Some(&token) {
//!             peekable_parser.next();
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
//!     fn parse<P>(parser: &mut PeekableParser<P, MyToken, MyError>) -> Result<Self, MyError>
//!     where
//!         P: Parser<MyToken, MyError>,
//!     {
//!         let mut two_bits = vec![];
//!         while parser.peek().is_some() {
//!             two_bits.push(TwoBit::parse(parser)?);
//!         }
//!         Ok(Self(two_bits))
//!     }
//! }
//!
//! impl parsey::Ast<MyToken, MyError> for TwoBit {
//!     fn parse<P>(parser: &mut PeekableParser<P, MyToken, MyError>) -> Result<Self, MyError>
//!     where
//!         P: parsey::Parser<MyToken, MyError>,
//!     {
//!         match parser.next() {
//!             Some(MyToken::Zero) => match parser.next() {
//!                 Some(MyToken::Zero) => Ok(TwoBit::ZeroZero),
//!                 Some(MyToken::One) => Ok(TwoBit::ZeroOne),
//!                 _ => Err(MyError),
//!             },
//!             Some(MyToken::One) => match parser.next() {
//!                 Some(MyToken::Zero) => Ok(TwoBit::OneZero),
//!                 Some(MyToken::One) => Ok(TwoBit::OneOne),
//!                 _ => Err(MyError),
//!             },
//!             _ => Err(MyError),
//!         }
//!     }
//! }
//! ```
//!
//! ### Step 3: Parse Tokens
//! Use your parser to parse a sequence of tokens into an AST.
//!
//! ```rust,ignore
//! fn main() {
//!     use MyToken::{One, Zero};
//!     use TwoBit::{OneOne, OneZero, ZeroOne, ZeroZero};
//!
//!     let tokens = vec![Zero, Zero, Zero, One, One, Zero, One, One];
//!     let parser = MyParser::new(tokens);
//!     let ast = parser.parse().unwrap();
//!     assert_eq!(ast, Root(vec![ZeroZero, ZeroOne, OneZero, OneOne]));
//! }
//! ```

#![no_std]

use core::{iter::Peekable, marker::PhantomData};

/// A trait representing a generic parser that consumes tokens and produces an AST.
///
/// This trait provides an abstraction for parsers that process tokens.
///
/// # Type Parameters
/// - `Token`: The type of tokens being parsed.
/// - `Error`: The type of errors that may occur during parsing.
pub trait Parser<Token, Error>: Iterator<Item = Token> + Sized {
    /// The root type of the AST produced by this parser.
    type Root: Ast<Token, Error>;

    /// Parses an AST from a peekable token stream.
    ///
    /// # Parameters
    /// - `parser`: A peekable iterator implementing the `Parser` trait.
    ///
    /// # Returns
    /// Returns the parsed AST root node or an error if parsing fails.
    ///
    /// # Errors
    /// Returns an error of type `Error` if the token sequence does not match the expected structure.
    fn parse(self) -> Result<Self::Root, Error> {
        Ast::parse(&mut PeekableParser {
            inner: self.peekable(),
            token_phantom: PhantomData,
            error_phantom: PhantomData,
        })
    }

    /// Validates whether a given token matches the expected token.
    ///
    /// This method is used to verify that the next token in the parsing sequence
    /// matches what is expected according to the grammar rules. if it matches,
    /// the token is consumed.
    ///
    /// # Parameters
    /// - `token`: The token to validate against the expected token
    ///
    /// # Returns
    /// - `Ok(())` if the token matches the expected token
    /// - `Err(Error)` if the token does not match the expected token
    fn expect(
        peekable_parser: &mut PeekableParser<Self, Token, Error>,
        expected: Token,
    ) -> Result<(), Error>;
}

/// A wrapper around a peekable parser that provides lookahead functionality.
///
/// `PeekableParser` enhances a parser by allowing it to look at the next token
/// without consuming it. This is essential for making parsing decisions based on
/// upcoming tokens.
///
/// # Type Parameters
/// - `P`: The underlying parser type that implements [`Parser`]
/// - `Token`: The type of tokens being parsed
/// - `Error`: The type of errors that may occur during parsing
///
/// # Examples
/// ```rust,ignore
/// use parsey::{Parser, PeekableParser};
///
/// // Assuming MyParser and MyToken are defined...
/// let tokens = vec![MyToken::One, MyToken::Zero];
/// let parser = MyParser::new(tokens);
/// let mut peekable = PeekableParser::new(parser);
///
/// // Peek at next token without consuming it
/// assert_eq!(peekable.peek(), Some(&MyToken::One));
/// ```
pub struct PeekableParser<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    inner: Peekable<P>,
    token_phantom: PhantomData<Token>,
    error_phantom: PhantomData<Error>,
}

impl<P, Token, Error> PeekableParser<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    /// Returns a reference to the next token without consuming it.
    ///
    /// This method allows the parser to examine the next token in the sequence
    /// without removing it from the stream. This is useful for making parsing
    /// decisions based on lookahead.
    ///
    /// # Returns
    /// - `Some(&Token)` if there is a next token
    /// - `None` if the end of the token stream has been reached
    ///
    /// # Examples
    /// ```rust,ignore
    /// use parsey::{Parser, PeekableParser};
    ///
    /// let tokens = vec![MyToken::One, MyToken::Zero];
    /// let parser = MyParser::new(tokens);
    /// let mut peekable = PeekableParser::new(parser);
    ///
    /// // Peek at next token
    /// assert_eq!(peekable.peek(), Some(&MyToken::One));
    ///
    /// // Token is still there after peeking
    /// assert_eq!(peekable.next(), Some(MyToken::One));
    /// ```
    pub fn peek(&mut self) -> Option<&Token> {
        self.inner.peek()
    }

    /// Validates whether a given token matches the expected token.
    ///
    /// This method is used to verify that the next token in the parsing sequence
    /// matches what is expected according to the grammar rules. if it matches,
    /// the token is consumed.
    ///
    /// # Parameters
    /// - `token`: The token to validate against the expected token
    ///
    /// # Returns
    /// - `Ok(())` if the token matches the expected token
    /// - `Err(Error)` if the token does not match the expected token
    pub fn expect(&mut self, expected: Token) -> Result<(), Error> {
        P::expect(self, expected)
    }
}

impl<P, Token, Error> Iterator for PeekableParser<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// A trait representing a component of an abstract syntax tree (AST).
///
/// This trait defines the interface for parsing a specific node or component of the AST
/// from a sequence of tokens.
///
/// # Type Parameters
/// - `Token`: The type of tokens being parsed.
/// - `Error`: The type of errors that may occur during parsing.
pub trait Ast<Token, Error>: Sized {
    /// Parses an AST node from a peekable token stream.
    ///
    /// # Parameters
    /// - `parser`: A peekable iterator implementing the `Parser` trait.
    ///
    /// # Returns
    /// Returns the parsed AST node or an error if parsing fails.
    ///
    /// # Errors
    /// Returns an error of type `Error` if the token sequence does not match the expected structure.
    fn parse<P>(parser: &mut PeekableParser<P, Token, Error>) -> Result<Self, Error>
    where
        P: Parser<Token, Error>;
}
