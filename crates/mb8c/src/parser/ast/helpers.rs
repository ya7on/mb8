use crate::{
    error::{CompileError, CompileResult},
    parser::base::Parser,
    tokenizer::token::{Keyword, TokenKind},
};

use super::Type;

impl Parser {
    pub fn parse_type(&mut self) -> CompileResult<Type> {
        match self.bump() {
            TokenKind::Keyword(Keyword::Int) => Ok(Type::Int),
            _ => Err(CompileError::UnexpectedToken {
                line: self.line(),
                column: self.column(),
            }),
        }
    }

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
