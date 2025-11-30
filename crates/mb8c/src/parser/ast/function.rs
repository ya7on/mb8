use crate::{error::CompileResult, parser::base::Parser, tokenizer::token::TokenKind};

use super::{Function, Type};

impl Parser {
    /// Parses a function from a list of tokens.
    ///
    /// # Errors
    /// Returns a [`CompileError`] if the function cannot be parsed.
    pub fn parse_function(&mut self, return_type: Type, name: String) -> CompileResult<Function> {
        let mut params = vec![];

        self.expect(&TokenKind::LeftParenthesis)?;
        while self.peek() != TokenKind::RightParenthesis {
            let ty = self.parse_type()?;
            let name = self.parse_ident()?;

            params.push((name, ty));

            if self.peek() == TokenKind::Comma {
                self.bump();
            }
        }
        self.expect(&TokenKind::RightParenthesis)?;

        if self.peek() == TokenKind::LeftBrace {
            let body = self.parse_stmt()?;

            return Ok(Function {
                name,
                return_type,
                params,
                body,
            });
        }

        unimplemented!()
    }
}
