use core::{
    iter::Peekable,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::parser::Parser;

/// A wrapper around a peekable parser that provides lookahead functionality.
///
/// `TokenStream` enhances a parser by allowing it to look at the next token
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
/// use parsey::{Parser, TokenStream};
///
/// // Assuming MyParser and MyToken are defined...
/// let tokens = vec![MyToken::One, MyToken::Zero];
/// let parser = MyParser::new(tokens);
/// let mut peekable = TokenStream::new(parser);
///
/// // Peek at next token without consuming it
/// assert_eq!(peekable.peek(), Some(&MyToken::One));
/// ```
pub struct TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    pub(crate) inner: Peekable<P>,
    pub(crate) token_phantom: PhantomData<Token>,
    pub(crate) error_phantom: PhantomData<Error>,
}

impl<P, Token, Error> TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    /// Validates whether a given token matches the expected token.
    ///
    /// This method is used to verify that the next token in the parsing sequence
    /// matches what is expected according to the grammar rules. if it matches,
    /// the token is consumed.
    ///
    /// # Parameters
    /// - `expected`: The token to validate against the expected token
    ///
    /// # Returns
    /// Returns () if the token matches the expected token or an error if not.
    ///
    /// # Errors
    /// Returns and error if the token does not match the expected token.
    pub fn expect(&mut self, expected: Token) -> Result<(), Error> {
        P::expect(self, expected)
    }
}

impl<P, Token, Error> Deref for TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    type Target = Peekable<P>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P, Token, Error> DerefMut for TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<P, Token, Error> Iterator for TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
