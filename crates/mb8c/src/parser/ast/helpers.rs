use crate::{
    error::{CompileError, CompileResult},
    parser::base::Parser,
    tokenizer::token::{Keyword, TokenKind},
};

use super::Type;

impl Parser {
    /// Parses a type from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the type cannot be parsed.
    pub fn parse_type(&mut self) -> CompileResult<Type> {
        match self.bump() {
            TokenKind::Keyword(Keyword::Int) => Ok(Type::Int),
            _ => Err(CompileError::UnexpectedToken {
                line: self.line(),
                column: self.column(),
            }),
        }
    }

    /// Parses an identifier from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the identifier cannot be parsed.
    pub fn parse_ident(&mut self) -> CompileResult<String> {
        match self.bump() {
            TokenKind::Ident(ident) => Ok(ident),
            _ => Err(CompileError::UnexpectedToken {
                line: self.line(),
                column: self.column(),
            }),
        }
    }
}
