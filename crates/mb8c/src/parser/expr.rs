use chumsky::{
    prelude::{just, recursive},
    select, Parser,
};

use crate::{
    parser::ast::Expr,
    tokenizer::token::{Operator, TokenKind},
};

pub fn expr_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Expr> + Clone {
    recursive(|expr| {
        let atom = select! {
            TokenKind::Number(number) => Expr::IntLiteral(number),
            TokenKind::Ident(name) => Expr::Var(name),
        }
        .or(expr.clone().delimited_by(
            just(TokenKind::LeftParenthesis),
            just(TokenKind::RightParenthesis),
        ));

        let product = atom.clone().foldl(
            (just(TokenKind::Operator(Operator::Asterisk))
                .to(Operator::Asterisk)
                .or(just(TokenKind::Operator(Operator::Slash)).to(Operator::Slash)))
            .then(atom)
            .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let sum = product.clone().foldl(
            (just(TokenKind::Operator(Operator::Plus))
                .to(Operator::Plus)
                .or(just(TokenKind::Operator(Operator::Minus)).to(Operator::Minus)))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let equality = sum.clone().foldl(
            just(TokenKind::Operator(Operator::EqEq))
                .to(Operator::EqEq)
                .then(sum)
                .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let assignment = select! {
            TokenKind::Ident(name) => name,
        }
        .then_ignore(just(TokenKind::Operator(Operator::Eq)))
        .then(expr.clone())
        .map(|(name, value)| Expr::Assign {
            name,
            value: Box::new(value),
        })
        .or(equality);

        assignment
    })
}
