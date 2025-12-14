use chumsky::{
    error::Simple,
    extra::Err,
    prelude::{just, recursive},
    select,
    span::SimpleSpan,
    IterParser, Parser,
};

use crate::{
    ast::{ASTBinaryOp, ASTExpr, Span},
    tokens::TokenKind,
};

#[must_use]
#[allow(clippy::too_many_lines)]
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
        .map_with(|(name, args), extra| {
            let span: SimpleSpan = extra.span();
            ASTExpr::Call {
                name,
                args,
                span: Span {
                    start: span.start,
                    end: span.end,
                },
            }
        });

        let primary = call_expr
            .or(
                select! { TokenKind::Number(number) => number }.map_with(|number, extra| {
                    let span: SimpleSpan = extra.span();
                    ASTExpr::IntLiteral {
                        value: number,
                        span: Span {
                            start: span.start,
                            end: span.end,
                        },
                    }
                }),
            )
            .or(
                select! { TokenKind::Ident(name) => name }.map_with(|name, extra| {
                    let span: SimpleSpan = extra.span();
                    ASTExpr::Var {
                        name,
                        span: Span {
                            start: span.start,
                            end: span.end,
                        },
                    }
                }),
            )
            .or(expr.clone().delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            ));

        let product = primary.clone().foldl_with(
            (just(TokenKind::OperatorAsterisk)
                .to(ASTBinaryOp::Mul)
                .or(just(TokenKind::OperatorSlash).to(ASTBinaryOp::Div)))
            .then(primary.clone())
            .repeated(),
            |lhs, (op, rhs), extra| {
                let span: SimpleSpan = extra.span();
                ASTExpr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            },
        );

        let sum = product.clone().foldl_with(
            (just(TokenKind::OperatorPlus)
                .to(ASTBinaryOp::Add)
                .or(just(TokenKind::OperatorMinus).to(ASTBinaryOp::Sub)))
            .then(product)
            .repeated(),
            |lhs, (op, rhs), extra| {
                let span: SimpleSpan = extra.span();
                ASTExpr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            },
        );

        let equality = sum.clone().foldl_with(
            just(TokenKind::OperatorEqEq)
                .to(ASTBinaryOp::Eq)
                .then(sum)
                .repeated(),
            |lhs, (op, rhs), extra| {
                let span: SimpleSpan = extra.span();
                ASTExpr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            },
        );

        select! {
            TokenKind::Ident(name) => name,
        }
        .then_ignore(just(TokenKind::OperatorEq))
        .then(expr.clone())
        .map_with(|(name, value), extra| {
            let span: SimpleSpan = extra.span();
            ASTExpr::Assign {
                name,
                value: Box::new(value),
                span: Span {
                    start: span.start,
                    end: span.end,
                },
            }
        })
        .or(equality)
    })
}
