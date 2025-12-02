use crate::{
    error::CompileResult,
    parser::base::Parser,
    tokenizer::token::{Keyword, Operator, TokenKind},
};

use super::Stmt;

impl Parser {
    /// Parses a statement
    ///
    /// # Errors
    /// Returns a `CompileError` if the statement cannot be parsed.
    pub fn parse_stmt(&mut self) -> CompileResult<Stmt> {
        match self.peek() {
            TokenKind::LeftBrace => self.parse_block_stmt(),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_stmt(),
            TokenKind::Keyword(Keyword::If) => self.parse_if_stmt(),
            TokenKind::Keyword(_) => self.parse_declaration_stmt(),
            TokenKind::Ident(_) => {
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::Semicolon)?;
                Ok(Stmt::Expression(expr))
            }
            _ => unimplemented!(),
        }
    }

    /// Parses a block statement from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the block statement cannot be parsed.
    pub fn parse_block_stmt(&mut self) -> CompileResult<Stmt> {
        self.expect(&TokenKind::LeftBrace)?;
        let mut stmts = Vec::new();
        while !matches!(self.peek(), TokenKind::RightBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RightBrace)?;

        Ok(Stmt::Block(stmts))
    }

    /// Parses a return statement from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the return statement cannot be parsed.
    pub fn parse_return_stmt(&mut self) -> CompileResult<Stmt> {
        self.expect(&TokenKind::Keyword(Keyword::Return))?;
        if matches!(self.peek(), TokenKind::Semicolon) {
            self.bump();
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expr()?;
            self.expect(&TokenKind::Semicolon)?;
            Ok(Stmt::Return(Some(expr)))
        }
    }

    /// Parses an if statement from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the if statement cannot be parsed.
    pub fn parse_if_stmt(&mut self) -> CompileResult<Stmt> {
        self.expect(&TokenKind::Keyword(Keyword::If))?;
        self.expect(&TokenKind::LeftParenthesis)?;
        let conition = self.parse_expr()?;
        self.expect(&TokenKind::RightParenthesis)?;

        let then_branch = self.parse_stmt()?;

        let else_branch = if matches!(self.peek(), TokenKind::Keyword(Keyword::Else)) {
            self.bump();
            Some(self.parse_stmt()?)
        } else {
            None
        };

        Ok(Stmt::If {
            condition: conition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    /// Parses a declaration statement from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the declaration statement cannot be parsed.
    pub fn parse_declaration_stmt(&mut self) -> CompileResult<Stmt> {
        let ty = self.parse_type()?;
        let name = self.parse_ident()?;

        let init = if matches!(self.peek(), TokenKind::Operator(Operator::Eq)) {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.expect(&TokenKind::Semicolon)?;

        Ok(Stmt::Declaration { name, ty, init })
    }
}
