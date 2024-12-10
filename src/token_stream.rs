use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use std::collections::VecDeque;

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
    parser: P,
    peeked: VecDeque<Option<Token>>,
    error_phantom: PhantomData<Error>,
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

    pub fn peek(&mut self) -> Option<&Token> {
        self.peek_n(1).map(|t| t[0])
    }

    /// Peek at the next `n` tokens without consuming them.
    pub fn peek_n(&mut self, n: usize) -> Option<Vec<&Token>> {
        // Ensure there are at least `n` tokens in the `peeked` queue.
        while self.peeked.len() < n {
            match self.parser.next() {
                Some(token) => self.peeked.push_back(Some(token)),
                None => return None, // Not enough tokens
            }
        }

        // SAFETY: `peeked` has at least `n` elements.
        Some(
            self.peeked
                .iter()
                .take(n)
                .map(|opt| opt.as_ref().unwrap())
                .collect::<Vec<&Token>>(),
        )
    }

    /// Consume and return the next `n` tokens.
    pub fn next_n(&mut self, n: usize) -> Option<Vec<Token>> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(self.next()?);
        }
        Some(result)
    }

    /// Returns true if there are no more tokens in the token stream.
    pub fn is_empty(&mut self) -> bool {
        self.peek_n(1).is_none()
    }
}

impl<P, Token, Error> From<P> for TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    fn from(value: P) -> Self {
        Self {
            parser: value,
            peeked: VecDeque::new(),
            error_phantom: PhantomData,
        }
    }
}

impl<P, Token, Error> Deref for TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}

impl<P, Token, Error> DerefMut for TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

impl<P, Token, Error> Iterator for TokenStream<P, Token, Error>
where
    P: Parser<Token, Error>,
{
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.pop_front() {
            Some(v) => v,
            None => self.parser.next(),
        }
    }
}
