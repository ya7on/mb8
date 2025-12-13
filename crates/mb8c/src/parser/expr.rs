use chumsky::{
    error::Simple,
    extra::Err,
    prelude::{just, recursive},
    select, IterParser, Parser,
};

use crate::{
    ast::{ASTBinaryOp, ASTExpr},
    tokens::TokenKind,
};

#[must_use]
pub fn expr_parser<'src>(
) -> impl Parser<'src, &'src [TokenKind], ASTExpr, Err<Simple<'src, TokenKind>>> + Clone {
    recursive(|expr| {
        let args = expr
            .clone()
            .separated_by(just(TokenKind::Comma))
            .collect::<Vec<_>>()
            .delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            );

        let call_expr = select! {
            TokenKind::Ident(name) => name,
        }
        .then(args.clone())
        .map(|(name, args)| ASTExpr::Call { name, args });

        let primary = call_expr
            .or(select! {
                TokenKind::Number(n) => ASTExpr::IntLiteral(n),
                TokenKind::Ident(name) => ASTExpr::Var(name),
            })
            .or(expr.clone().delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            ));

        let product = primary.clone().foldl(
            (just(TokenKind::OperatorAsterisk)
                .to(ASTBinaryOp::Mul)
                .or(just(TokenKind::OperatorSlash).to(ASTBinaryOp::Div)))
            .then(primary.clone())
            .repeated(),
            |lhs, (op, rhs)| ASTExpr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let sum = product.clone().foldl(
            (just(TokenKind::OperatorPlus)
                .to(ASTBinaryOp::Add)
                .or(just(TokenKind::OperatorMinus).to(ASTBinaryOp::Sub)))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| ASTExpr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let equality = sum.clone().foldl(
            just(TokenKind::OperatorEqEq)
                .to(ASTBinaryOp::Eq)
                .then(sum)
                .repeated(),
            |lhs, (op, rhs)| ASTExpr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        select! {
            TokenKind::Ident(name) => name,
        }
        .then_ignore(just(TokenKind::OperatorEq))
        .then(expr.clone())
        .map(|(name, value)| ASTExpr::Assign {
            name,
            value: Box::new(value),
        })
        .or(equality)
    })
}
