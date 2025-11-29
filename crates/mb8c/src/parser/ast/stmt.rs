use crate::{
    error::CompileResult,
    parser::base::Parser,
    tokenizer::token::{Keyword, TokenKind},
};

use super::Stmt;

impl Parser {
    pub fn parse_stmt(&mut self) -> CompileResult<Stmt> {
        match self.peek() {
            TokenKind::LeftBrace => self.parse_block_stmt(),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_stmt(),
            _ => unimplemented!(),
        }
    }

    pub fn parse_block_stmt(&mut self) -> CompileResult<Stmt> {
        self.expect(TokenKind::LeftBrace)?;
        let mut stmts = Vec::new();
        while !matches!(self.peek(), TokenKind::RightBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RightBrace)?;

        Ok(Stmt::Block(stmts))
    }

    pub fn parse_return_stmt(&mut self) -> CompileResult<Stmt> {
        self.expect(TokenKind::Keyword(Keyword::Return))?;
        if matches!(self.peek(), TokenKind::Semicolon) {
            self.bump();
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Stmt::Return(Some(expr)))
        }
    }
}
