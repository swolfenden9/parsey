use core::marker::PhantomData;

use crate::{ast::Ast, token_stream::TokenStream};

/// A trait representing a generic parser that consumes tokens and produces an AST.
///
/// This trait provides an abstraction for parsers that process tokens.
///
/// # Type Parameters
/// - `Token`: The type of tokens being parsed.
/// - `Error`: The type of errors that may occur during parsing.
pub trait Parser<Token, Error>: Iterator<Item = Token> + From<Vec<Token>> + Sized {
    /// The root type of the AST produced by this parser.
    type Root: Ast<Token, Error>;

    /// Parses an AST from a peekable token stream.
    ///
    /// # Returns
    /// Returns the parsed AST root node or an error if parsing fails.
    ///
    /// # Errors
    /// Returns an error of type `Error` if the token sequence does not match the expected structure.
    fn parse(self) -> Result<Self::Root, Error> {
        Self::Root::parse(&mut TokenStream {
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
    /// - `token_stream`: A peekable iterator implementing the `Parser` trait
    /// - `expected`: The token to validate against the expected token
    ///
    /// # Returns
    /// Returns () if the token matches the expected token or an error if not.
    ///
    /// # Errors
    /// Returns and error if the token does not match the expected token.
    fn expect(
        token_stream: &mut TokenStream<Self, Token, Error>,
        expected: Token,
    ) -> Result<(), Error>;
}
