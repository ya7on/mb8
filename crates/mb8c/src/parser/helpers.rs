use crate::{
    error::{CompileError, CompileResult},
    tokenizer::token::TokenKind,
};

use super::base::Parser;

impl Parser {
    #[must_use]
    pub fn line(&self) -> usize {
        self.tokens[self.position].span.line
    }

    #[must_use]
    pub fn column(&self) -> usize {
        self.tokens[self.position].span.column
    }

    #[must_use]
    pub fn peek(&self) -> TokenKind {
        if self.position >= self.tokens.len() {
            TokenKind::Eof
        } else {
            self.tokens[self.position].kind.clone()
        }
    }

    pub fn bump(&mut self) -> TokenKind {
        let kind = self.peek();
        self.position += 1;
        kind
    }

    /// Expect the next token to be of the given kind.
    ///
    /// # Errors
    /// Returns a `CompileError::UnexpectedToken` if the next token is not of the expected kind.
    pub fn expect(&mut self, expected: &TokenKind) -> CompileResult<()> {
        let kind = self.peek();
        if kind == *expected {
            self.bump();
            return Ok(());
        }
        Err(CompileError::ParseError {
            line: self.line(),
            column: self.column(),
            message: format!("Expected type {expected:?}"),
        })
    }
}
