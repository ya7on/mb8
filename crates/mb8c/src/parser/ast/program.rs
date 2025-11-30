use crate::{error::CompileResult, parser::base::Parser, tokenizer::token::TokenKind};

use super::Program;

impl Parser {
    /// Parses a program from a list of tokens.
    ///
    /// # Errors
    ///
    /// Returns a [`CompileError`] if the program cannot be parsed.
    pub fn parse_program(&mut self) -> CompileResult<Program> {
        let mut functions = Vec::new();

        while self.peek() != TokenKind::Eof {
            let ty = self.parse_type()?;
            let name = self.parse_ident()?;

            let function = self.parse_function(ty, name)?;
            functions.push(function);
        }

        Ok(Program { functions })
    }
}
