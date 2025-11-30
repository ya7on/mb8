use crate::{
    error::CompileResult,
    parser::base::Parser,
    tokenizer::token::{Operator, TokenKind},
};

use super::Expr;

impl Parser {
    /// Parses an expression from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the expression cannot be parsed.
    pub fn parse_expr(&mut self) -> CompileResult<Expr> {
        let expr = self.parse_add_expr()?;
        Ok(expr)
    }

    /// Parses an addition expression from a list of tokens.
    ///
    /// # Errors
    /// Returns a `CompileError` if the expression cannot be parsed.
    pub fn parse_add_expr(&mut self) -> CompileResult<Expr> {
        let mut lhs = self.parse_mul_expr()?;

        loop {
            match self.peek() {
                TokenKind::Operator(Operator::Plus) => {
                    self.bump();
                    let rhs = self.parse_mul_expr()?;
                    lhs = Expr::BinaryOp {
                        op: Operator::Plus,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                TokenKind::Operator(Operator::Minus) => {
                    self.bump();
                    let rhs = self.parse_mul_expr()?;
                    lhs = Expr::BinaryOp {
                        op: Operator::Minus,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => return Ok(lhs),
            }
        }
    }

    /// Parses a multiplication expression.
    ///
    /// # Errors
    /// Returns an error if the expression is invalid.
    pub fn parse_mul_expr(&mut self) -> CompileResult<Expr> {
        let mut lhs = self.parse_unary_expr()?;

        loop {
            match self.peek() {
                TokenKind::Operator(Operator::Asterisk) => {
                    self.bump();
                    let rhs = self.parse_unary_expr()?;
                    lhs = Expr::BinaryOp {
                        op: Operator::Asterisk,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                TokenKind::Operator(Operator::Slash) => {
                    self.bump();
                    let rhs = self.parse_unary_expr()?;
                    lhs = Expr::BinaryOp {
                        op: Operator::Slash,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => return Ok(lhs),
            }
        }
    }

    /// Parses a unary expression.
    ///
    /// # Errors
    /// Returns a `CompileError` if the expression is invalid.
    pub fn parse_unary_expr(&mut self) -> CompileResult<Expr> {
        if let TokenKind::Operator(Operator::Minus) = self.peek() {
            self.bump();
            let inner = self.parse_unary_expr()?;
            Ok(Expr::Negation(Box::new(inner)))
        } else {
            self.parse_primary_expr()
        }
    }

    /// Parses a primary expression.
    ///
    /// # Errors
    /// Returns a `CompileError` if the expression is invalid.
    pub fn parse_primary_expr(&mut self) -> CompileResult<Expr> {
        match self.peek() {
            TokenKind::Number(num) => {
                self.bump();
                Ok(Expr::IntLiteral(num))
            }
            TokenKind::LeftParenthesis => {
                self.bump();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RightParenthesis)?;
                Ok(expr)
            }
            _ => unimplemented!(),
        }
    }
}
