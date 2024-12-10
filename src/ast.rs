use crate::{parser::Parser, token_stream::TokenStream};

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
    /// - `token_stream`: A peekable iterator implementing the `Parser` trait.
    ///
    /// # Returns
    /// Returns the parsed AST node or an error if parsing fails.
    ///
    /// # Errors
    /// Returns an error of type `Error` if the token sequence does not match the expected structure.
    fn parse<P>(token_stream: &mut TokenStream<P, Token, Error>) -> Result<Self, Error>
    where
        P: Parser<Token, Error>;
}
