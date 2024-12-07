//! # Parsey: A Minimalistic Parser-Generator Framework
//!
//! `parsey` is a lightweight, `no_std` framework for creating custom parsers and abstract syntax trees (ASTs).
//! It provides two key traits: [`Parser`][crate::Parser] and [`Ast`][crate::Ast], which together form the foundation
//! for building parsers and representing the structure of parsed data.
//!
//! ## Key Features
//! - **Generic Parsing Framework:** Abstracts the process of parsing tokens into structured data.
//! - **Customizable AST Nodes:** Easily define nodes of your AST by implementing the [`Ast`][crate::Ast] trait.
//! - **Integration with `no_std`:** Ideal for embedded or constrained environments.
//!
//! ## Getting Started
//!
//! ### Step 1: Implement the `Parser` Trait
//! Define a struct that will serve as your parser. This struct must implement the [`Parser`][crate::Parser] trait,
//! which processes tokens and produces an AST.
//!
//! ```rust
//! use parsey::{Parser, Ast};
//!
//! #[derive(Debug)]
//! pub enum Token {
//!     Zero,
//!     One,
//! }
//!
//! #[derive(Debug)]
//! pub struct MyError;
//!
//! pub struct MyParser {
//!     tokens: Vec<Token>,
//! }
//!
//! impl MyParser {
//!     pub fn new(mut tokens: Vec<Token>) -> Self {
//!         tokens.reverse();
//!         Self { tokens }
//!     }
//! }
//!
//! impl parsey::Parser<Token, MyError> for MyParser {
//!     type Root = Program;
//! }
//!
//! impl Iterator for MyParser {
//!     type Item = Token;
//!
//!     fn next(&mut self) -> Option<Self::Item> {
//!         self.tokens.pop()
//!     }
//! }
//! ```
//!
//! ### Step 2: Define the AST Nodes
//! Create the structure for your AST by implementing the [`Ast`][crate::Ast] trait for each node.
//! The root node must match the type defined in `Parser::Root`.
//!
//! ```rust
//! #[derive(Debug)]
//! pub struct Program(Vec<Statement>);
//!
//! #[derive(Debug)]
//! pub enum Statement {
//!     ZeroZero,
//!     ZeroOne,
//!     OneZero,
//!     OneOne,
//! }
//!
//! impl parsey::Ast<Token, MyError> for Program {
//!     fn parse<P>(parser: &mut std::iter::Peekable<P>) -> Result<Self, MyError>
//!     where
//!         P: parsey::Parser<Token, MyError>,
//!     {
//!         let mut statements = vec![];
//!         while parser.peek().is_some() {
//!             statements.push(Statement::parse(parser)?);
//!         }
//!         Ok(Self(statements))
//!     }
//! }
//!
//! impl parsey::Ast<Token, MyError> for Statement {
//!     fn parse<P>(parser: &mut std::iter::Peekable<P>) -> Result<Self, MyError>
//!     where
//!         P: parsey::Parser<Token, MyError>,
//!     {
//!         match parser.next() {
//!             Some(Token::Zero) => match parser.next() {
//!                 Some(Token::Zero) => Ok(Statement::ZeroZero),
//!                 Some(Token::One) => Ok(Statement::ZeroOne),
//!                 _ => Err(MyError),
//!             },
//!             Some(Token::One) => match parser.next() {
//!                 Some(Token::Zero) => Ok(Statement::OneZero),
//!                 Some(Token::One) => Ok(Statement::OneOne),
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
//! ```rust
//! fn main() {
//!     let tokens = vec![Token::One, Token::Zero, Token::One, Token::Zero];
//!     let parser = MyParser::new(tokens);
//!     let ast = parsey::Parser::parse(parser);
//!     match ast {
//!         Ok(ast) => println!("Parsed AST: {:?}", ast),
//!         Err(e) => eprintln!("Parsing error: {:?}", e),
//!     }
//! }
//! ```

#![no_std]

use core::iter::Peekable;

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
        Ast::parse(&mut self.peekable())
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
    fn parse<P>(parser: &mut Peekable<P>) -> Result<Self, Error>
    where
        P: Parser<Token, Error>;
}
