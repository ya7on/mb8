use crate::{error::CompileResult, parser::base::Parser, tokenizer::token::TokenKind};

use super::Expr;

impl Parser {
    /// Parses an expression from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the expression cannot be parsed.
    pub fn parse_expr(&mut self) -> CompileResult<Expr> {
        match self.peek() {
            TokenKind::Number(num) => {
                self.bump();
                Ok(Expr::IntLiteral(num))
            }
            _ => unimplemented!(),
        }
    }
}
