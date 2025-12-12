use chumsky::{
    prelude::{just, recursive},
    select, IterParser, Parser,
};

use crate::{
    ast::{BinaryOp, Expr},
    tokens::TokenKind,
};

#[must_use]
pub fn expr_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Expr> + Clone {
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
        .map(|(name, args)| Expr::Call { name, args });

        let primary = call_expr
            .or(select! {
                TokenKind::Number(n) => Expr::IntLiteral(n),
                TokenKind::Ident(name) => Expr::Var(name),
            })
            .or(expr.clone().delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            ));

        let product = primary.clone().foldl(
            (just(TokenKind::OperatorAsterisk)
                .to(BinaryOp::Mul)
                .or(just(TokenKind::OperatorSlash).to(BinaryOp::Div)))
            .then(primary.clone())
            .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let sum = product.clone().foldl(
            (just(TokenKind::OperatorPlus)
                .to(BinaryOp::Add)
                .or(just(TokenKind::OperatorMinus).to(BinaryOp::Sub)))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let equality = sum.clone().foldl(
            just(TokenKind::OperatorEqEq)
                .to(BinaryOp::Eq)
                .then(sum)
                .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
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
        .map(|(name, value)| Expr::Assign {
            name,
            value: Box::new(value),
        })
        .or(equality)
    })
}
